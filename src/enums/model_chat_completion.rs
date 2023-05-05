pub enum Model {
    gpt_4, 
    gpt_4_0314,
    gpt_4_32k, 
    gpt_4_32k_0314, 
    gpt_35_turbo, 
    gpt_35_turbo_0301
}

pub fn parse_model(s: &str) -> Option<Model> {
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