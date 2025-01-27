use ureq::{self, serde_json::Value};
use crate::services::api::traits::ai_payloads::AiPayloadTrait;

pub struct GPTVisionPayLoad { }

impl AiPayloadTrait for GPTVisionPayLoad {
  fn build_payload(&self) -> Value {
    ureq::json!({
      "messages": [{
        "role": "user",
        "content": "Me puedes decir qué puedes ver en esta imágen",
        "img_url": "https://raw.githubusercontent.com/rphrp1985/mycodes/main/matagimage.png"
      }]
    })
  }
}
