//! The `macros` module provides convenient macros for creating and registering command and query buses.
//!
//! This module includes macros for initializing `CommandBus` and `QueryBus` instances with their respective
//! handlers, as well as macros for creating and registering command and query handler registries.
//!
//! Key macros:
//!
//! - [command_bus](crate::command_bus): Creates a `CommandBus` and registers handlers.
//! - [command_registry](crate::command_registry): Creates a `CommandHandlerRegistry` and registers handlers.
//! - [query_bus](crate::query_bus): Creates a `QueryBus` and registers handlers.
//! - [query_registry](crate::query_registry): Creates a `QueryHandlerRegistry` and registers handlers.

/// A macro for creating a `CommandBus` instance.
///
/// This macro provides a convenient way to initialize a `CommandBus` and register multiple
/// command handlers at once.
///
/// # Usage
///
/// You can use this macro in two ways:
///
/// 1. **Providing only handlers:**
///
/// ```
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// # rt.block_on(async {
/// # use discern::command::Command;
/// # use discern::async_trait;
/// # use discern::command::CommandHandler;
/// #
/// # #[derive(Debug)]
/// # enum CreateUserError {
/// #    UsernameAlreadyExists,
/// #    EmailAlreadyExists,
/// # }
/// #
/// # #[derive(Debug)]
/// # struct CreateUserCommand {
/// #    username: String,
/// #    email: String,
/// # }
/// #
/// # impl Command for CreateUserCommand {
/// #   // The identifier of the newly created user.
/// #   type Metadata = u64;
/// #   // The error type that is returned if the command fails.
/// #   type Error = CreateUserError;
/// # }
/// #
/// # struct CreateUserCommandHandler;
/// #
/// # #[async_trait]
/// # impl CommandHandler<CreateUserCommand> for CreateUserCommandHandler {
/// #    async fn handle(&self, command: CreateUserCommand) -> Result<u64, CreateUserError> {
/// #       // Create a new user.
/// #       Ok(1)
/// #   }
/// # }
/// use discern::command_bus;
///
/// let command_bus = command_bus! {
///    CreateUserCommandHandler { /* ... */ },
/// };
/// #
/// #   let command = CreateUserCommand {
/// #       username: "alice".to_string(),
/// #       email: "alice@localhost".to_string(),
/// #   };
/// #
/// #   let result = command_bus.dispatch(command).await;
/// #   match result {
/// #       Ok(user_id) => {
/// #           assert_eq!(user_id, 1);
/// #           println!("User created with id: {}", user_id);
/// #       },
/// #       Err(err) => {
/// #           assert!(false);
/// #           eprintln!("Failed to create user: {:?}", err);
/// #       }
/// #   }
/// # });
/// ```
///
/// This assumes that the types of the handlers can be inferred automatically.
///
/// 2. **Providing type-handler pairs:**
///
/// ```
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// # rt.block_on(async {
/// # use discern::command::Command;
/// # use discern::async_trait;
/// # use discern::command::CommandHandler;
/// #
/// # #[derive(Debug)]
/// # enum CreateUserError {
/// #    UsernameAlreadyExists,
/// #    EmailAlreadyExists,
/// # }
/// #
/// # #[derive(Debug)]
/// # struct CreateUserCommand {
/// #    username: String,
/// #    email: String,
/// # }
/// #
/// # impl Command for CreateUserCommand {
/// #   // The identifier of the newly created user.
/// #   type Metadata = u64;
/// #   // The error type that is returned if the command fails.
/// #   type Error = CreateUserError;
/// # }
/// #
/// # struct CreateUserCommandHandler;
/// #
/// # #[async_trait]
/// # impl CommandHandler<CreateUserCommand> for CreateUserCommandHandler {
/// #    async fn handle(&self, command: CreateUserCommand) -> Result<u64, CreateUserError> {
/// #       // Create a new user.
/// #       Ok(1)
/// #   }
/// # }
/// use discern::command_bus;
///
/// let command_bus = command_bus! {
///    CreateUserCommand => CreateUserCommandHandler { /* ... */ },
/// };
/// #
/// #   let command = CreateUserCommand {
/// #       username: "alice".to_string(),
/// #       email: "alice@localhost".to_string(),
/// #   };
/// #
/// #   let result = command_bus.dispatch(command).await;
/// #   match result {
/// #       Ok(user_id) => {
/// #           assert_eq!(user_id, 1);
/// #           println!("User created with id: {}", user_id);
/// #       },
/// #       Err(err) => {
/// #           assert!(false);
/// #           eprintln!("Failed to create user: {:?}", err);
/// #       }
/// #   }
/// # });
/// ```
/// This explicitly specifies the command type associated with each handler.
///
/// # See Also
///
/// - [CommandBus](crate::command::CommandBus)
#[macro_export]
macro_rules! command_bus {
        () => {{
            $crate::command::CommandBus::new($crate::registry::CommandHandlerRegistry::new())
        }};
        ($($handler:expr),*$(,)?) => {{
            let mut command_handler_registry = $crate::registry::CommandHandlerRegistry::new();
            $(command_handler_registry.register($handler);)*
            $crate::command::CommandBus::new(command_handler_registry)
        }};
        ($($command:ty => $handler:expr),*$(,)?) => {{
            let mut command_handler_registry = $crate::registry::CommandHandlerRegistry::new();
            $(command_handler_registry.register::<$command>($handler);)*
            $crate::command::CommandBus::new(command_handler_registry)
        }};
    }

