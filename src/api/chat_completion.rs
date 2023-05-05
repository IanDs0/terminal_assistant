use reqwest::Error;
use reqwest::header::AUTHORIZATION;

use serde::{Deserialize, Serialize};




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


//message
//foi
#[derive(Debug, Serialize, Deserialize)]
enum RoleMessage {
    assistant,
    user,
    system,
}
//
#[derive(Debug, Serialize, Deserialize)]
struct Message{
    role: RoleMessage,
    content: String,
    name:String,
}

//model
//foi
#[derive(Debug, Serialize, Deserialize)]
enum Model {
    gpt_4, 
    gpt_4_0314,
    gpt_4_32k, 
    gpt_4_32k_0314, 
    gpt_35_turbo, 
    gpt_35_turbo_0301
}
//foi
fn parse_model(s: &str) -> Option<Model> {
    match s {
        "gpt-4" => Some(Model::gpt_4),
        "gpt-4-0314" => Some(Model::gpt_4_0314),
        "gpt-4-32k" => Some(Model::gpt_4_32k),
        "gpt-4-32k-0314" => Some(Model::gpt_4_32k_0314),
        "gpt-3.5-turbo" => Some(Model::gpt_35_turbo),
        "gpt-3.5-turbo-0301" => Some(Model::gpt_35_turbo_0301),
        _ => None,
    }
}

//meio q o main
struct ChatCompletion{
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
    fn new(
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

    /*
    pub fn get_api(&self)-> Result<(), Error>{
        let client = reqwest::blocking::Client::new();

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.user.token))
            .json(&serde_json::json!({
                "model": self.model,
                "messages": self.messages,
                "user": self.user.user_name,
                // "max_tokens": max_tokens,
                // "temperature": 0,
                // "top_p": 1,
                // "n": 1,
                // "stream": false,
                // "logprobs": null,
                // "stop": "\n"
            }))
            .send();
        // let teste: ResponseMessage = match serde_json::from_str(response?.text()?.as_str()){
        //     Ok(teste) => teste,
        //     Err(e) => panic!("Error: {}", e),
        // };
        // println!("{}", teste.choices[0].message.content);
    
        Ok(())
    }
    */
}