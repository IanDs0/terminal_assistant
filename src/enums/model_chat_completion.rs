#[derive(Debug)]
pub enum Model {
    Gpt4, 
    Gpt4_0314,
    Gpt4_32k, 
    Gpt4_32k0314, 
    Gpt3_5Turbo, 
    Gpt3_5Turbo0301
}

impl Model {
    pub fn parse_model(&self) -> String {
        match self {
            Model::Gpt4 => String::from("gpt-4"),
            Model::Gpt4_0314 => String::from("gpt-4-0314"),
            Model::Gpt4_32k => String::from("gpt-4-32k"),
            Model::Gpt4_32k0314 => String::from("gpt-4-32k-0314"),
            Model::Gpt3_5Turbo => String::from("gpt-3.5-turbo"),
            Model::Gpt3_5Turbo0301 => String::from("gpt-3.5-turbo-0301"),
        }
    }
}