use std::vec;

use reqwest::Error;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

use serde::{Deserialize, Serialize};

use crate::enums;
use enums::model_chat_completion::Model;
use enums::role_message::RoleMessage;
use enums::role_message::parse_role;

use crate::structs;
use structs::message_chat_completion::Message;
// use structs::message_chat_completion::format_messages;
use structs::user_chat::ChatUser;

/*
//user
//
struct ChatUser{
    pub user_name: String,
    pub token: String,
}
//
impl ChatUser{
    fn new(
            user_name: String, 
            token: String
            ) -> Self{

        ChatUser{
            user_name: user_name,
            token: token,
        }
    }
}
*/

/*
//message
//foi
#[derive(Debug, Serialize, Deserialize)]
enum RoleMessage {
    assistant,
    user,
    system,
}
//temporario
#[derive(Debug, Serialize, Deserialize)]
struct Message{
    role: RoleMessage,
    content: String,
    name:String,
}
 */

/*
//model
//foi
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
enum Model {
    gpt_4, 
    gpt_4_0314,
    gpt_4_32k, 
    gpt_4_32k_0314, 
    gpt_35_turbo, 
    gpt_35_turbo_0301
}
//foi
impl Model {
    
    fn parse_model(&self) -> String {
        
        match self {
            Model::gpt_4 => String::from("gpt-4"),
            Model::gpt_4_0314 => String::from("gpt-4-0314"),
            Model::gpt_4_32k => String::from("gpt-4-32k"),
            Model::gpt_4_32k_0314 => String::from("gpt-4-32k-0314"),
            Model::gpt_35_turbo => String::from("gpt-3.5-turbo"),
            Model::gpt_35_turbo_0301 => String::from("gpt-3.5-turbo-0301"),
        }
        
        // match s {
        //     "gpt-4" => Some(Model::gpt_4),
        //     "gpt-4-0314" => Some(Model::gpt_4_0314),
        //     "gpt-4-32k" => Some(Model::gpt_4_32k),
        //     "gpt-4-32k-0314" => Some(Model::gpt_4_32k_0314),
        //     "gpt-3.5-turbo" => Some(Model::gpt_35_turbo),
        //     "gpt-3.5-turbo-0301" => Some(Model::gpt_35_turbo_0301),
        //     _ => None,
        // }
    }
}
*/



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

    // /*
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

        let mut json = serde_json::json!({
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
            .body(json.to_string())
            .send();
        let teste: ResponseMessage = match serde_json::from_str(response?.text()?.as_str()){
            Ok(teste) => teste,
            Err(e) => panic!("Error: {}", e),
        };
        println!("{:#}", teste.choices[0].message.content);
    
        Ok(())
    }
    // */
}