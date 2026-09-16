mod thread_pool;

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};
use std::thread;
use std::time::Duration;
use crate::thread_pool::ThreadPool;
use std::sync::{Arc, Mutex};
//127.0.0.1 -> loopback address means the computer inself

struct ServerStats {
    total_requests: usize,
    successful_requests: usize,
    not_found_requests: usize,
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");
    println!("Server running on http://127.0.0.1:7878");

    let pool = ThreadPool::new(4);

    let stats = Arc::new(Mutex::new(ServerStats {
            total_requests: 0,
            successful_requests: 0,
            not_found_requests: 0,
        }));

    // To accept one request
    // match listener.accept() {
    //     Ok((stream, address)) => {
    //         println!("New connection from {address}");
    //     }

    //     Err(error) => {
    //         eprintln!("Connection failed: {error}");
    //     }
    // }

    // For multiple request
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let stats = Arc::clone(&stats);

                pool.execute(move || {
                    handle_connection(stream, stats);
                });
            }

            Err(error) => {
                eprintln!("Connection failed: {}", error);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, stats: Arc<Mutex<ServerStats>>) {
    // read request
    // find route
    // read html
    // create response
    // write response

    {
        let mut stats = stats.lock().unwrap();
        stats.total_requests += 1;
    }

    println!("New connection from {:?}", stream.peer_addr());

    let mut buffer = [0; 1024];

    let bytes_read = stream.read(&mut buffer).expect("Failed to read request");

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Request:\n{}", request);

    let first_line = request.lines().next().unwrap_or("");

    let (status_line, filename, is_success) = match first_line {
        "GET / HTTP/1.1" => {
            thread::sleep(Duration::from_secs(20));
            ("HTTP/1.1 200 OK", "public/index.html", true)
        }

        "GET /hello HTTP/1.1" => {
            thread::sleep(Duration::from_secs(20));
            ("HTTP/1.1 200 OK", "public/hello.html", true)
        }

        "GET /about HTTP/1.1" => {
            thread::sleep(Duration::from_secs(20));
            ("HTTP/1.1 200 OK", "public/about.html", true)
        }
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(20));
            ("HTTP/1.1 200 OK", "public/sleep.html", true)
        }

        _ => ("HTTP/1.1 404 Not Found", "public/404.html", false),
    };

    {
        let mut stats = stats.lock().unwrap();

        if is_success {
            stats.successful_requests += 1;
        } else {
            stats.not_found_requests += 1;
        }

        println!("", );

        println!(
            "Stats -> Total: {}, Success: {}, 404: {}",
            stats.total_requests,
            stats.successful_requests,
            stats.not_found_requests
        );
    }

    // println!("{first_line} -> {status_line}");

    let body = fs::read_to_string(filename).expect("Failed to read HTML file");

    // Proper HTTP response
    let response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        status_line,
        body.len(),
        body
    );

    stream
        .write_all(response.as_bytes())
        .expect("Failed to write response");
}

/* pusedo code

1. create a tcp connection
2. check if it is connected or not
3. then read through the tcpStream the request from the browser and print it
4. create a http response
5. sends through the tcpStream the response to the browser
6. when we reading the tcp req from the browswer take out status line and match for routing
7. Route according to the need

*/
