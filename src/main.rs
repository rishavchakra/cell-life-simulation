#[tokio::main]
async fn main() {
    println!("Hello, world!");
    cells::run().await;
}
