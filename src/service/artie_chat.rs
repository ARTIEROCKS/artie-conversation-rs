use mongodb::{Database, bson::{doc, Document, Bson, DateTime}};
use tonic::{Request, Response, Status};
use crate::config::pb::chat_server::Chat;
use crate::config::pb::{ChatRequest, ChatResponse};
use std::env;
use log::{info, error};
use chrono::Utc;

#[derive(Debug)]
pub struct ArtieChat {
    db: Database,
}

#[tonic::async_trait]
impl Chat for ArtieChat {

    async fn get_response(
        &self,
        request: Request<ChatRequest>,
    ) -> Result<Response<ChatResponse>, Status> {

        let ChatRequest { user_id, context_id, message } = request.into_inner();
        info!("Received gRPC request with message: {}", message);

        // Retrieve the context from MongoDB
        let conversation = self.get_conversation(&user_id, &context_id).await.unwrap_or_default();

        // Add the user message to the conversation context
        let mut updated_conversation = conversation.clone();
        updated_conversation.push(("user".to_string(), message.clone()));

        // Get response from ChatGPT
        let reply = match call_chatgpt_api(message.to_string()).await {
            Ok(response) => {

                info!("Received response from ChatGPT API: {}", response);
                updated_conversation.push(("assistant".to_string(), response.clone()));

                 // Update the conversation in MongoDB
                self.update_conversation(&user_id, &context_id, &updated_conversation);
                response
            },
            Err(err) => {
                error!("Error calling ChatGPT API: {}", err);
                "Error calling ChatGPT API".to_string()
            },
        };

        info!("Sending gRPC response with reply: {}", reply);
        Ok(Response::new(ChatResponse { user_id, context_id, reply }))
    }
}

impl ArtieChat{

    pub fn new(db: Database) -> Self {
        ArtieChat { db }
    }

    /**
     * Gets the conversation context for a given user and context id
     */
    async fn get_conversation(&self, user_id: &str, context_id: &str) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        
        let collection = self.db.collection::<Document>("conversations");
        let filter = doc! { "user_id": user_id, "context_id": context_id };

        if let Some(result) = collection.find_one(filter, None).await? {
            let context = result.get_array("context")?
                .iter()
                .map(|doc| {
                    let doc = doc.as_document().unwrap();
                    let role = doc.get_str("role").unwrap().to_string();
                    let message = doc.get_str("message").unwrap().to_string();
                    (role, message)
                })
                .collect();
            Ok(context)
        } else {
            Ok(Vec::new())
        }
    }

    /**
     * Updates the conversation context for a given user and context id
     */
    async fn update_conversation(&self, user_id: &str, context_id: &str, context: &[(String, String)]) -> Result<(), Box<dyn std::error::Error>> {
        
        let collection = self.db.collection::<Document>("conversations");
        let filter = doc! { "user_id": user_id, "context_id": context_id };
        
        let context_docs: Vec<Document> = context.iter()
            .map(|(role, message)| {
                doc! { 
                    "role": role, 
                    "message": message, 
                    "timestamp": Bson::DateTime(DateTime::from_millis(Utc::now().timestamp_millis()))
                }
            })
            .collect();

        let update = doc! {
            "$set": {
                "user_id": user_id,
                "context_id": context_id,
                "context": context_docs,
                "last_updated": Bson::DateTime(DateTime::from_millis(Utc::now().timestamp_millis())),
            }
        };
        collection.update_one(filter, update, mongodb::options::UpdateOptions::builder().upsert(true).build()).await?;
        Ok(())
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