/// A macro for creating a `CommandHandlerRegistry` instance.
///
/// This macro provides a convenient way to initialize a `CommandHandlerRegistry` and register multiple
/// command handlers at once.
///
/// # Usage
///
/// You can use this macro in two ways:
///
/// 1. **Providing only handlers:**
///
/// ```
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// # rt.block_on(async {
/// # use discern::command::Command;
/// # use discern::async_trait;
/// # use discern::command::CommandHandler;
/// #
/// # #[derive(Debug)]
/// # enum CreateUserError {
/// #    UsernameAlreadyExists,
/// #    EmailAlreadyExists,
/// # }
/// #
/// # #[derive(Debug)]
/// # struct CreateUserCommand {
/// #    username: String,
/// #    email: String,
/// # }
/// #
/// # impl Command for CreateUserCommand {
/// #   // The identifier of the newly created user.
/// #   type Metadata = u64;
/// #   // The error type that is returned if the command fails.
/// #   type Error = CreateUserError;
/// # }
/// #
/// # struct CreateUserCommandHandler;
/// #
/// # #[async_trait]
/// # impl CommandHandler<CreateUserCommand> for CreateUserCommandHandler {
/// #    async fn handle(&self, command: CreateUserCommand) -> Result<u64, CreateUserError> {
/// #       // Create a new user.
/// #       Ok(1)
/// #   }
/// # }
/// use discern::command_registry;
///
/// let command_registry = command_registry! {
///    CreateUserCommandHandler { /* ... */ },
/// };
/// # });
/// ```
///
/// This assumes that the types of the handlers can be inferred automatically.
///
/// 2. **Providing type-handler pairs:**
///
/// ```
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// # rt.block_on(async {
/// # use discern::command::Command;
/// # use discern::async_trait;
/// # use discern::command::CommandHandler;
/// #
/// # #[derive(Debug)]
/// # enum CreateUserError {
/// #    UsernameAlreadyExists,
/// #    EmailAlreadyExists,
/// # }
/// #
/// # #[derive(Debug)]
/// # struct CreateUserCommand {
/// #    username: String,
/// #    email: String,
/// # }
/// #
/// # impl Command for CreateUserCommand {
/// #   // The identifier of the newly created user.
/// #   type Metadata = u64;
/// #   // The error type that is returned if the command fails.
/// #   type Error = CreateUserError;
/// # }
/// #
/// # struct CreateUserCommandHandler;
/// #
/// # #[async_trait]
/// # impl CommandHandler<CreateUserCommand> for CreateUserCommandHandler {
/// #    async fn handle(&self, command: CreateUserCommand) -> Result<u64, CreateUserError> {
/// #       // Create a new user.
/// #       Ok(1)
/// #   }
/// # }
/// use discern::command_registry;
///
/// let command_registry = command_registry! {
///    CreateUserCommand => CreateUserCommandHandler { /* ... */ },
/// };
/// # });
/// ```
/// This explicitly specifies the command type associated with each handler.
///
/// # See Also
///
/// - [CommandHandlerRegistry](crate::registry::CommandHandlerRegistry)
#[macro_export]
macro_rules! command_registry {
        () => {{
            $crate::command::CommandHandlerRegistry::new()
        }};
        ($($handler:expr),*$(,)?) => {{
            let mut command_handler_registry = $crate::registry::CommandHandlerRegistry::new();
            $(command_handler_registry.register($handler);)*
            command_handler_registry
        }};
        ($($command:ty => $handler:expr),*$(,)?) => {{
            let mut command_handler_registry = $crate::registry::CommandHandlerRegistry::new();
            $(command_handler_registry.register::<$command>($handler);)*
            command_handler_registry
        }};
    }
