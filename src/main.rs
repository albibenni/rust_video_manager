use std::io::BufRead;

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
    }
}

fn handle_connection(stream: std::net::TcpStream) {
    let buf_reader = std::io::BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.expect("nope"))
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {http_request:#?}");
}
