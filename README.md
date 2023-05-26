# Rust-w-MongoDB
Contains two simple CRUD projects in Rust Lang. One uses a warp dependency and the other does not.

## For testconnection

### Curl command in on Windows CMD

#### Create User
```
curl -X POST http://localhost:3030/ --data-raw "{\"name\": \"Test User\", \"age\": 32, \"email\": \"email@example.com\", \"characteristics\": [\"depressed\", \"dead inside\"]}" -H "Content-Type: application/json"
```
