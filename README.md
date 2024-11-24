# STRADA: SQLite Database Server

## Overview

STRADA is a lightweight, Rust-based SQLite database server that provides REST API endpoints for database operations.

## Features

- SQLite database connection
- Schema parsing
- RESTful query execution
- Async server implementation

## Prerequisites

- Rust (1.65+ / 2021 Edition recommended)
- Cargo (Comes with Rust Installation)
- SQLite3

## Installation

1. Clone the repository:
```bash
git clone https://github.com/stradadb/strada.git
cd strada
```

2. Build the project:
```bash
cargo build --release
```

## Configuration

### Environment Variables

- `DATABASE_PATH`: Path to your SQLite database file
  - Default: `./strada.db`

## Development

### Running Tests

```bash
cargo test
```

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release
```

## Project Structure

- `src/database/connection.rs`: Database connection management
- `src/database/parser.rs`: SQLite schema parsing
- `src/database/server.rs`: HTTP server implementation
- `src/database/mod.rs`: Mod file for connections
- `src/main.rs`: Application entry point

## Dependencies

- `rusqlite`: SQLite database interface
- `actix-web`: HTTP server framework
- `tokio`: Async runtime
- `serde`: Serialization/deserialization

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## License

[Apache 2.0](LICENSE)