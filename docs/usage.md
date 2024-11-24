# Running STRADA locally

```bash
# With default database path
cargo run

# With custom database path
export DATABASE_PATH=/path/to/database.db && cargo run
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