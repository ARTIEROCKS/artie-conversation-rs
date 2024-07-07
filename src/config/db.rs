use mongodb::{Client, options::ClientOptions};
use std::env;

pub async fn get_mongo_client() -> Result<Client, Box<dyn std::error::Error>> {
    let mongo_uri = env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let client_options = ClientOptions::parse(&mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}