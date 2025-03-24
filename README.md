![Rust](https://img.shields.io/badge/Rust-8F0000?style=for-the-badge&logo=rust&logoColor=white)
![Actix](https://img.shields.io/badge/Actix-7036ab?style=for-the-badge&logo=actix&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2CA5E0?style=for-the-badge&logo=docker&logoColor=white)
![MySQL](https://img.shields.io/badge/MySQL-005C84?style=for-the-badge&logo=mysql&logoColor=white)
![SeaORM](https://img.shields.io/badge/SeaORM-007ACC?style=for-the-badge&logo=seaorm&logoColor=white)
![JWT](https://img.shields.io/badge/JWT-FF4785?style=for-the-badge&logo=JSON%20web%20tokens&logoColor=white)
![RustRover](https://img.shields.io/badge/RustRover-000000.svg?&style=for-the-badge&logo=RustRover&logoColor=white)


# Spenser

Spenser is a RESTful API server built with **Rust**, **Actix Web**, **SeaORM**, **MySQL**, and **Docker**. It provides authentication and transaction management functionalities.

## Requirements
- Rust
- Docker & Docker-Compose
- MySQL

## Environment Variables
The application requires the following environment variables:

```
DATABASE_URL=mysql://<USER>:<PASSWORD>@localhost:3306/<DB_NAME>
MYSQL_URL=mysql://<USER>:<PASSWORD>@db:3306/<DB_NAME>
MYSQL_USER=<USER>
MYSQL_DB=<DB_NAME>
MYSQL_PASSWORD=<PASSWORD>
MYSQL_ROOT_PASSWORD=<ROOT_PASSWORD>
JWT_SECRET=<YOUR_SECRET>
```

## Running with Docker-Compose
```sh
docker-compose up --build
```

## API Endpoints
| Method | Path                               | Auth? | Description                          |
|--------|------------------------------------|------|----------------------------------|
| POST   | `/auth/sign-up`                    | ❌    | Create a new account.               |
| POST   | `/auth/sign-in`                    | ❌    | Sign in, returns JWT on success.    |
| GET    | `/api/categories`                  | ✅    | List categories.                    |
| POST   | `/api/categories`                  | ✅    | Create a new category.              |
| GET    | `/api/categories/{id}`             | ✅    | Get category by ID.                 |
| PUT    | `/api/categories/{id}`             | ✅    | Update category by ID.              |
| DELETE | `/api/categories/{id}`             | ✅    | Delete category by ID.              |
| GET    | `/api/categories/{id}/transactions`| ✅    | List transactions in a category.    |
| GET    | `/api/transactions`                | ✅    | List all transactions.              |
| POST   | `/api/transactions`                | ✅    | Create a new transaction.           |
| GET    | `/api/transactions/{id}`           | ✅    | Get transaction by ID.              |
| PUT    | `/api/transactions/{id}`           | ✅    | Update transaction by ID.           |
| DELETE | `/api/transactions/{id}`           | ✅    | Delete transaction by ID.           |

**Authentication:** Include `Authorization: Bearer <JWT>` in the header for authenticated requests.

## Database
The service uses **MySQL** as the database and runs inside a **Docker container**. Ensure MySQL is configured correctly before running the app.

## Build and Run Manually
```sh
cargo build --release
./target/release/Spenser
```

## Notes
- Ensure `.env` is correctly configured before running.
- Uses **JWT-based authentication** with a token lifetime of **4 hours**.