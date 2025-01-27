use std::collections::HashMap;

use crate::services::api::{client::payloads::{chatgpt_payload::ChatGptPayLoad, gpt_vision_payload::GPTVisionPayLoad}, config::{chatgpt::chat_gpt_api_config::ChatGPTApiConfig, gpt_vision::gpt_api_config::GPTVisionConfig}, traits::{ai_configs::AiConfigTrait, ai_payloads::AiPayloadTrait}};

pub fn get_models_list() -> HashMap<&'static str, (Box<dyn AiConfigTrait>, Box<dyn AiPayloadTrait>)> {
  let mut models: HashMap<&str, (Box<dyn AiConfigTrait>, Box<dyn AiPayloadTrait>)> = HashMap::new();
  models.insert("GPTVISION", (Box::new(GPTVisionConfig::new().unwrap()), Box::new(GPTVisionPayLoad {})));
  models.insert("CHATGPT", (Box::new(ChatGPTApiConfig::new().unwrap()), Box::new(ChatGptPayLoad {})));

  models
}