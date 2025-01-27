use crate::services::api::traits::ai_configs::AiConfigTrait;
use crate::utilities::get_dotenvs_vars::return_dotenv_var;
use std::env;

pub struct GPTVisionConfig {
  key: String,
  host: &'static str,
  url: &'static str 
}

impl GPTVisionConfig {
  pub fn new() -> Result<Self, env::VarError> {
    Ok(Self {
      key: return_dotenv_var("API_KEY")?,
      host: "chatgpt-42.p.rapidapi.com",
      url: "https://chatgpt-42.p.rapidapi.com/matagvision"
    })
  }   
}

impl AiConfigTrait for GPTVisionConfig {
  fn get_api_header(&self) -> Vec<(&str, &str)> {
    vec![
      ("x-rapidapi-key", self.key.as_str()),
      ("x-rapidapi-host", self.host)
    ]
  }

  fn get_url(&self) -> &str {
    self.url
  }
}