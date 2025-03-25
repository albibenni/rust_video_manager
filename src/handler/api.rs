use std::io::{BufRead, Write};

pub fn handle_connection(mut stream: std::net::TcpStream) {
    let buf_reader = std::io::BufReader::new(&stream);
    let request_line = buf_reader
        .lines()
        .next()
        .expect("Line not found")
        .expect("Something went wrong");
    //println!("Request: {http_request:#?}");
    // GET
    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /health HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            std::thread::sleep(std::time::Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 400 OK", "404.html"),
    };
    let contents = std::fs::read_to_string(filename).expect("file not found");
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream
        .write_all(response.as_bytes())
        .expect("Something went wrong")
}
