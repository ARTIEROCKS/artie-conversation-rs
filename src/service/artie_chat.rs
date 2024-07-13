use mongodb::{Database, bson::{doc, Document, Bson, DateTime}};
use tonic::{Request, Response, Status};
use crate::config::pb::chat_server::Chat;
use crate::config::pb::{ChatRequest, ChatResponse};
use std::env;
use log::{info, error};
use chrono::Utc;
use crate::config::error::ArtieError;

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
        let ChatRequest { user_id, context_id, message, prompt } = request.into_inner();
        info!("Received gRPC request with message: {}", message);

        let conversation = self.get_conversation(&user_id, &context_id).await.unwrap_or_default();
        let mut updated_conversation = conversation.clone();

        // If the conversation is empty, add the prompt as the first message
        if updated_conversation.len() == 0 {
            updated_conversation.push(("user".to_string(), prompt.clone()));
        }else{
            updated_conversation.push(("user".to_string(), message.clone()));
        } 

        let reply = match call_chatgpt_api(&updated_conversation).await {
            Ok(response) => {
                info!("Received response from ChatGPT API: {}", response);
                updated_conversation.push(("assistant".to_string(), response.clone()));

                if let Err(err) = self.update_conversation(&user_id, &context_id, &updated_conversation, conversation.len() == 0).await {
                    error!("Error updating conversation in MongoDB: {}", err);
                }

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

impl ArtieChat {
    pub fn new(db: Database) -> Self {
        ArtieChat { db }
    }

    async fn get_conversation(&self, user_id: &str, context_id: &str) -> Result<Vec<(String, String)>, ArtieError> {
        let collection = self.db.collection::<Document>("conversations");
        let filter = doc! { "user_id": user_id, "context_id": context_id };

        if let Some(result) = collection.find_one(filter).await? {
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

    async fn update_conversation(&self, user_id: &str, context_id: &str, context: &[(String, String)], create_new: bool) -> Result<(), ArtieError> {
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

        if create_new {
            collection.insert_one(
                doc! {
                    "user_id": user_id,
                    "context_id": context_id,
                    "context": context_docs,
                    "last_updated": Bson::DateTime(DateTime::from_millis(Utc::now().timestamp_millis())),
                }
            ).await?;
            return Ok(());
        }

        let update = doc! {
            "$set": {
                "user_id": user_id,
                "context_id": context_id,
                "context": context_docs,
                "last_updated": Bson::DateTime(DateTime::from_millis(Utc::now().timestamp_millis())),
            }
        };

        collection.update_one(filter, update).await?;
        Ok(())
    }
}


async fn call_chatgpt_api(messages: &Vec<(String, String)>) -> Result<String, ArtieError> {
    
    let api_key = env::var("API_KEY")?;
    let llm_model = env::var("LLM_MODEL")?;
    let client = reqwest::Client::new();

    // Format the messages in the required format by the OpenAI API
    let formatted_messages: Vec<_> = messages.into_iter()
        .map(|(role, content)| {
            serde_json::json!({
                "role": role,
                "content": content
            })
        })
        .collect();

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": llm_model,
            "messages": formatted_messages
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
