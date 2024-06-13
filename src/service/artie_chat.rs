use tonic::{Request, Response, Status};
use crate::pb::chat::chat_server::Chat;
use crate::pb::chat::{ChatRequest, ChatResponse};
use std::env;
use log::{info, error};

#[derive(Debug, Default)]
pub struct ArtieChat {}

#[tonic::async_trait]
impl Chat for ArtieChat {
    async fn get_response(
        &self,
        request: Request<ChatRequest>,
    ) -> Result<Response<ChatResponse>, Status> {
        let message = request.into_inner().message;
        info!("Received gRPC request with message: {}", message);

        let reply = match call_chatgpt_api(message).await {
            Ok(response) => {
                info!("Received response from ChatGPT API: {}", response);
                response
            },
            Err(err) => {
                error!("Error calling ChatGPT API: {}", err);
                "Error calling ChatGPT API".to_string()
            },
        };

        info!("Sending gRPC response with reply: {}", reply);
        Ok(Response::new(ChatResponse { reply }))
    }
}

async fn call_chatgpt_api(message: String) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("API_KEY")?;
    let client = reqwest::Client::new();

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-3.5-turbo",
            "messages": [{"role": "user", "content": message}]
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let reply = res["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();
    Ok(reply)
}
