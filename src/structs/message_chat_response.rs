use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageChoices{
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessageChoices{
    pub message: MessageChoices,
    pub finish_reason: String,
    pub index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage{
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessage {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<ResponseMessageChoices>,
}