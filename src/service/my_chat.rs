use tonic::{Request, Response, Status};
use crate::pb::chat::chat_server::Chat;
use crate::pb::chat::{ChatRequest, ChatResponse};
use std::env;

#[derive(Debug, Default)]
pub struct MyChat {}

#[tonic::async_trait]
impl Chat for MyChat {
    async fn get_response(
        &self,
        request: Request<ChatRequest>,
    ) -> Result<Response<ChatResponse>, Status> {
        let message = request.into_inner().message;

        let reply = match call_chatgpt_api(message).await {
            Ok(response) => response,
            Err(_) => "Error al llamar a la API de ChatGPT".to_string(),
        };

        Ok(Response::new(ChatResponse { reply }))
    }
}

async fn call_chatgpt_api(message: String) -> Result<String, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key = env::var("API_KEY")?;

    let client = reqwest::Client::new();

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-4",
            "messages": [{"role": "user", "content": message}]
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let reply = res["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
    Ok(reply)
}