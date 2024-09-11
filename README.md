# Distributed File Storage
This project implements a distributed file storage system using Rust, Axum, and PostgreSQL. It allows users to upload, download, and retrieve metadata for files stored across a distributed system.


## Features

- File upload with automatic chunking
- File download with chunk reassembly
- File metadata retrieval
- Distributed storage using PostgreSQL

## Technologies Used

- Rust
- Axum (Web framework)
- SQLx (Database ORM)
- PostgreSQL (Database)
- Docker (Containerization)

## Getting Started
1. Clone the repository:
```
git clone https://github.com/aagbotemi/distributed-file-storage.git
cd distributed-file-storage
```

2. Build and start the Docker containers:
```
docker-compose up --build
```
This command will build the Rust application, set up the PostgreSQL database, and start both services.

3. The application should now be running and accessible at `http://localhost:8081`.

## API Endpoints

1. `POST /upload`: Upload a file
2. `GET /file/{id}`: Get metadata for a specific file
3. `GET /download/{id}`: Download a specific file

## Development
To run the application locally for development:

1. Install Rust and PostgreSQL on your local machine.
2. Set up the required environment variables.
3. Run the application:
    ```
    cargo run
    ```