use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    TcpListener::bind("127.0.0.1:8000").map(run)??.await
}
