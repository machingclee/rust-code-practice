use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("{}", buffer.starts_with(get));

    let status_line: &str;
    let filename: &str;

    if buffer.starts_with(get) {
        status_line = "HTTP/1.1 200 OK";
        filename = "hello.html";
    } else if buffer.starts_with(sleep) {
        println!("{}", "sleeping...");
        thread::sleep(Duration::from_secs(5));
        println!("{}", "awake!");
        status_line = "HTTP/1.1 200 OK";
        filename = "hello.html";
    } else {
        status_line = "HTTP/1.1 404 NOT FOUND";
        filename = "404.html";
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        &contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
