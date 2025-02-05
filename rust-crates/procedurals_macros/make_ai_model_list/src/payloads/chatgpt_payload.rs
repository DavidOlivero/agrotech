use serde_json::{Value, json};
use crate::traits::ai_payloads_trait::AiPayloadTrait;

pub struct CHATGPTPAYLOAD { }

impl AiPayloadTrait for CHATGPTPAYLOAD {
  fn build_payload(&self) -> Value {
    json!({
      "messages": [{
        "role": "user",
        "content": "Hola, me puedes contar un poco sobre la época victoriana",
        "web_access": false
      }]
    })
  }
}