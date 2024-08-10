# Discern

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/yourusername/discern/ci.yml)
![License](https://img.shields.io/github/license/yourusername/discern)

Discern is a Rust library that implements the Command Query Responsibility Segregation (CQRS) pattern. It provides an easy-to-use framework for separating the write operations (commands) from the read operations (queries) in your Rust applications.

## Features

- **Command Handling**: Easily define commands that change the state of your system.
- **Query Handling**: Define queries that retrieve data without modifying the state.
- **Handler Registration**: Register command and query handlers using convenient macros.
- **Async Support**: Fully asynchronous handling of commands and queries using `async_trait`.

## Installation

Add `discern` to your `Cargo.toml`:

```toml
[dependencies]
discern = "0.1.0"
```

## Usage

Below is a simple example of how to use Discern to create a command bus that handles a `CreateUserCommand`:

```rust
use discern::async_trait;
use discern::command::Command;
use discern::command::CommandBus;
use discern::command::CommandHandler;
use discern::command_bus;

#[derive(Debug)]
struct CreateUserCommand {
    username: String,
    email: String,
}

#[derive(Debug)]
enum CreateUserError {
    UsernameAlreadyExists,
    EmailAlreadyExists,
}

#[derive(Debug)]
struct CreateUserMetadata(u32);

impl Command for CreateUserCommand {
    type Metadata = CreateUserMetadata;
    type Error = CreateUserError;
}

struct CreateUserCommandHandler {
    // Add your dependencies here
}

#[async_trait]
impl CommandHandler<CreateUserCommand> for CreateUserCommandHandler {
    async fn handle(&self, command: CreateUserCommand) -> Result<CreateUserMetadata, CreateUserError> {
        // Add your command handling logic here, e.g.:
        // let user = User::new(command.username, command.email);
        // let id = self.repository.persist(user);
        let id = 42;

        Ok(CreateUserMetadata(id))
    }
}

#[tokio::main]
async fn main() {
    let command_bus: CommandBus = command_bus! {
        CreateUserCommand => CreateUserCommandHandler {
            // Add your dependencies here
        },
    };

    let command = CreateUserCommand {
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    match command_bus.dispatch(command).await {
        Ok(metadata) => {
            println!("User created with ID: {}", metadata.0);
        }
        Err(error) => {
            eprintln!("Failed to create user: {:?}", err);
        }
    }
}
```

## Documentation

- [API Documentation](https://docs.rs/discern)

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
