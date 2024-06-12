use tonic::transport::Server;
use crate::service::artie_chat::ArtieChat;
use crate::pb::chat::chat_server::ChatServer;
use tokio::signal;

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let chat_service = ArtieChat::default();

    println!("gRPC server listening in {}", addr);

    Server::builder()
        .add_service(ChatServer::new(chat_service))
        .serve_with_shutdown(addr, shutdown_signal())
        .await?;

    println!("gRPC server stopped");
    Ok(())
}

async fn shutdown_signal() {
    // Capture Ctrl+C
    signal::ctrl_c()
        .await
        .expect("error configuring Ctrl+C");

    println!("Ctrl+C received, stopping the server...");
}
