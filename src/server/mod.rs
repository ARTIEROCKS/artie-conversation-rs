use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflectionBuilder;
use crate::service::artie_chat::ArtieChat;
use crate::config::pb::chat_server::ChatServer;
use crate::config::db::get_mongo_client;
use tokio::signal;

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();

    let mongo_client = get_mongo_client().await?;
    let db = mongo_client.database("artie");
    let chat_service = ArtieChat::new(db);

    println!("gRPC server listening on {}", addr);

    // Agrega el servicio de reflexión al servidor
    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(crate::config::pb::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(ChatServer::new(chat_service))
        .add_service(reflection_service)
        .serve_with_shutdown(addr, shutdown_signal())
        .await?;

    println!("gRPC server stopped");
    Ok(())
}

async fn shutdown_signal() {
    // Captura Ctrl+C
    signal::ctrl_c()
        .await
        .expect("error configuring Ctrl+C");

    println!("Ctrl+C received, stopping the server...");
}
