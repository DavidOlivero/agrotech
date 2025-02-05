use serde::{Deserialize, Serialize};
use make_ai_model_list::{ AiModulesTrait, AiModules, CHATGPT, GPTVISION, CHATGPTPAYLOAD, GPTVISIONPAYLOAD };
use make_ai_model_list_derive::MakeAiList;

#[derive(Serialize, Deserialize, MakeAiList, Debug)]
pub enum AIModels {
  GPTVISION,
  CHATGPT
}

impl AIModels {
  pub fn make_ai_modules(&self) -> AiModules {
    let test: AIModels = AIModels::CHATGPT;
    test.initialize_ai_modules()
  }
}