/// A macro for creating a `QueryBus` instance.
///
/// This macro provides a convenient way to initialize a `QueryBus` and register multiple
/// query handlers at once.
///
/// # Usage
///
/// You can use this macro in two ways:
///
/// 1. **Providing only handlers:**
///
/// ```
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// # rt.block_on(async {
/// # use discern::query::Query;
/// # use discern::async_trait;
/// # use discern::query::QueryHandler;
/// #
/// # #[derive(Debug)]
/// # struct GetUserQuery {
/// #     user_id: u64,
/// # }
/// #
/// # impl Query for GetUserQuery {
/// #     type Output = String;
/// #     type Error = std::io::Error;
/// # }
/// #
/// # struct GetUserQueryHandler;
/// #
/// # #[async_trait]
/// # impl QueryHandler<GetUserQuery> for GetUserQueryHandler {
/// #     async fn handle(&self, query: GetUserQuery) -> Result<String, std::io::Error> {
/// #         Ok("Alice".to_string())
/// #     }
/// # }
/// use discern::query_bus;
///
/// let query_bus = query_bus! {
///     GetUserQueryHandler { /* ... */ },
/// };
/// #
/// #   let query = GetUserQuery { user_id: 1 };
/// #   let result = query_bus.dispatch(query).await;
/// #   match result {
/// #       Ok(name) => println!("User name: {}", name),
/// #       Err(err) => eprintln!("Failed to get user: {:?}", err),
/// #   }
/// # });
/// ```
///
/// This assumes that the types of the handlers can be inferred automatically.
///
/// 2. **Providing type-handler pairs:**
///
/// ```
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// # rt.block_on(async {
/// # use discern::query::Query;
/// # use discern::async_trait;
/// # use discern::query::QueryHandler;
/// #
/// # #[derive(Debug)]
/// # struct GetUserQuery {
/// #     user_id: u64,
/// # }
/// #
/// # impl Query for GetUserQuery {
/// #     type Output = String;
/// #     type Error = std::io::Error;
/// # }
/// #
/// # struct GetUserQueryHandler;
/// #
/// # #[async_trait]
/// # impl QueryHandler<GetUserQuery> for GetUserQueryHandler {
/// #     async fn handle(&self, query: GetUserQuery) -> Result<String, std::io::Error> {
/// #         Ok("Alice".to_string())
/// #     }
/// # }
/// use discern::query_bus;
///
/// let query_bus = query_bus! {
///     GetUserQuery => GetUserQueryHandler { /* ... */ },
/// };
/// #
/// #   let query = GetUserQuery { user_id: 1 };
/// #   let result = query_bus.dispatch(query).await;
/// #   match result {
/// #       Ok(name) => println!("User name: {}", name),
/// #       Err(err) => eprintln!("Failed to get user: {:?}", err),
/// #   }
/// # });
/// ```
///
/// This explicitly specifies the query type associated with each handler.
///
/// # See Also
///
/// - [QueryBus](crate::query::QueryBus)
#[macro_export]
macro_rules! query_bus {
        () => {{
            $crate::query::QueryBus::new($crate::registry::QueryHandlerRegistry::new())
        }};
        ($($handler:expr),*$(,)?) => {{
            let mut query_handler_registry = $crate::registry::QueryHandlerRegistry::new();
            $(query_handler_registry.register($handler);)*
            $crate::query::QueryBus::new(query_handler_registry)
        }};
        ($($query:ty => $handler:expr),*$(,)?) => {{
            let mut query_handler_registry = $crate::registry::QueryHandlerRegistry::new();
            $(query_handler_registry.register::<$query>($handler);)*
            $crate::query::QueryBus::new(query_handler_registry)
        }};
    }

