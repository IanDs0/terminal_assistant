use crate::enums::role_message::RoleMessage;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "role")]
    role: RoleMessage,
    
    #[serde(rename = "content")]
    content: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    name: Option<String>,
}

impl Message {
    pub fn new(
            role: RoleMessage,
            content: String,
            name:Option<String>,
            ) -> Self{

        Message{
            role: role,
            content: content,
            name: name,
        }
    }
}