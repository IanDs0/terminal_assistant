use clap::Parser;
use clap;

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use infer;

use dotenv::dotenv;

mod args;
use args::AssistantArgs;
use args::HelperType;

mod enums;
use crate::enums::model_chat_completion::Model;
use crate::enums::role_message::RoleMessage;

mod api;
use crate::api::chat_completion::ChatCompletion;

mod structs;
use crate::structs::user_chat::ChatUser;
use crate::structs::message_chat_completion::Message;

fn main() {
    
    let args = AssistantArgs::parse();
    
    let user:String = "Ian".to_string();
    
    match args.helper {
        HelperType::RegisterToken(register_token) => {
            // println!("{}", RegisterToken.key);
            
            let mut file = match fs::File::create(".env"){
                Ok(file) => file,
                Err(e) => panic!("Error: {}", e),
            };
            match file.write_all(("KEY=".to_owned() + &register_token.key).as_bytes()){
                Ok(_) => return,
                Err(e) => println!("Error: {}", e),
            };
        },

        
        HelperType::HelpCommand(help_command) => {

            let default_chat: Message = Message::new(
                RoleMessage::system,
                "You are a handy terminal command assistant that only responds to the command and response in pt-br. answer me in the space of 15 tokens".to_string(), 
                None
            );

            let model: Model = Model::Gpt3_5Turbo;
            let mut array:Vec<Message> = Vec::new();

            array.push(default_chat);

            match help_command.key {
                Some(token) => {

                    //user set
                    let user_chat: ChatUser = ChatUser::new(
                        Some(help_command.user.unwrap_or(user)),
                        token,
                    );

                    //promt usuario
                    let default_chat: Message = Message::new(
                        RoleMessage::user,
                        help_command.question, 
                        Some("Ian".to_string())
                    );

                    array.push(default_chat);


                    let chat: ChatCompletion = ChatCompletion::new(
                        user_chat,
                        model,
                        array,
                        help_command.temperature,
                        help_command.top_p,
                        help_command.n,
                        help_command.max_tokens,
                        Some(false),
                        
                        None,
                    
                        None,
                        None,
                    );

                    match chat.get_api(){
                        Ok(response) => {
                            println!("{:#}", response.choices[0].message.content);
                            return;
                        },
                        Err(e) => println!("Error: {}", e),
                    };
                },
                None => {
                    if let Ok(metadata) = fs::metadata(".env") {
                        if metadata.is_file() {
                            
                            dotenv().ok();
                            let token = std::env::var("KEY").unwrap();

                            //user set
                            let user_chat: ChatUser = ChatUser::new(
                                Some(help_command.user.unwrap_or(user)),
                                token,
                            );

                            //promt usuario
                            let default_chat: Message = Message::new(
                                RoleMessage::user,
                                help_command.question, 
                                Some("Ian".to_string())
                            );

                            array.push(default_chat);


                            let chat: ChatCompletion = ChatCompletion::new(
                                user_chat,
                                model,
                                array,
                                help_command.temperature,
                                help_command.top_p,
                                help_command.n,
                                help_command.max_tokens,
                                Some(false),
                                
                                None,
                                None,
                                None,
                            );

                            match chat.get_api(){
                                Ok(response) => {
                                    println!("{:#}", response.choices[0].message.content);
                                    return;
                                },
                                Err(e) => println!("Error: {}", e),
                            };

                        } else {
                            println!("Register the Token so you can make this call or use the ' --key ' parameter");
                        }
                    } else {
                        println!("Register the Token so you can make this call or use the ' --key ' parameter");
                    }
                    return;
                }
            };
            
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
                    println!("Invalid relative path");
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
                println!("Filename: {}", file_name);

            
                match infer::get_from_path(file_path.to_string_lossy().to_string()) {
                    Ok(Some(info)) => {

                        match info.extension() {
                            "mp3"|"mp4"|"mpeg"|"m4a"|"wav"|"webm" => println!("Valid file"),
                            /*
                            "mp3" => println!("Arquivo de áudio MP3 válido!"),
                            "mp4" => println!("Arquivo de vídeo MP4 válido!"),
                            "mpeg" => println!("Arquivo de vídeo MPEG válido!"),
                            "m4a" => println!("Arquivo de áudio MPEG-4 válido!"),
                            "wav" => println!("Arquivo de áudio WAV válido!"),
                            "webm" => println!("Arquivo de vídeo WebM válido!"),
                            */
                            _ => println!("Invalid file"),
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