use reqwest::Error;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

use crate::enums;
use enums::model_chat_completion::Model;

use crate::structs;
use structs::message_chat_completion::Message;
use structs::user_chat::ChatUser;
use structs::message_chat_response::ResponseMessage;

#[derive(Debug)]
pub struct ChatCompletion{
    pub user: ChatUser,//global user nao pode ser null
    pub model: Model,//modelo a ser utilizado
    pub messages: Vec<Message>,//chat messages
    pub temperature: Option<f32>,//positivo entre 0 e 2
    pub top_p: Option<f32>,//positivo entre 0 e 1
    pub n: Option<u64>,//numero de respostas por entrada
    pub max_tokens: Option<u32>,//maximo de tokens que podem ser utilizados
    pub stream: Option<bool>,//stream de respostas
    
    pub stop: Option<String>,

    pub presence_penalty: Option<f32>,//opcional
    pub frequency_penalty: Option<f32>,//opcional
    // pub logit_bias: map,//opcional nao sei colocar
    // A ser implementado
    /*
    */
}

impl ChatCompletion{
   pub fn new(
            user: ChatUser,
            model: Model,
            messages: Vec<Message>, 
            temperature: Option<f32>,
            top_p: Option<f32>,
            n: Option<u64>,
            max_tokens: Option<u32>,
            stream: Option<bool>,

            stop: Option<String>,

            presence_penalty: Option<f32>,
            frequency_penalty: Option<f32>
            ) -> Self{

        ChatCompletion{
            user: user,
            model: model,
            messages: messages,
            
            temperature: match temperature{
                Some(temperature)if (temperature <= 2.0)||(temperature >= 0.0) => Some(temperature),
                None => Some(1.0),
                _ => panic!("Error: temperature must be between 0 and 2"),
            },
            
            top_p: match top_p {
                Some(top_p) if (top_p <= 1.0)||(top_p >= 0.0) => Some(top_p),
                None => Some(1.0),
                _ => panic!("Error: top_p must be between 0 and 1"),
            },
            n: match n{
                Some(n) => Some(n),
                _ => Some(1),
            },
            max_tokens: max_tokens,
            stream: match stream{
                Some(stream) => Some(stream),
                _ => Some(false)
            },

            stop: match stop{
                Some(stop) => Some(stop),
                _ => None,
            },

            presence_penalty: match presence_penalty{
                Some(presence_penalty) if (presence_penalty <= 2.0)||(presence_penalty >= -2.0) => Some(presence_penalty),
                None => Some(0.0),
                _ => panic!("Error: presence_penalty must be between -2 and 2"),
            },
            frequency_penalty: match frequency_penalty {
                Some(frequency_penalty) if (frequency_penalty <= 2.0)||(frequency_penalty >= -2.0) => Some(frequency_penalty),
                None => Some(0.0),
                _ => panic!("Error: frequency_penalty must be between -2 and 2"),
            },
        }
    }

    // pub fn set_chat_user(user: ChatUser) -> Result<String, String> {
    //     if user.chat_user_is_valid() {
    //         return Ok("Chat user is not invalid".to_string());
    //     }else {
    //         return Err("Chat user is not invalid".to_string());
    //     }
    // }

    // pub fn set_temperature(&mut self, temperature: Option<f32>) -> Result<String, String> {
    //     match temperature{
    //         Some(temperature) if (temperature < 2.0)||(temperature >= 0.0) => {
    //             self.temperature = Some(temperature);
    //             Ok("Ok".to_string())
    //         },
            
    //         Some(_) => panic!("Error: temperature must be between 0 and 2"),
            
    //         None => {
    //             self.temperature = temperature;
    //             Ok("Ok".to_string())
    //         },
    //     }
    // }

    pub fn get_api(&self)-> Result<ResponseMessage, Error>{
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
            "max_tokens": self.max_tokens,
            "temperature": self.temperature,
            "top_p": self.top_p,
            "n": self.n,
            "stream": self.stream,
            // "logprobs": nullz,
            "stop": self.stop,
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
        
        Ok(response)
    }
}