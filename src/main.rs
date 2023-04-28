// // use std::collections::HashMap;
// use std::io;

use reqwest::Error;
use reqwest::header::AUTHORIZATION;

use clap::Parser;
use clap;

use serde::{Deserialize, Serialize};

use std::fs;
use std::io::Write;

use dotenv::dotenv;

mod args;

use args::AssistantArgs;
use args::HelperType;

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
/// Simple program to greet a person
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Token to use
    #[arg(short, long)]
    token: String,
}


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
            // println!("{:?}", help_command.token);

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

            
        },

        HelperType::CrateTranscription(crate_transcription) => {
            // println!("{}", crate_transcription.token);

            let input = crate_transcription.file_path;
            match input.contains(".") {
                true => {
                    if let Ok(current_dir) = std::env::current_dir() {
                        println!("{:?}{}", current_dir, input);
                    } else {
                        println!("Não foi possível obter o diretório atual.");
                    }
                },
                false => {
                    println!("{:?}", input);
                },
            };
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