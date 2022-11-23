use salvo::prelude::*;

#[handler]
async fn hello() -> &'static str {
    "Hello, Rust!"
}

#[tokio::main]
async fn main() {
    println!("Hello, Rust!");
    let router = Router::new().get(hello);
    let acceptor = TcpListener::bind("127.0.0.1:8080");
    Server::new(acceptor).serve(router).await;
}