
# Axum SQLX REST API Template

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen?logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/axum-0.6-blue)](https://github.com/tokio-rs/axum)
[![SQLx](https://img.shields.io/badge/sqlx-0.6-orange)](https://github.com/launchbadge/sqlx)

A production-ready template for building robust, type-safe, and asynchronous REST APIs in Rust using the Axum framework and SQLx for PostgreSQL.

## ✨ Features

- **🚀 Async by Default**: Built on [Tokio](https://tokio.rs/) and [Axum](https://github.com/tokio-rs/axum) for high-performance asynchronous handling.
- **📝 Structured Logging**: Integrated [tracing](https://docs.rs/tracing) for insightful, structured application logs.
- **🗄️ Database Ready**: Pre-configured with [SQLx](https://github.com/launchbadge/sqlx) for compile-time checked queries against PostgreSQL.
- **🔄 (De)serialization**: Seamless JSON handling with [serde](https://docs.rs/serde) and `axum::Json`.
- **🎯 Error Handling**: Elegant error handling by using [anyhow](https://docs.rs/anyhow) and [thiserror](https://docs.rs/thiserror) libraries.
- **🔧 Configuration Management**: Environment-based configuration using `dotenvy` (via `dotenv`).
- **🧪 Health Check**: A ready-to-use endpoint to verify API and database status.

## 🚀 Getting Started

### Prerequisites

- **Rust Toolchain**: Install the latest stable Rust via [rustup](https://rustup.rs/).
- **PostgreSQL**: A running PostgreSQL instance. You can use Docker for a quick setup:
    ```bash
    docker run --name axum-postgres -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres
    ```

### Installation & Setup

1.  **Clone the repository**
    ```bash
    git clone https://github.com/your-username/axum-sqlx-template.git
    cd axum-sqlx-template
    ```

2.  **Setup Environment Variables**
    ```bash
    # Copy the example environment file
    cp .env.example .env
    ```
    Edit `.env` and set your database connection string and other variables:
    ```env
    DATABASE_URL=postgres://postgres:mysecretpassword@localhost:5432/postgres
    HOST=127.0.0.1
    PORT=3000
    ```
3.  **Run the Application**
    ```bash
    cargo run
    ```
    The API server will start on `http://127.0.0.1:3000`.

### Usage

- **Health Check**: `GET http://localhost:3000/health`
    - Verifies the API is running and can connect to the database.

## 🛣️ Roadmap & Ideas for Contribution

This template is a starting point. Here are some ideas for how you could extend it:
- [ ] Add example CRUD endpoints for a resource (e.g., `User` or `Post`).
- [ ] Integrate a testing suite (unit, integration).
- [ ] Add Dockerfile and docker-compose setup.
- [ ] Implement request rate limiting.
- [ ] Add OpenAPI/Swagger documentation.

## 🤝 Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have an idea to improve this template, please first open an **Issue** to discuss it. This helps ensure we're aligned before you write any code.

### How to Contribute

1.  **Fork** the Project
2.  **Create your Feature Branch** (`git checkout -b feature/AmazingFeature`)
3.  **Commit your Changes** (`git commit -m 'Add some AmazingFeature'`)
4.  **Push to the Branch** (`git push origin feature/AmazingFeature`)
5.  **Open a Pull Request**

Please ensure your PR is focused on a single change and includes a clear description.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/frankPairs/axum-sqlx-template/blob/main/LICENSE) file for details.

## 🙏 Acknowledgments

- The [Tokio](https://github.com/tokio-rs), [Axum](https://github.com/tokio-rs/axum), and [SQLx](https://github.com/launchbadge/sqlx) teams for the amazing async ecosystem.
