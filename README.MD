# Mutlithreaded Server

A small HTTP server written in Rust using only the standard library. Mutlithreaded Server
accepts TCP connections, distributes them to a custom worker thread pool, and
serves HTML pages with simple route matching. It demonstrates concurrent request
handling, shared state, and thread pool shutdown.

## Features

- TCP listener at `127.0.0.1:7878`.
- A custom thread pool with four workers and an `mpsc` job queue.
- Routes for the home, hello, about, and sleep pages.
- A custom HTML response for unmatched requests (`404 Not Found`).
- Responses with `Content-Length` and `Content-Type` headers.
- Connection and request logging in the terminal.
- Shared counters for total requests, successful route matches, and 404s,
  protected by `Arc<Mutex<ServerStats>>` and printed in the terminal.
- Worker shutdown and thread joining when the pool is dropped.
- No external dependencies.

## Getting started

Install Rust and Cargo using [rustup](https://rustup.rs/). This project uses the
Rust 2024 edition, which requires Rust 1.85 or newer.

From the directory containing this project:

```sh
cd multithreaded_server
cargo run
```

Open <http://127.0.0.1:7878> in your browser. Each recognized route currently
waits 20 seconds before responding to demonstrate concurrent work.

The listener uses `listener.incoming().take(2)` as an example to demonstrate
shutdown after a limited number of connections. **The value `2` is just an
example**, independent of the worker count: change it to another positive number
or remove `.take(2)` to accept connections continuously. With the example limit,
the server waits for queued work to finish and shuts down after two connections.
Browsers may open extra connections, so use the `curl` examples below for a more
predictable demonstration. Restart with `cargo run` for another run. You can
interrupt the process with `Ctrl+C`, but that does not run the pool's normal
shutdown sequence.

Run commands from the project directory because HTML files are loaded using
relative paths under `public/`.

## Routes

| Request | HTML file | Status |
| --- | --- | --- |
| `GET / HTTP/1.1` | `public/index.html` | `200 OK` |
| `GET /hello HTTP/1.1` | `public/hello.html` | `200 OK` |
| `GET /about HTTP/1.1` | `public/about.html` | `200 OK` |
| `GET /sleep HTTP/1.1` | `public/sleep.html` | `200 OK` |
| Any unmatched request | `public/404.html` | `404 Not Found` |

All four recognized routes have a 20-second delay. Unmatched requests have no
intentional delay.

Try a request from another terminal while the server is running:

```sh
curl -i http://127.0.0.1:7878/hello
curl -i http://127.0.0.1:7878/missing
```

### Demonstrate concurrent requests

Start a fresh server, then run these commands in a Bash terminal:

```sh
curl -i http://127.0.0.1:7878/sleep &
curl -i http://127.0.0.1:7878/hello &
wait
```

Two workers handle these requests concurrently, so both should finish in about
20 seconds overall. With the example `.take(2)` limit, the server then joins its
workers and exits. With a larger limit or no limit, it continues accepting
connections until the configured limit is reached or the process is stopped.

## How the thread pool works

1. `ThreadPool::new(4)` creates four workers sharing an `mpsc` receiver through
   `Arc<Mutex<_>>`. The pool size must be greater than zero.
2. The listener submits each connection handler as a boxed `FnOnce` job using
   `ThreadPool::execute`.
3. A worker locks the receiver to obtain a job, releases the lock, and executes
   the job. Other workers can receive and run jobs while that handler is busy.
4. Handlers briefly lock the shared statistics to update and print counters.
   Success and 404 counters reflect route selection, not confirmed delivery.
5. When the pool is dropped, it drops its sender. Workers finish queued jobs,
   exit when the channel disconnects, and are joined before shutdown completes.

## Project structure

```text
multithreaded_server/
├── public/
│   ├── index.html
│   ├── hello.html
│   ├── about.html
│   ├── sleep.html
│   └── 404.html
├── src/
│   ├── main.rs
│   └── thread_pool.rs
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

The server reads each request once into a 1,024-byte buffer and matches the first
line exactly. It is a learning project with limited HTTP support and listens only
on the local machine. Several I/O and synchronization operations use `unwrap`
or `expect`, so failures can panic.

Configure the listener loop in `src/main.rs` for the desired behavior:

```rust
// Example: accept two connections, then finish queued work and shut down.
for stream in listener.incoming().take(2) {
    // Existing connection handling goes here.
}

// Alternatively, accept connections continuously.
for stream in listener.incoming() {
    // Existing connection handling goes here.
}
```

Use one of these loop forms. `.take(n)` requires a connection count; an empty
`.take()` is invalid. Adjust `ThreadPool::new(4)` separately to change the worker
count, and change or remove the `thread::sleep` calls to adjust the demonstration
delays.

Possible next steps include more robust request parsing, graceful error handling,
and signal-driven shutdown.
