mod render;
mod compute;
mod shared;

#[tokio::main]
async fn main() {
    println!("Hello World!");
    let window = render::window::WindowData::new("Cells").await;
    render::window::run(window);
}
