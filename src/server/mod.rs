use tonic::transport::Server;
use crate::service::my_chat::MyChat;
use crate::pb::chat::chat_server::ChatServer;

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let chat_service = MyChat::default();

    println!("Servidor gRPC escuchando en {}", addr);

    Server::builder()
        .add_service(ChatServer::new(chat_service))
        .serve(addr)
        .await?;

    Ok(())
}
