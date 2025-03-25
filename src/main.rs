use std::io::{BufRead, Write};

fn main() {
    dotenv::dotenv().ok();
    let key: &str = "HOST";

    println!("Hello, world!");

    let port = match std::env::var(key) {
        Ok(val) => val,
        Err(e) => {
            println!("Error {}: {}", key, e);
            "localhost:8081".to_string()
        }
    };
    let listener = std::net::TcpListener::bind(port).expect("Something went wrong");
    for stream in listener.incoming() {
        println!("Connection established!");
        handle_connection(stream.expect("Something went wrong"));
        //println!("{:?}", stream);
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let buf_reader = std::io::BufReader::new(&stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.expect("nope"))
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    let request_line = buf_reader
        .lines()
        .next()
        .expect("Line not found")
        .expect("Something went wrong");
    //println!("Request: {http_request:#?}");
    // GET
    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = std::fs::read_to_string("index.html").expect("file not found");
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream
            .write_all(response.as_bytes())
            .expect("Something went wrong")
    } else {
    }
}
