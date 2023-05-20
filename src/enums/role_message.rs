use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum RoleMessage {
    assistant,
    user,
    system,
}

pub fn parse_role(role: &RoleMessage) -> String {
    match role {
        RoleMessage::assistant => String::from("assistant"),
        RoleMessage::user => String::from("user"),
        RoleMessage::system => String::from("system"),
    }
}