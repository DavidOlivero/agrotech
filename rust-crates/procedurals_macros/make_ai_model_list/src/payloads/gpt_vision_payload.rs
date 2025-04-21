use serde_json::{Value, json};
use domain::traits::ai_payloads_trait::AiPayloadTrait;

pub struct GPTVISIONPAYLOAD { }

impl AiPayloadTrait for GPTVISIONPAYLOAD {
  fn build_payload(&self) -> Value {
    json!({
      "messages": [{
        "role": "user",
        "content": "Me puedes decir qué puedes ver en esta imágen",
        "img_url": "https://raw.githubusercontent.com/rphrp1985/mycodes/main/matagimage.png"
      }]
    })
  }
}
