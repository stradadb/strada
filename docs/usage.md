# Running STRADA locally

```bash
# With default database path
cargo run

# With custom database path
export STRADA_DATABASE_PATH=/path/to/database.db && cargo run
```

### API Endpoints

#### Get Database Schema
```bash
curl http://localhost:8080/schema
```

#### Execute SQL Query
```bash
# Create a table
curl -X POST http://localhost:8080/query \
     -H "Content-Type: application/json" \
     -d '"CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)"'

# Insert data
curl -X POST http://localhost:8080/query \
     -H "Content-Type: application/json" \
     -d '"INSERT INTO users (name) VALUES (\'John Doe\')"'
```

#### Get Protected Resource Access a protected route using the token received during login:

```bash
curl http://localhost:8080/protected \
     -H "Authorization: Bearer YOUR_TOKEN_HERE"
```

#### Error Handling

If a query execution fails or if authentication fails, the server will respond with appropriate error messages. For example, if a query fails, you might receive a response like:

```json
{
    "error": "Query execution failed: [error details]"
}
```