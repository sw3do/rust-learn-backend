# Rust Learn Backend

[![Rust](https://img.shields.io/badge/Rust-2024-blue?logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-Web_Framework-green?logo=rust)](https://github.com/tokio-rs/axum)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## 🚀 Overview

**Rust Learn Backend** is a modern, fast, and minimal backend API built with [Axum](https://github.com/tokio-rs/axum) and Rust 2024 edition. It is designed for learning, experimenting, and rapid prototyping of backend features with clean code and best practices.

---

## ✨ Features

- ⚡ **Blazing Fast**: Powered by Rust and Axum
- 🧩 **Modular Routing**: Organized route structure for easy expansion
- 🔒 **Type Safety**: Strong typing with Rust and Serde
- 🧪 **Ready for Experimentation**: Perfect for learning and prototyping
- 🌐 **CORS Enabled**: Open for frontend integration

---

## 📦 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (2021/2024 edition)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/)

### Installation

```bash
# Clone the repository
$ git clone https://github.com/sw3do/rust-learn-backend.git
$ cd rust-learn-backend

# Install dependencies
$ cargo build

# Copy and edit .env if needed
$ cp .env.example .env
```

### Running the Server

```bash
$ cargo run
```

The server will start on the port specified in your `.env` file.

---

## 🛣️ Example Endpoints

- `GET /text/` — Home handshake
- `POST /text/ascii` — Converts JSON `{ "text": "yourtext" }` to ASCII codes (space-separated)

---

## 🗂️ Project Structure

```
├── src/
│   ├── main.rs
│   └── routes/
│       ├── home.rs
│       └── text/
│           ├── ascii.rs
│           └── mod.rs
├── Cargo.toml
├── .env.example
└── README.md
```

---

## 🤝 Contributing

Pull requests and issues are welcome! For major changes, please open an issue first to discuss what you would like to change.

---

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## 🙏 Credits

Made with ❤️ by [sw3do](https://github.com/sw3do) 