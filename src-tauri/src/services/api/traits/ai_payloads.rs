use ureq::serde_json::Value;

pub trait AiPayloadTrait {
  fn build_payload(&self) -> Value;
}