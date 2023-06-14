# Rust REST API Example

This repository contains an example Rust REST API implementation using the Actix web framework. The code demonstrates how to build a simple CRUD (Create, Read, Update, Delete) API for managing users.

## Requirements

- Rust (nightly)
- Cargo

## Setup

1. Clone the repository:

```bash
git clone https://github.com/TechSavvyScribe/rust-rest-api.git
```

2. Navigate to the project directory:

```bash
cd rust-rest-api
```

3. Build and run the project:

```bash
cargo run
```

4. Access the API endpoints using your preferred HTTP client. The base URL is `http://127.0.0.1:8000/api/users`.

## API Endpoints

The API provides the following endpoints:

- `GET /api/users`: Retrieve a list of all users.
- `GET /api/users/{id}`: Retrieve a specific user by ID.
- `POST /api/users`: Create a new user.
- `PUT /api/users/{id}`: Update an existing user by ID.
- `DELETE /api/users/{id}`: Delete a user by ID.

## Example Usage

Assuming the server is running locally, you can use cURL to interact with the API.

1. Retrieve all users:

```bash
curl -i http://localhost:8000/api/users
```

2. Retrieve a specific user by ID:

```bash
curl -i http://localhost:8000/api/users/{id}
```

3. Create a new user:

```bash
curl -i -X POST -H "Content-Type: application/json" -d '{"name":"John Doe","email":"john@example.com"}' http://localhost:8000/api/users
```

4. Update an existing user by ID:

```bash
curl -i -X PUT -H "Content-Type: application/json" -d '{"name":"Updated Name","email":"updated@example.com"}' http://localhost:8000/api/users/{id}
```

5. Delete a user by ID:

```bash
curl -i -X DELETE http://localhost:8000/api/users/{id}
```

## Contributing

Contributions are welcome! If you have any suggestions or improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).
