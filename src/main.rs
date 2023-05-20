use reqwest::Error;
use reqwest::header::AUTHORIZATION;

use clap::Parser;
use clap;

use serde::{Deserialize, Serialize};

use std::fs;
use std::io::Write;
use std::path::PathBuf;
// // use std::collections::HashMap;
// use std::env::args;
// use std::process::exit;

use infer;//::{Infer, Type};

use dotenv::dotenv;

mod args;
use args::AssistantArgs;
use args::HelperType;

mod enums;
use crate::enums::model_chat_completion;
use crate::enums::role_message;

mod api;
use crate::api::chat_completion;

mod structs;
use crate::structs::user_chat::ChatUser;
use crate::structs::message_chat_completion;
/*
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
*/

/*
// #[tokio::main]
fn get_api(
    prompt:String, 
    token:String, 
    user:String,
    _max_tokens:u32,
    ) -> Result<(), Error>{

    let client = reqwest::blocking::Client::new();

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .json(&serde_json::json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "system", 
                    "content": "You are a handy terminal command assistant that only responds to the command and response in pt-br. answer me in the space of 15 tokens"},
                {
                    "role": "user", 
                    "content": prompt
                }
            ],
            "user": user,
            // "max_tokens": max_tokens,
            // "temperature": 0,
            // "top_p": 1,
            // "n": 1,
            // "stream": false,
            // "logprobs": null,
            // "stop": "\n"
        }))
        .send();
    let teste: ResponseMessage = match serde_json::from_str(response?.text()?.as_str()){
        Ok(teste) => teste,
        Err(e) => panic!("Error: {}", e),
    };
    println!("{}", teste.choices[0].message.content);

    Ok(())
}
*/


fn main() {
    
    let args = AssistantArgs::parse();
    
    let max_tokens:u32 = 25;
    let user:String = "Ian".to_string();
    
    match args.helper {
        HelperType::RegisterToken(register_token) => {
            // println!("{}", RegisterToken.token);
            
            let mut file = match fs::File::create(".env"){
                Ok(file) => file,
                Err(e) => panic!("Error: {}", e),
            };
            match file.write_all(("TOKEN=".to_owned() + &register_token.token).as_bytes()){
                Ok(_) => return,
                Err(e) => println!("Error: {}", e),
            };
        },

        
        HelperType::HelpCommand(help_command) => {

            //token
            // println!("{:?}", help_command.token);
            let token = "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".to_string(); 
            


            //model
            let model: model_chat_completion::Model = model_chat_completion::Model::gpt_35_turbo;
            // println!("{}", model.parse_model());



            //user set
            let user_chat: ChatUser = ChatUser::new(
                help_command.user.unwrap_or(user),
                help_command.token.unwrap_or(token),
            );
            // println!("{:?}", user_chat);



            //array promt
            let default_chat: message_chat_completion::Message = message_chat_completion::Message::new(
                role_message::RoleMessage::system,
                "You are a handy terminal command assistant that only responds to the command and response in pt-br. answer me in the space of 15 tokens".to_string(), 
                None
            );

            // println!("{:?}", default_chat);

            let mut array:Vec<message_chat_completion::Message> = Vec::new();

            array.push(default_chat);

            //promt usuario
            let default_chat: message_chat_completion::Message = message_chat_completion::Message::new(
                role_message::RoleMessage::user,
                help_command.question, 
                Some("Ian".to_string())
            );

            // println!("{:?}", default_chat);

            array.push(default_chat);

            // println!("{:?}", array);

            let temperature:f32 = 0.0;
            let top_p:f32 = 1.0;
            let n:u64 = 1;

            let mut chat: chat_completion::ChatCompletion = chat_completion::ChatCompletion::new(
                user_chat,
                model,
                array,
                temperature,
                top_p,
                n,
                max_tokens,
            );

            // println!("\n\n\n{:?}\n\n\n", chat);

            match chat.get_api(){
                Ok(_) => {},
                Err(e) => println!("Error: {}", e),
            };

            /*
            match help_command.token {
                Some(token) => {
                    match get_api(
                        help_command.question, 
                        token, 
                        user,
                        max_tokens
                        ){
                            
                        Ok(_) => return,
                        Err(e) => println!("Error: {}", e),
                    };
                },
                None => {
                    if let Ok(metadata) = fs::metadata(".env") {
                        if metadata.is_file() {
                            
                            dotenv().ok();
                            let token = std::env::var("TOKEN").unwrap();

                            match get_api(
                                help_command.question, 
                                token, 
                                user,
                                max_tokens
                                ){
                                    
                                Ok(_) => return,
                                Err(e) => println!("Error: {}", e),
                            };

                        } else {
                            println!("Register the Token so you can make this call or use the ' --token ' parameter");
                        }
                    } else {
                        println!("Register the Token so you can make this call or use the ' --token ' parameter");
                    }
                    return;
                }
            };
            */
            
        },

        /*tamanho maximo de 25mb dos tipos [mp3, mp4, mpeg, mpga, m4a, wav, webm].*/
        HelperType::CrateTranscription(crate_transcription) => {
            // println!("{}", crate_transcription.token);

            let mut file_path = PathBuf::from(crate_transcription.file_path);

            // Verifica se o caminho é relativo
            if file_path.is_relative() {
                // Transforma o caminho relativo em um caminho absoluto
                if let Ok(abs_path) = std::fs::canonicalize(&file_path) {
                    file_path = abs_path;
                } else {
                    println!("Caminho relativo inválido");
                    return;
                }
            }

            // Verifica se o arquivo existe
            if file_path.exists() {
                // Obtém o nome do arquivo
                let file_name = file_path
                    .file_name()
                    .and_then(|os_str| os_str.to_str())
                    .unwrap();
                println!("Nome do arquivo: {}", file_name);

            
                match infer::get_from_path(file_path.to_string_lossy().to_string()) {
                    Ok(Some(info)) => {

                        match info.extension() {
                            "mp3" => println!("Arquivo de áudio MP3 válido!"),
                            "mp4" => println!("Arquivo de vídeo MP4 válido!"),
                            "mpeg" => println!("Arquivo de vídeo MPEG válido!"),
                            "m4a" => println!("Arquivo de áudio MPEG-4 válido!"),
                            "wav" => println!("Arquivo de áudio WAV válido!"),
                            "webm" => println!("Arquivo de vídeo WebM válido!"),
                            _ => println!("Arquivo inválido"),
                        }

                    }
                    Ok(None) => {
                        eprintln!("Unknown file type 😞");
                        eprintln!("If you think the assissten should be able to recognize this file type open an issue on GitHub!");
                    }
                    Err(e) => {
                        eprintln!("Looks like something went wrong 😔");
                        eprintln!("{}", e);
                    }
                }            

            } else {
                println!("File not found");
            }
        },
    };
    // Ok(());
}


/*
{
    "id":"chatcmpl-78cZGiYU7lp5jsoIt76MPj2Ea24CA",
    "object":"chat.completion",
    "created":1682289234,
    "model":"gpt-3.5-turbo-0301",
    "usage":{
        "prompt_tokens":11,
        "completion_tokens":7,
        "total_tokens":18
    },
    "choices":[
        {
            "message":{
                "role":"assistant",
                "content":"Olá! Como posso ajud"
            },
            "finish_reason":"length",
            "index":0
        }
    ]
}
*/