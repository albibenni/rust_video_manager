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
        println!("{:?}", stream);
    }
}
