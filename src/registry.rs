//! The `registry` module provides the infrastructure for managing and retrieving command and query handlers.
//!
//! This module contains the `CommandHandlerRegistry` and `QueryHandlerRegistry` structs, which are responsible for
//! maintaining the mappings between command/query types and their corresponding handlers. These registries
//! are used internally by the `CommandBus` and `QueryBus` to dispatch commands and queries to the correct handlers.
//!
//! The `executor` submodule is an internal implementation detail used by the registries to execute commands and queries.
//!
//! - [CommandHandlerRegistry]: The registry for command handlers.
//! - [QueryHandlerRegistry]: The registry for query handlers.

use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FormatterResult;
use std::sync::Arc;

use crate::command::Command;
use crate::command::CommandHandler;
use crate::query::Query;
use crate::query::QueryHandler;
use crate::registry::executor::CommandHandlerWrapper;
use crate::registry::executor::QueryHandlerWrapper;

/// The `CommandHandlerRegistry` struct manages the registration and retrieval of command handlers.
///
/// This registry maintains a mapping between command types and their corresponding handlers.
/// It is used internally by the `CommandBus` to dispatch commands to the correct handler.
#[derive(Default)]
pub struct CommandHandlerRegistry {
    #[doc(hidden)]
    pub(crate) handlers: HashMap<TypeId, Arc<dyn CommandHandlerWrapper>>,
}

/// The `QueryHandlerRegistry` struct manages the registration and retrieval of query handlers.
///
/// This registry maintains a mapping between query types and their corresponding handlers.
/// It is used internally by the `QueryBus` to dispatch queries to the correct handler.
#[derive(Default)]
pub struct QueryHandlerRegistry {
    #[doc(hidden)]
    pub(crate) handlers: HashMap<TypeId, Arc<dyn QueryHandlerWrapper>>,
}

/// `CommandHandlerRegistry` implementation.
impl CommandHandlerRegistry {
    /// Creates a new, empty `CommandHandlerRegistry`.
    ///
    /// # Example
    ///
    /// ```
    /// use discern::registry::CommandHandlerRegistry;
    ///
    /// let registry = CommandHandlerRegistry::new();
    /// # assert!(true);
    /// ```
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Registers a command handler for a specific command type.
    ///
    /// # Arguments
    ///
    /// * `handler` - The handler to be registered for the command type `C`.
    ///
    /// This method associates a command type with its corresponding handler.
    /// When a command of type `C` is dispatched, the `CommandBus` will use the handler registered here.
    ///
    /// # Example
    ///
    /// ```
    /// # use discern::command::{Command, CommandHandler};
    /// # use discern::async_trait;
    /// # use discern::registry::CommandHandlerRegistry;
    /// #
    /// # #[derive(Debug)]
    /// # struct MyCommand;
    /// #
    /// # impl Command for MyCommand {
    /// #   type Metadata = ();
    /// #   type Error = std::io::Error;
    /// # }
    /// #
    /// # #[derive(Debug)]
    /// # struct MyCommandHandler;
    /// #
    /// # #[async_trait]
    /// # impl CommandHandler<MyCommand> for MyCommandHandler {
    /// #   async fn handle(&self, _command: MyCommand) -> Result<(), std::io::Error> {
    /// #     Ok(())
    /// #   }
    /// # }
    /// let mut registry = CommandHandlerRegistry::new();
    /// registry.register::<MyCommand>(MyCommandHandler { /* ... */ });
    /// # assert!(true);
    /// ```
    pub fn register<C: Command>(&mut self, handler: impl CommandHandler<C> + 'static) {
        self.handlers.insert(
            TypeId::of::<C>(),
            Arc::new(Box::new(handler) as Box<dyn CommandHandler<C>>),
        );
    }

    /// Retrieves the command handler for a specific command type.
    ///
    /// # Returns
    ///
    /// An `Option` containing the command handler if found, or `None` if no handler is registered for the command type `C`.
    ///
    /// # Example
    ///
    /// ```
    /// # use discern::command::{Command, CommandHandler};
    /// # use discern::async_trait;
    /// # use discern::registry::CommandHandlerRegistry;
    /// #
    /// # #[derive(Debug)]
    /// # struct MyCommand;
    /// #
    /// # impl Command for MyCommand {
    /// #   type Metadata = ();
    /// #   type Error = std::io::Error;
    /// # }
    /// #
    /// # #[derive(Debug)]
    /// # struct MyCommandHandler;
    /// #
    /// # #[async_trait]
    /// # impl CommandHandler<MyCommand> for MyCommandHandler {
    /// #   async fn handle(&self, _command: MyCommand) -> Result<(), std::io::Error> {
    /// #     Ok(())
    /// #   }
    /// # }
    /// let mut registry = CommandHandlerRegistry::new();
    /// registry.register(MyCommandHandler { /* ... */ });
    ///
    /// let handler = registry.get_handler::<MyCommand>();
    /// assert!(handler.is_some());
    /// ```
    pub fn get_handler<C: Command>(&self) -> Option<Box<dyn CommandHandler<C>>> {
        self.handlers
            .get(&TypeId::of::<C>())
            .cloned()
            .map(|handler| Box::new(handler) as Box<dyn CommandHandler<C>>)
    }
}

