use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use mtr_server_app::ThreadPool;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });

        // println!("Connection established!");
        // handle_connection(stream);
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    // if buffer.starts_with(get) {
    //     let contents = fs::read_to_string("hello_world.html").unwrap();
    //     let resp = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);

    //     stream.write(resp.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }
    // else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND";           
    // }

    // let resp = "HTTP/1.1 200 OK\r\n\r\n";

    let (status_name, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello_world.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    println!("{}", filename);
    let contents = fs::read_to_string(filename).unwrap();

    let resp = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_name, contents.len(), contents);
    
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();   
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}