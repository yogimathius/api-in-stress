async fn hello() {
    println!("Hello, world!");
}

#[tokio::main]
async fn main() {
    hello().await;
}