<div align="center">
  <img src="./assets/logo.svg" alt="Logo" />
</div>
<h1 align="center">STRADA: SQLite Database Server</h1>
<div align="center">
  <img src="https://img.shields.io/badge/build-passing-green" alt="Build Status" />
  <img src="https://img.shields.io/badge/license-Apache%202.0-blue" alt="License" />
  <img src="https://img.shields.io/badge/rust-1.65%2B-orange" alt="Rust Version" />
</div>

<div align="center">

**STRADA** is a lightweight, Rust-based SQLite database server that exposes REST API endpoints for interacting with SQLite databases.

</div>

---

## Features

- **SQLite Database Connection**: Provides an easy-to-use interface for interacting with SQLite databases.
- **Schema Parsing**: Automatic parsing of database schema for seamless interactions.
- **RESTful Query Execution**: Exposes RESTful API endpoints for querying and modifying the database.
- **Authentication**: Secure authentication mechanism for API usage.
- **Async Server**: Fully asynchronous server for high-performance operations.

---

## Prerequisites

Ensure you have the following installed:

- **Rust** (1.65+ / 2021 Edition recommended)
- **Cargo** (comes with Rust installation)
- **SQLite3** (for SQLite operations)

---

## Installation

### Clone the Repository

```bash
git clone https://github.com/stradadb/strada.git
cd strada
```

### Build the Project

```bash
cargo build --release
```

---

## Configuration

### Environment Variables

- **`STRADA_DATABASE_PATH`**: Path to your SQLite database file  
  Default: `./strada.db`

Example:
```bash
STRADA_DATABASE_PATH="/path/to/your/database.db"
```

---

## Development

### Running Tests

To run tests for the project, use the following command:

```bash
cargo test
```

### Building the Project

- **Development Build**:
  ```bash
  cargo build
  ```

- **Release Build**:
  ```bash
  cargo build --release
  ```

---

## Dependencies

- `rusqlite`: SQLite database interface for Rust
- `actix-web`: HTTP server framework for building fast and robust web services
- `tokio`: Asynchronous runtime for efficient execution
- `serde`: Serialization and deserialization framework for Rust

---

## Contributing

We welcome contributions to **STRADA**! To contribute:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature-branch`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature-branch`)
5. Open a Pull Request with a detailed description of your changes

---

## License

This project is licensed under the [Apache 2.0 License](LICENSE).

---

Explore a simple demo of STRADA in action:

- **Create a database**
- **Execute queries via REST API**
- **Inspect schema dynamically**