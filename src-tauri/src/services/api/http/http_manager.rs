use crate::services::{ai_model::models_manager::AIModels, api::{client::client::AiClient, traits::ai_clients::AiClientsTrait}};
use super::http_responses::HTTPResponse;

pub struct HTTPManager { }

impl HTTPManager {
  pub fn make_request(ai_model: AIModels) -> String {
    let ai_modules = ai_model.initialize_ai_modules();
    let response = AiClient::consume_ai(&*ai_modules.ai_config, &*ai_modules.ai_payload);

    response.unwrap_or_else(|err| {
      HTTPResponse::get_message(err)
    })
  }
}