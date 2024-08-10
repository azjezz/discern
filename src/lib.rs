//! The `discern` crate is an implementation of the CQRS (Command Query Responsibility Segregation) pattern in Rust.
//!
//! # Command Query Responsibility Segregation (CQRS)
//!
//! CQRS is a pattern that separates the write operations (commands) from read operations (queries).
//!
//! The `CommandBus` is responsible for handling commands that change state in the system, while the `QueryBus` is responsible for handling queries that retrieve data.
//!
//! Commands are often associated with actions that modify the system's state, while queries are used to retrieve data without making any modifications.
//!
//! The `CommandBus` and `QueryBus` are designed to ensure that commands and queries are processed by their respective handlers, which are registered in handler registries.
//! This allows for decoupled, maintainable, and scalable code.
//!
//! - [CommandBus](crate::command::CommandBus): Dispatches commands to their respective handlers.
//! - [QueryBus](crate::query::QueryBus): Dispatches queries to their respective handlers.
//!
//! # Example: Handling Commands
//!
//! The following example demonstrates how to define and handle commands in a CQRS-based system.
//! In this example, we create a `CreateUserCommand` that is responsible for creating a new user.
//! The `CreateUserCommandHandler` processes this command and returns the ID of the newly created user.
//!
//! ```rust
//! use discern::async_trait;
//! use discern::command_bus;
//! use discern::command::Command;
//! use discern::command::CommandHandler;
//!
//! // Define the CreateUserCommand.
//! #[derive(Debug)]
//! struct CreateUserCommand {
//!     username: String,
//!     email: String,
//! }
//!
//! // Define possible errors for the CreateUserCommand.
//! #[derive(Debug)]
//! enum CreateUserError {
//!     UsernameAlreadyExists,
//!     EmailAlreadyExists,
//! }
//!
//! // Implement the Command trait for CreateUserCommand.
//! impl Command for CreateUserCommand {
//!     type Metadata = u64; // Return the ID of the created user as metadata.
//!     type Error = CreateUserError;
//! }
//!
//! // Define a handler for the CreateUserCommand.
//! struct CreateUserCommandHandler {
//!     // Add any dependencies needed by the handler.
//! }
//!
//! #[async_trait]
//! impl CommandHandler<CreateUserCommand> for CreateUserCommandHandler {
//!     async fn handle(&self, _command: CreateUserCommand) -> Result<u64, CreateUserError> {
//!         # return Ok(0);
//!         // Handle command logic here, e.g., create a user, validate uniqueness, etc.
//!         todo!("Implement user creation logic");
//!     }
//! }
//!
//! # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
//! # rt.block_on(async {
//! // Create a command bus with the CreateUserCommand and its handler.
//! let command_bus = command_bus! {
//!     CreateUserCommand => CreateUserCommandHandler { /* ... */ },
//! };
//!
//! // Create a command to add a new user.
//! let command = CreateUserCommand {
//!     username: "alice".to_string(),
//!     email: "alice@example.com".to_string(),
//! };
//!
//! // Dispatch the command.
//! match command_bus.dispatch(command).await {
//!     Ok(user_id) => println!("User created with ID: {}", user_id),
//!     Err(err) => println!("Failed to create user: {:?}", err),
//! }
//! # });
//! ```
//!
//! # Example: Handling Queries
//!
//! //! # Example: Handling Queries
//!
//! The following example demonstrates how to define and handle queries in a CQRS-based system.
//! In this example, we create a `GetUserQuery` that is responsible for retrieving user information by user ID.
//! The `GetUserQueryHandler` processes this query and returns the user's information if found.
//!
//! ```rust
//! use discern::async_trait;
//! use discern::query::Query;
//! use discern::query::QueryHandler;
//! use discern::query_bus;
//!
//! // Define the GetUserQuery.
//! #[derive(Debug)]
//! struct GetUserQuery {
//!     user_id: u64,
//! }
//!
//! // Define possible errors for the GetUserQuery.
//! #[derive(Debug)]
//! enum GetUserError {
//!     UserNotFound,
//! }
//!
//! // Define a User struct.
//! #[derive(Debug)]
//! struct User {
//!     id: u64,
//!     username: String,
//!     email: String,
//! }
//!
//! // Implement the Query trait for GetUserQuery.
//! impl Query for GetUserQuery {
//!     type Output = User; // Return the user as output.
//!     type Error = GetUserError;
//! }
//!
//! // Define a handler for the GetUserQuery.
//! struct GetUserQueryHandler;
//!
//! #[async_trait]
//! impl QueryHandler<GetUserQuery> for GetUserQueryHandler {
//!     async fn handle(&self, _query: GetUserQuery) -> Result<User, GetUserError> {
//!         # return Ok(User { id: 0, username: "".to_string(), email: "".to_string() });
//!         // Handle query logic here, e.g., retrieve the user by ID.
//!         todo!("Implement user retrieval logic");
//!     }
//! }
//!
//! # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
//! # rt.block_on(async {
//! // Create a query bus with the GetUserQuery and its handler.
//! let query_bus = query_bus! {
//!     GetUserQuery => GetUserQueryHandler { /* ... */ },
//! };
//!
//! // Create a query to retrieve the user with ID 1.
//! let query = GetUserQuery { user_id: 1 };
//!
//! // Dispatch the query.
//! match query_bus.dispatch(query).await {
//!     Ok(user) => println!("User found: {:?}", user),
//!     Err(err) => println!("Failed to retrieve user: {:?}", err),
//! }
//! # });
//! ```

pub mod command;
pub mod macros;
pub mod query;
pub mod registry;

/// Re-exports the `async_trait` crate.
///
/// This crate provides a procedural macro for defining async traits in Rust.
///
/// For more details on `#[async_trait]`, see [mod@async_trait]
pub use async_trait::async_trait;
