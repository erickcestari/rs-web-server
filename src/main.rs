use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
};

use pool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();
    let num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let pool = ThreadPool::new(num_threads);

    println!("Listening on port 9999");
    println!("Using {} threads", num_threads);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (_method, path, _protocol) = parse_request_line(&request_line);

    println!("Request for {}", path);

    let path = if path == "/" {
        "pages/index.html".to_string()
    } else {
        if !path.contains(".") {
            format!("pages/{}.html", path)
        } else {
            format!("pages{}", path)
        }
    };

    let (status_line, filename) = if Path::new(&path).exists() {
        ("HTTP/1.1 200 OK", path)
    } else {
        ("HTTP/1.1 404 NOT FOUND", "pages/404.html".to_string())
    };

    let contents = fs::read(&filename).unwrap();
    let length = contents.len();

    let content_type = match Path::new(&filename)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        _ => "application/octet-stream",
    };

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n"
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(&contents).unwrap();
}

fn parse_request_line(request_line: &str) -> (&str, &str, &str) {
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/");
    let protocol = parts.next().unwrap_or("");
    (method, path, protocol)
}
