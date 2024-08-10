//! The `query` module defines the core abstractions and components related to handling queries in the CQRS pattern.
//!
//! Queries are used to retrieve data without modifying the state of the system. This module provides the `Query` trait,
//! which represents a query, and the `QueryHandler` trait, which defines the behavior for handling queries.
//!
//! The `QueryBus` is responsible for dispatching queries to their respective handlers. It utilizes the
//! `QueryHandlerRegistry` from the [registry](crate::registry) module to manage and retrieve the appropriate handlers.
//!
//! - [Query]: Represents a query in the system.
//! - [QueryHandler]: Trait for handling queries.
//! - [QueryBus]: Dispatches queries to the appropriate handlers.
//!
//! # See Also
//!
//! - [QueryHandlerRegistry]: Manages query handlers.

use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

use crate::async_trait;
use crate::registry::QueryHandlerRegistry;

/// The `Query` trait represents a query that retrieves data from the system.
///
/// Queries are typically used to retrieve information without modifying the state of the system.
///
/// # Example
///
/// ```
/// use discern::query::Query;
///
/// #[derive(Debug)]
/// struct User {
///     id: u64,
///     username: String,
///     email: String,
/// }
///
/// #[derive(Debug)]
/// enum GetUserError {
///     UserNotFound,
///     DatabaseError,
/// }
///
/// #[derive(Debug)]
/// struct GetUserQuery {
///    user_id: u64,
/// }
///
/// impl Query for GetUserQuery {
///   // The data retrieved by the query, in this case, a `User`.
///   type Output = User;
///   // The error type that is returned if the query fails.
///   type Error = GetUserError;
/// }
/// ```
pub trait Query: Send + Sync + Any + Debug {
    /// The output type that represents the data retrieved by the query.
    ///
    /// This type must implement the `Send` and `Sync` traits.
    type Output: Send + Sync;

    /// The error type that is returned if the query fails.
    ///
    /// This type must implement the `Debug`, `Send`, and `Sync` traits.
    type Error: Debug + Send + Sync;
}

/// The `QueryHandler` trait represents a handler that processes a query.
///
/// # Example
///
/// ```
/// # use discern::query::Query;
/// # use discern::async_trait;
/// #
/// # #[derive(Debug)]
/// # struct User {
/// #     id: u64,
/// #     username: String,
/// #     email: String,
/// # }
/// #
/// # #[derive(Debug)]
/// # enum GetUserError {
/// #     UserNotFound,
/// #     DatabaseError,
/// # }
/// #
/// # #[derive(Debug)]
/// # struct GetUserQuery {
/// #    user_id: u64,
/// # }
/// #
/// # impl Query for GetUserQuery {
/// #   type Output = User;
/// #   type Error = GetUserError;
/// # }
/// #
/// use discern::query::QueryHandler;
///
/// struct GetUserQueryHandler;
///
/// #[async_trait]
/// impl QueryHandler<GetUserQuery> for GetUserQueryHandler {
///    async fn handle(&self, query: GetUserQuery) -> Result<User, GetUserError> {
///       // retrieve the user from the persistence layer...
/// #     Ok(User {
/// #         id: query.user_id,
/// #         username: "Alice".to_string(),
/// #         email: "alice@example.com".to_string(),
/// #     })
///   }
/// }
/// ```
#[async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync {
    /// Handles the processing of a query.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to be processed.
    ///
    /// # Returns
    ///
    /// The result of the query handler, which includes the output data or an error.
    async fn handle(&self, query: Q) -> Result<Q::Output, Q::Error>;
}

