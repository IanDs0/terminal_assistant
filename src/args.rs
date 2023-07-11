use clap::{
    // Args, 
    Parser, 
    // Subcommand
};


#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct AssistantArgs {
    /// Token to use
    #[clap(subcommand)]
    pub helper: HelperType,
}

#[derive(Parser, Debug)]
pub enum HelperType{

    RegisterToken(RegisterToken),

    HelpCommand(HelpCommand),

    CrateTranscription(CrateTranscription),

}

#[derive(Parser, Debug)]
pub struct RegisterToken {
    /// Key to use
    #[clap(short, long)]
    pub key: String,
}

#[derive(Parser, Debug)]
pub struct HelpCommand {
    /// Question to command
    // #[clap(short, long)]
    pub question: String,
    
    /// Key to use
    #[clap(short, long)]
    pub key: Option<String>,

    /// Max tokens to use
    #[clap(long)]
    pub max_tokens: Option<u32>,

    /// User to use
    #[clap(short, long)]
    pub user: Option<String>,

    /// Use to temperature
    #[clap(long)]
    pub temperature: Option<f32>,

    /// Use to top_p
    #[clap(long)]
    pub top_p: Option<f32>,
    
    /// Use to n
    #[clap(short, long)]
    pub n: Option<u64>,

    /// Use to model
    #[clap(long)]
    pub model: Option<String>,
}

#[derive(Parser, Debug)]
pub struct CrateTranscription{

    /// Path to file
    // #[clap(short, long)]
    pub file_path: String,

    /// Question to command
    #[clap(short, long)]
    pub save_name: Option<String>,
    
    /// Key to use
    #[clap(short, long)]
    pub key: Option<String>,
}