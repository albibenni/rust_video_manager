use handler::api::handle_connection;

mod handler;
mod utils;

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
    let pool = utils::pool::ThreadPool::new(4);
    let listener = std::net::TcpListener::bind(port).expect("Something went wrong");
    for stream in listener.incoming() {
        println!("Connection established!");
        // infinite thread
        // std::thread::spawn(|| {
        //     handler::api::handle_connection(stream.expect("Something went wrong"));
        // });
        pool.execute(|| handle_connection(stream.expect("ee")));
    }
}
