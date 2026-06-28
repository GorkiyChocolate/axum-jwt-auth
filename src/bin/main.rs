use auth::{App, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("{:?}", std::env::current_dir()?);
    App::run().await
}