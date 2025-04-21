use crate::traits::{ ai_config_trait::AiConfigTrait, ai_payloads_trait::AiPayloadTrait };

pub struct AiModules {
  pub ai_config: Box<dyn AiConfigTrait>,
  pub ai_payload: Box<dyn AiPayloadTrait>
}