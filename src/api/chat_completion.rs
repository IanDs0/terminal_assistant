// use std::vec;

use reqwest::Error;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

use serde::{Deserialize, Serialize};

use crate::enums;
use enums::model_chat_completion::Model;
// use enums::role_message::RoleMessage;
// use enums::role_message::parse_role;

use crate::structs;
use structs::message_chat_completion::Message;
// use structs::message_chat_completion::format_messages;
use structs::user_chat::ChatUser;


#[derive(Debug, Serialize, Deserialize)]
struct MessageChoices{
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseMessageChoices{
    message: MessageChoices,
    finish_reason: String,
    index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Usage{
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseMessage {
    id: String,
    object: String,
    created: u32,
    model: String,
    usage: Usage,
    choices: Vec<ResponseMessageChoices>,
}


//meio q o main
#[derive(Debug)]
pub struct ChatCompletion{
    pub user: ChatUser,//global user nao pode ser null
    pub model: Model,//modelo a ser utilizado
    pub messages: Vec<Message>,//chat messages
    pub temperature: f32,//positivo entre 0 e 2
    pub top_p: f32,//positivo entre 0 e 1
    pub n: u64,//numero de respostas por entrada
    pub max_tokens: u32,//maximo de tokens que podem ser utilizados
    
    // A ser implementado
    /*
    // stream: bool,//stream de respostas 
    // stop: String,//
    // presence_penalty: f32,//opcional
    // frequency_penalty: f32,//opcional
    // logit_bias: map,//opcional nao sei colocar
    // prompt: String,// erro
    */
}

impl ChatCompletion{
    pub fn new(
            user: ChatUser,
            model: Model,
            messages: Vec<Message>, 
            temperature: f32,
            top_p: f32,
            n: u64,
            max_tokens: u32,
            ) -> Self{

        ChatCompletion{
            user: user,
            model: model,
            messages: messages,

            temperature: match temperature {
                temperature if (temperature < 2.0)||(temperature >= 0.0) => temperature,
                _ => panic!("Error: temperature must be between 0 and 2"),
            },
            
            top_p: match top_p {
                top_p if (top_p < 1.0)||(top_p >= 0.0) => top_p,
                _ => panic!("Error: top_p must be between 0 and 1"),
            },
            n: n,
            max_tokens: max_tokens,
        }
    }

    pub fn get_api(&self)-> Result<(), Error>{
        let client = reqwest::blocking::Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.user.token)).unwrap(),
        );

        let body = serde_json::json!({
            "model": self.model.parse_model(),
            "messages": serde_json::json!(self.messages),
            "user": self.user.user_name,
            // "max_tokens": self.max_tokens,
            // "temperature": self.temperature,
            // "top_p": self.top_p,
            // "n": self.n,
            // "stream": false,
            // "logprobs": null,
            // "stop": "\n"
        });

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .headers(headers)
            .body(body.to_string())
            .send();

        let response: ResponseMessage = match serde_json::from_str(response?.text()?.as_str()){
            Ok(response) => response,
            Err(e) => panic!("Error: {}", e),
        };
        println!("{:#}", response.choices[0].message.content);
    
        Ok(())
    }
}