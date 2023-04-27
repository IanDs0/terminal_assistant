// // use std::collections::HashMap;
// use std::io;

// use reqwest::Error;
// use reqwest::header::AUTHORIZATION;

use clap::Parser;
use clap;

use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::prelude::*;

// use std::env;

mod args;

use args::AssistantArgs;
use args::HelperType;

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    text: Option<String>,
}
/// Simple program to greet a person
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Token to use
    #[arg(short, long)]
    token: String,
}




/*
fn getAPI(prompt:String) -> Result<(), Error>{
    
    let key_api: &str = "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";

    let client = reqwest::blocking::Client::new();

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", key_api))
        .json(&serde_json::json!({
            "model": "gpt-3.5-turbo",
            "messages": [{"role": "user", "content": prompt}],
            // "max_tokens": 7,
            // "temperature": 0,
            // "top_p": 1,
            // "n": 1,
            // "stream": false,
            // "logprobs": null,
            // "stop": "\n"
        }))
        .send();

    println!("{}", response?.text()?);

    Ok(())
}
*/
fn main() {
    // let args = Args::parse();
    
    let args = AssistantArgs::parse();
    
    match args.helper {
        HelperType::RegisterToken(RegisterToken) => {
            println!("{}", RegisterToken.token);
            
            // let mut file = File::create(".env")?;
            // file.write_all(("TOKEN=".to_owned() + &RegisterToken.token).as_bytes())?;
        },

        HelperType::HelpCommand(HelpCommand) => {
            println!("{}", HelpCommand.token);
        },

        HelperType::CrateTranscription(CrateTranscription) => {
            println!("{}", CrateTranscription.token);
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