/// A macro for creating a `QueryHandlerRegistry` instance.
///
/// This macro provides a convenient way to initialize a `QueryHandlerRegistry` and register multiple
/// query handlers at once.
///
/// # Usage
///
/// You can use this macro in two ways:
///
/// 1. **Providing only handlers:**
///
/// ```
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// # rt.block_on(async {
/// # use discern::query::Query;
/// # use discern::async_trait;
/// # use discern::query::QueryHandler;
/// #
/// # #[derive(Debug)]
/// # struct GetUserQuery {
/// #     user_id: u64,
/// # }
/// #
/// # impl Query for GetUserQuery {
/// #     type Output = String;
/// #     type Error = std::io::Error;
/// # }
/// #
/// # struct GetUserQueryHandler;
/// #
/// # #[async_trait]
/// # impl QueryHandler<GetUserQuery> for GetUserQueryHandler {
/// #     async fn handle(&self, query: GetUserQuery) -> Result<String, std::io::Error> {
/// #         Ok("Alice".to_string())
/// #     }
/// # }
/// use discern::query_registry;
///
/// let query_registry = query_registry! {
///     GetUserQueryHandler { /* ... */ },
/// };
/// # });
/// ```
///
/// This assumes that the types of the handlers can be inferred automatically.
///
/// 2. **Providing type-handler pairs:**
///
/// ```
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// # rt.block_on(async {
/// # use discern::query::Query;
/// # use discern::async_trait;
/// # use discern::query::QueryHandler;
/// #
/// # #[derive(Debug)]
/// # struct GetUserQuery {
/// #     user_id: u64,
/// # }
/// #
/// # impl Query for GetUserQuery {
/// #     type Output = String;
/// #     type Error = std::io::Error;
/// # }
/// #
/// # struct GetUserQueryHandler;
/// #
/// # #[async_trait]
/// # impl QueryHandler<GetUserQuery> for GetUserQueryHandler {
/// #     async fn handle(&self, query: GetUserQuery) -> Result<String, std::io::Error> {
/// #         Ok("Alice".to_string())
/// #     }
/// # }
/// use discern::query_registry;
///
/// let query_registry = query_registry! {
///     GetUserQuery => GetUserQueryHandler { /* ... */ },
/// };
/// # });
/// ```
///
/// This explicitly specifies the query type associated with each handler.
///
/// # See Also
///
/// - [QueryHandlerRegistry](crate::registry::QueryHandlerRegistry)
#[macro_export]
macro_rules! query_registry {
        () => {{
            $crate::query::QueryHandlerRegistry::new()
        }};
        ($($handler:expr),*$(,)?) => {{
            let mut query_handler_registry = $crate::registry::QueryHandlerRegistry::new();
            $(query_handler_registry.register($handler);)*
            query_handler_registry
        }};
        ($($query:ty => $handler:expr),*$(,)?) => {{
            let mut query_handler_registry = $crate::registry::QueryHandlerRegistry::new();
            $(query_handler_registry.register::<$query>($handler);)*
            query_handler_registry
        }};
    }
