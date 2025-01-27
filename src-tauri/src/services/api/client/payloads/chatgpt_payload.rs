use ureq::{self, serde_json::Value};
use crate::services::api::traits::ai_payloads::AiPayloadTrait;

pub struct ChatGptPayLoad { }

impl AiPayloadTrait for ChatGptPayLoad {
  fn build_payload(&self) -> Value {
    ureq::json!({
      "messages": [{
        "role": "user",
        "content": "Hola, me puedes contar un poco sobre la época victoriana",
        "web_access": false
      }]
    })
  }
}