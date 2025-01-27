use super::{ai_configs::AiConfigTrait, ai_payloads::AiPayloadTrait};
use ureq::Error;

pub trait AiClientsTrait {
  fn consume_ai(ai_config: &dyn AiConfigTrait, payload_manager: &dyn AiPayloadTrait) -> Result<String, Error>;
}