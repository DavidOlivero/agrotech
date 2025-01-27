use serde::{Deserialize, Serialize};
use crate::services::api::traits::{ai_configs::AiConfigTrait, ai_payloads::AiPayloadTrait};
use crate::make_ai_modules;
use make_ai_model_list::HelloMacro;
use make_ai_model_list_derive::MakeAiList;

#[derive(Serialize, Deserialize, Debug, MakeAiList)]
pub enum AIModels {
  GPTVISION,
  CHATGPT
}

pub struct AiModules {
  pub ai_config: Box<dyn AiConfigTrait>,
  pub ai_payload: Box<dyn AiPayloadTrait>
}

impl AIModels {
  pub fn initialize_ai_modules(&self) -> AiModules {
    let option = AIModels::CHATGPT;
    option.hello_macro();
    make_ai_modules!(self)
  }
}