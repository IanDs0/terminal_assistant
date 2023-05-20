#[derive(Debug)]
pub enum Model {
    gpt_4, 
    gpt_4_0314,
    gpt_4_32k, 
    gpt_4_32k_0314, 
    gpt_35_turbo, 
    gpt_35_turbo_0301
}

impl Model {
    pub fn parse_model(&self) -> String {
        match self {
            Model::gpt_4 => String::from("gpt-4"),
            Model::gpt_4_0314 => String::from("gpt-4-0314"),
            Model::gpt_4_32k => String::from("gpt-4-32k"),
            Model::gpt_4_32k_0314 => String::from("gpt-4-32k-0314"),
            Model::gpt_35_turbo => String::from("gpt-3.5-turbo"),
            Model::gpt_35_turbo_0301 => String::from("gpt-3.5-turbo-0301"),
        }
    }
}