/// The `QueryBus` is responsible for dispatching queries to their respective handlers.
///
/// Queries are dispatched through the `QueryBus` to the appropriate handler, which processes
/// the query and returns the result.
///
/// # Example
///
/// ```
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// # rt.block_on(async {
/// # use discern::query::Query;
/// # use discern::async_trait;
/// # use discern::query::QueryHandler;
/// #
/// # #[derive(Debug)]
/// # struct User {
/// #     id: u64,
/// #     username: String,
/// #     email: String,
/// # }
/// #
/// # #[derive(Debug)]
/// # enum GetUserError {
/// #     UserNotFound,
/// #     DatabaseError,
/// # }
/// #
/// # #[derive(Debug)]
/// # struct GetUserQuery {
/// #    user_id: u64,
/// # }
/// #
/// # impl Query for GetUserQuery {
/// #   type Output = User;
/// #   type Error = GetUserError;
/// # }
/// #
/// # struct GetUserQueryHandler;
/// #
/// # #[async_trait]
/// # impl QueryHandler<GetUserQuery> for GetUserQueryHandler {
/// #    async fn handle(&self, query: GetUserQuery) -> Result<User, GetUserError> {
/// #       // Retrieve user data.
/// #       Ok(User {
/// #           id: query.user_id,
/// #           username: "Alice".to_string(),
/// #           email: "alice@example.com".to_string(),
/// #       })
/// #   }
/// # }
/// use discern::query_registry;
/// use discern::query::QueryBus;
///
/// let registry = query_registry! {
///    GetUserQuery => GetUserQueryHandler { /* ... */ },
/// };
///
/// let query_bus = QueryBus::new(registry);
///
/// let query = GetUserQuery {
///    user_id: 1,
/// };
///
/// let result = query_bus.dispatch(query).await;
/// match result {
///     Ok(user) => {
///         # assert_eq!(user.username, "Alice".to_string());
///         println!("User name: {}", user.username);
///     },
///     Err(err) => {
///         # assert!(false);
///         match err {
///             GetUserError::UserNotFound => println!("User not found"),
///             GetUserError::DatabaseError => println!("A database error occurred"),
///         }
///     }
/// }
/// # });
/// ```
#[derive(Clone, Debug)]
pub struct QueryBus {
    #[doc(hidden)]
    registry: Arc<QueryHandlerRegistry>,
}

/// The `QueryBus` implementation.
impl QueryBus {
    /// Creates a new `QueryBus` instance.
    ///
    /// The `QueryBus` is initialized with a `QueryHandlerRegistry`, which contains
    /// all the registered query handlers.
    ///
    /// # Arguments
    ///
    /// * `registry` - The query handler registry.
    ///
    /// # Example
    ///
    /// ```
    /// use discern::query::QueryBus;
    /// use discern::registry::QueryHandlerRegistry;
    ///
    /// let registry = QueryHandlerRegistry::new();
    /// let query_bus = QueryBus::new(registry);
    ///
    /// # assert!(true);
    /// ```
    pub fn new(registry: QueryHandlerRegistry) -> Self {
        Self {
            registry: Arc::new(registry),
        }
    }

    /// Dispatches a query to its respective handler.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to dispatch.
    ///
    /// # Returns
    ///
    /// The result of the query handler, which includes the output data or an error.
    ///
    /// # Panics
    ///
    /// This method will panic if the query handler is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    /// # rt.block_on(async {
    /// # use discern::query::Query;
    /// # use discern::async_trait;
    /// # use discern::query::QueryHandler;
    /// #
    /// # #[derive(Debug)]
    /// # struct User {
    /// #     id: u64,
    /// #     username: String,
    /// #     email: String,
    /// # }
    /// #
    /// # #[derive(Debug)]
    /// # enum GetUserError {
    /// #     UserNotFound,
    /// #     DatabaseError,
    /// # }
    /// #
    /// # #[derive(Debug)]
    /// # struct GetUserQuery {
    /// #    user_id: u64,
    /// # }
    /// #
    /// # impl Query for GetUserQuery {
    /// #   type Output = User;
    /// #   type Error = GetUserError;
    /// # }
    /// #
    /// # struct GetUserQueryHandler;
    /// #
    /// # #[async_trait]
    /// # impl QueryHandler<GetUserQuery> for GetUserQueryHandler {
    /// #    async fn handle(&self, query: GetUserQuery) -> Result<User, GetUserError> {
    /// #       Ok(User {
    /// #           id: query.user_id,
    /// #           username: "Alice".to_string(),
    /// #           email: "alice@example.com".to_string(),
    /// #       })
    /// #   }
    /// # }
    /// use discern::query::QueryBus;
    /// use discern::query_registry;
    ///
    /// let registry = query_registry! {
    ///    GetUserQuery => GetUserQueryHandler { /* ... */ },
    /// };
    ///
    /// let query_bus = QueryBus::new(registry);
    ///
    /// let result = query_bus.dispatch(GetUserQuery { user_id: 1 }).await;
    ///
    /// match result {
    ///     Ok(user) => {
    ///         # assert_eq!(user.username, "Alice".to_string());
    ///         println!("User name: {}", user.username);
    ///     },
    ///     Err(err) => {
    ///         # assert!(false);
    ///         match err {
    ///             GetUserError::UserNotFound => println!("User not found"),
    ///             GetUserError::DatabaseError => println!("A database error occurred"),
    ///         }
    ///     }
    /// }
    /// # });
    /// ```
    pub async fn dispatch<Q: Query>(&self, query: Q) -> Result<Q::Output, Q::Error> {
        match self.registry.get_handler::<Q>() {
            Some(handler) => handler.handle(query).await,
            None => {
                panic!(
                    "No handler registered for query: {:?}",
                    std::any::type_name::<Q>()
                );
            }
        }
    }
}
