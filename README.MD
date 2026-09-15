# Mutlithreaded Server

A small HTTP server written in Rust using only the standard library. Mutlithreaded Server
accepts TCP connections, reads HTTP requests, and serves HTML pages with simple
route matching—an approachable project for learning how web servers work.

## Features

- TCP listener at `127.0.0.1:7878`.
- Routes for the home, hello, and about pages.
- A custom HTML response for unmatched requests (`404 Not Found`).
- Responses with `Content-Length` and `Content-Type` headers.
- Connection and request logging in the terminal.
- No external dependencies.

## Getting started

Install Rust and Cargo using [rustup](https://rustup.rs/). This project uses the
Rust 2024 edition, which requires Rust 1.85 or newer.

From the directory containing this project:

```sh
cd multithreaded_server
cargo run
```

Open <http://127.0.0.1:7878> in your browser. Stop the server with `Ctrl+C`.

Run commands from the project directory because HTML files are loaded using
relative paths under `public/`.

## Routes

| Request | HTML file | Status |
| --- | --- | --- |
| `GET / HTTP/1.1` | `public/index.html` | `200 OK` |
| `GET /hello HTTP/1.1` | `public/hello.html` | `200 OK` |
| `GET /about HTTP/1.1` | `public/about.html` | `200 OK` |
| Any unmatched request | `public/404.html` | `404 Not Found` |

Try a request from another terminal while the server is running:

```sh
curl -i http://127.0.0.1:7878/hello
curl -i http://127.0.0.1:7878/missing
```

## Project structure

```text
multithreaded_server/
├── public/
│   ├── index.html
│   ├── hello.html
│   ├── about.html
│   └── 404.html
├── src/
│   └── main.rs
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── README.MD
```

## Development

```sh
cargo check
cargo build --release
```

The release executable is `target/release/mutlithreaded-server` (or `mutlithreaded-server.exe`
on Windows). Keep `Cargo.lock` in version control; build output in `target/`
is ignored.

## Current scope

Despite the original directory name, the server currently handles connections
sequentially on a single thread. It reads each request once into a 1,024-byte
buffer and matches the first line exactly. It is a learning project with limited
HTTP support, and it listens only on the local machine.

Possible next steps include a worker thread pool, more robust request parsing,
and graceful error handling.
