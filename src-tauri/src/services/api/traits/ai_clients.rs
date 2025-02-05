use make_ai_model_list::{ AiConfigTrait, AiPayloadTrait };
use ureq::Error;

pub trait AiClientsTrait {
  fn consume_ai(ai_config: &dyn AiConfigTrait, payload_manager: &dyn AiPayloadTrait) -> Result<String, Error>;
}