/// `QueryHandlerRegistry` implementation.
impl QueryHandlerRegistry {
    /// Creates a new, empty `QueryHandlerRegistry`.
    ///
    /// # Example
    ///
    /// ```
    /// use discern::registry::QueryHandlerRegistry;
    ///
    /// let registry = QueryHandlerRegistry::new();
    /// # assert!(true);
    /// ```
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Registers a query handler for a specific query type.
    ///
    /// # Arguments
    ///
    /// * `handler` - The handler to be registered for the query type `Q`.
    ///
    /// This method associates a query type with its corresponding handler.
    /// When a query of type `Q` is dispatched, the `QueryBus` will use the handler registered here.
    ///
    /// # Example
    ///
    /// ```
    /// # use discern::query::{Query, QueryHandler};
    /// # use discern::async_trait;
    /// # use discern::registry::QueryHandlerRegistry;
    /// #
    /// # #[derive(Debug)]
    /// # struct MyQuery;
    /// #
    /// # impl Query for MyQuery {
    /// #   type Output = String;
    /// #   type Error = std::io::Error;
    /// # }
    /// #
    /// # #[derive(Debug)]
    /// # struct MyQueryHandler;
    /// #
    /// # #[async_trait]
    /// # impl QueryHandler<MyQuery> for MyQueryHandler {
    /// #   async fn handle(&self, _query: MyQuery) -> Result<String, std::io::Error> {
    /// #     Ok("Result".to_string())
    /// #   }
    /// # }
    /// let mut registry = QueryHandlerRegistry::new();
    /// registry.register(MyQueryHandler);
    /// # assert!(true);
    /// ```
    pub fn register<Q: Query>(&mut self, handler: impl QueryHandler<Q> + 'static) {
        self.handlers.insert(
            TypeId::of::<Q>(),
            Arc::new(Box::new(handler) as Box<dyn QueryHandler<Q>>),
        );
    }

    /// Retrieves the query handler for a specific query type.
    ///
    /// # Returns
    ///
    /// An `Option` containing the query handler if found, or `None` if no handler is registered for the query type `Q`.
    ///
    /// # Example
    ///
    /// ```
    /// # use discern::query::{Query, QueryHandler};
    /// # use discern::async_trait;
    /// # use discern::registry::QueryHandlerRegistry;
    /// #
    /// # #[derive(Debug)]
    /// # struct MyQuery;
    /// #
    /// # impl Query for MyQuery {
    /// #   type Output = String;
    /// #   type Error = std::io::Error;
    /// # }
    /// #
    /// # #[derive(Debug)]
    /// # struct MyQueryHandler;
    /// #
    /// # #[async_trait]
    /// # impl QueryHandler<MyQuery> for MyQueryHandler {
    /// #   async fn handle(&self, _query: MyQuery) -> Result<String, std::io::Error> {
    /// #     Ok("Result".to_string())
    /// #   }
    /// # }
    /// let mut registry = QueryHandlerRegistry::new();
    /// registry.register(MyQueryHandler);
    ///
    /// let handler = registry.get_handler::<MyQuery>();
    /// assert!(handler.is_some());
    /// ```
    pub fn get_handler<Q: Query>(&self) -> Option<Box<dyn QueryHandler<Q>>> {
        self.handlers
            .get(&TypeId::of::<Q>())
            .cloned()
            .map(|handler| Box::new(handler) as Box<dyn QueryHandler<Q>>)
    }
}

/// Debug implementation for `CommandHandlerRegistry`
impl Debug for CommandHandlerRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatterResult {
        f.debug_struct("CommandHandlerRegistry").finish()
    }
}

/// Debug implementation for `QueryHandlerRegistry`
impl Debug for QueryHandlerRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatterResult {
        f.debug_struct("QueryHandlerRegistry").finish()
    }
}

#[doc(hidden)]
mod executor {
    use std::any::Any;
    use std::sync::Arc;

    use crate::async_trait;
    use crate::command::Command;
    use crate::command::CommandHandler;
    use crate::query::Query;
    use crate::query::QueryHandler;

    #[async_trait]
    pub trait CommandHandlerWrapper: Send + Sync {
        async fn execute(&self, command: Box<dyn Any + Send>) -> Box<dyn Any + Send>;
    }

    #[async_trait]
    pub trait QueryHandlerWrapper: Send + Sync {
        async fn execute(&self, query: Box<dyn Any + Send>) -> Box<dyn Any + Send>;
    }

    #[async_trait]
    impl<C: Command> CommandHandlerWrapper for Box<dyn CommandHandler<C>> {
        async fn execute(&self, command: Box<dyn Any + Send>) -> Box<dyn Any + Send> {
            let command = *command.downcast::<C>().unwrap();
            let result = self.handle(command).await;
            Box::new(result) as Box<dyn Any + Send>
        }
    }

    #[async_trait]
    impl<Q: Query> QueryHandlerWrapper for Box<dyn QueryHandler<Q>> {
        async fn execute(&self, query: Box<dyn Any + Send>) -> Box<dyn Any + Send> {
            let result = self.handle(*query.downcast::<Q>().unwrap()).await;
            Box::new(result) as Box<dyn Any + Send>
        }
    }

    #[async_trait]
    impl<C: Command> CommandHandler<C> for Arc<dyn CommandHandlerWrapper> {
        async fn handle(&self, command: C) -> Result<C::Metadata, C::Error> {
            let result = self.execute(Box::new(command)).await;
            *result.downcast().unwrap()
        }
    }

    #[async_trait]
    impl<Q: Query> QueryHandler<Q> for Arc<dyn QueryHandlerWrapper> {
        async fn handle(&self, query: Q) -> Result<Q::Output, Q::Error> {
            let result = self.execute(Box::new(query)).await;
            *result.downcast().unwrap()
        }
    }
}
