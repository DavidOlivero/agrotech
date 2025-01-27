use crate::services::api::traits::{ai_configs::AiConfigTrait, ai_clients::AiClientsTrait, ai_payloads::AiPayloadTrait};
use ureq::{self, Error};

pub struct AiClient { }

impl AiClientsTrait for AiClient {
  fn consume_ai(ai_config: &dyn AiConfigTrait, payload_manager: &dyn AiPayloadTrait) -> Result<String, Error> {
    let mut request = ureq::post(ai_config.get_url());
    for (key, value) in ai_config.get_api_header() {
        request = request.set(key, value)
    }
    
    let payload = payload_manager.build_payload();
    let response = request.send_json(payload)?.into_string()?;
    Ok(response)
  }
}