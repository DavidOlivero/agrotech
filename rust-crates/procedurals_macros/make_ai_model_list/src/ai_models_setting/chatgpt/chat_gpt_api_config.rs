use crate::traits::ai_config_trait::AiConfigTrait;
use crate::utilities::get_dotenvs_vars::return_dotenv_var;
use std::env;

pub struct CHATGPT {
  key: String,
  host: &'static str,
  url: &'static str 
}

impl CHATGPT {
  pub fn new() -> Result<Self, env::VarError> {
    Ok(Self {
      key: return_dotenv_var("API_KEY")?,
      host: "chatgpt-42.p.rapidapi.com",
      url: "https://chatgpt-42.p.rapidapi.com/gpt4"
    })
  }
}

impl AiConfigTrait for CHATGPT {
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