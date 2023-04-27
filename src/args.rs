use clap::{
    Args, 
    Parser, 
    Subcommand
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
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
    /// Token to use
    #[clap(short, long)]
    pub token: String,
}

#[derive(Parser, Debug)]
pub struct HelpCommand {
    /// Token to use
    #[clap(short, long)]
    pub token: String,

    /// Question to command
    #[clap(short, long)]
    pub question: String,
}

#[derive(Parser, Debug)]
pub struct CrateTranscription{
    /// Token to use
    #[clap(short, long)]
    pub token: String,

    /// Question to command
    #[clap(short, long)]
    pub text: String,

    /// Path to file
    #[clap(short, long)]
    pub path: String,
}