# rust_todo_cli

A simple command-line todo application written in Rust.

## Installation

Ensure that you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

Clone the repository and navigate to the project directory:

```sh
git clone <repository-url>
cd rust_todo_cli
```

Build the project using Cargo:

```sh
cargo build --release
```

## Usage

To use the todo CLI application, run the following command pattern:

```sh
todo <command> [arguments]
```

The available commands are:
- `add <task_description>`: Add a new task.
- `remove <task_id>`: Remove a task by its ID.
- `list`: List all tasks.

### Examples

Add a new task:

```sh
todo add "Finish writing documentation"
```

Remove a task:

```sh
todo remove 1
```

List all tasks:

```sh
todo list
```

## Project Structure

- `src/main.rs`: The main source code for the application.
- `Cargo.toml`: The project's manifest file, which includes metadata and dependency information.

## Dependencies

- `serde`
- `serde_json`

## License

This project is licensed unser the MIT License.
