//! The `command` module defines the core abstractions and components related to handling commands in the CQRS pattern.
//!
//! Commands are actions that change the state of the system. This module provides the `Command` trait, which represents
//! a command, and the `CommandHandler` trait, which defines the behavior for handling commands.
//!
//! The `CommandBus` is responsible for dispatching commands to their respective handlers. It utilizes the
//! `CommandHandlerRegistry` from the [registry](crate::registry) module to manage and retrieve the appropriate handlers.
//!
//! - [Command]: Represents a command in the system.
//! - [CommandHandler]: Trait for handling commands.
//! - [CommandBus]: Dispatches commands to the appropriate handlers.
//!
//! # See Also
//!
//! - [CommandHandlerRegistry]: Manages command handlers.

use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

use crate::async_trait;
use crate::registry::CommandHandlerRegistry;

/// The `Command` trait represents a command that changes the state of the system.
///
/// # Example
///
/// ```
/// use discern::command::Command;
///
/// #[derive(Debug)]
/// enum CreateUserError {
///    UsernameAlreadyExists,
///    EmailAlreadyExists,
/// }
///
/// #[derive(Debug)]
/// struct CreateUserCommand {
///    username: String,
///    email: String,
/// }
///
/// impl Command for CreateUserCommand {
///   // The identifier of the newly created user.
///   type Metadata = u64;
///   // The error type that is returned if the command fails.
///   type Error = CreateUserError;
/// }
/// ```
pub trait Command: Send + Sync + Any + Debug {
    /// The metadata type that contains information about the command's execution,
    /// e.g., the identifier of a newly created entity.
    ///
    /// This type must implement the `Debug`, `Send`, and `Sync` traits.
    type Metadata: Debug + Send + Sync;

    /// The error type that is returned if the command fails.
    ///
    /// This type must implement the `Debug`, `Send`, and `Sync` traits.
    type Error: Debug + Send + Sync;
}

/// The `CommandHandler` trait represents a handler that processes a command.
///
/// # Example
///
/// ```
/// # use discern::command::Command;
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
/// use discern::async_trait;
/// use discern::command::CommandHandler;
///
/// struct CreateUserCommandHandler;
///
/// #[async_trait]
/// impl CommandHandler<CreateUserCommand> for CreateUserCommandHandler {
///    async fn handle(&self, command: CreateUserCommand) -> Result<u64, CreateUserError> {
///       // Create a new user.
///       Ok(1)
///   }
/// }
/// ```
#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    async fn handle(&self, command: C) -> Result<C::Metadata, C::Error>;
}

/// The `CommandBus` is responsible for dispatching commands to their respective handlers.
///
/// # Example
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
/// use discern::command::CommandBus;
///
/// let registry = command_registry! {
///    CreateUserCommand => CreateUserCommandHandler { /* ... */ },
/// };
///
/// let command_bus = CommandBus::new(registry);
///
/// let command = CreateUserCommand {
///   username: "alice".to_string(),
///   email: "alice@localhost".to_string(),
/// };
///
/// let result = command_bus.dispatch(command).await;
/// match result {
///     Ok(user_id) => {
///         # assert_eq!(user_id, 1);
///         println!("User created with id: {}", user_id);
///     },
///     Err(err) => {
///         # assert!(false);
///         eprintln!("Failed to create user: {:?}", err);
///     }
/// }
/// # });
/// ```
#[derive(Clone, Debug)]
pub struct CommandBus {
    #[doc(hidden)]
    registry: Arc<CommandHandlerRegistry>,
}

/// The `CommandBus` implementation.
impl CommandBus {
    /// Creates a new `CommandBus` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use discern::command::CommandBus;
    /// use discern::registry::CommandHandlerRegistry;
    ///
    /// let registry = CommandHandlerRegistry::new();
    /// let command_bus = CommandBus::new(registry);
    ///
    /// # assert!(true);
    /// ```
    pub fn new(registry: CommandHandlerRegistry) -> Self {
        Self {
            registry: Arc::new(registry),
        }
    }

    /// Dispatches a command to its respective handler.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to dispatch.
    ///
    /// # Returns
    ///
    /// The result of the command handler.
    ///
    /// # Panics
    ///
    /// This method will panic if the command handler is not found.
    ///
    /// # Example
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
    /// use discern::command::CommandBus;
    ///
    /// let registry = command_registry! {
    ///    CreateUserCommand => CreateUserCommandHandler { /* ... */ },
    /// };
    ///
    /// let command_bus = CommandBus::new(registry);
    ///
    /// let command = CreateUserCommand {
    ///   username: "alice".to_string(),
    ///   email: "alice@localhost".to_string(),
    /// };
    ///
    /// let result = command_bus.dispatch(command).await;
    /// match result {
    ///     Ok(user_id) => {
    ///         # assert_eq!(user_id, 1);
    ///         println!("User created with id: {}", user_id);
    ///     },
    ///     Err(err) => {
    ///         # assert!(false);
    ///         eprintln!("Failed to create user: {:?}", err);
    ///     }
    /// }
    /// # });
    /// ```
    pub async fn dispatch<C: Command>(&self, command: C) -> Result<C::Metadata, C::Error> {
        match self.registry.get_handler::<C>() {
            None => {
                panic!(
                    "No handler registered for command: {:?}",
                    std::any::type_name::<C>()
                );
            }
            Some(handler) => handler.handle(command).await,
        }
    }
}
