mod config;
mod server;
mod service;

use log::info;
use std::env;
use std::io::Write; // Import the Write trait

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            writeln!(
                buf,
                "{{\"level\":\"{}\",\"message\":\"{}\"}}",
                record.level(),
                record.args()
            )
        })
        .init();

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    info!("Starting gRPC server...");

    server::start_server().await
}
