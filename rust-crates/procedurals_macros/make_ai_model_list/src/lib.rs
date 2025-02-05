mod traits {
  pub mod ai_modules_trait;
  pub mod ai_config_trait;
  pub mod ai_payloads_trait;
}

mod ai_models_setting {
  pub mod chatgpt {
    pub mod chat_gpt_api_config;
  }

  pub mod gpt_vision {
    pub mod gpt_api_config;
  }
}

mod utilities {
  pub mod get_dotenvs_vars;
}

mod model {
  pub mod ai_module_model;
}

mod payloads {
  pub mod chatgpt_payload;
  pub mod gpt_vision_payload;
}

pub use traits::ai_modules_trait::AiModulesTrait;
pub use traits::{ ai_config_trait::AiConfigTrait, ai_payloads_trait::AiPayloadTrait };
pub use ai_models_setting::{ chatgpt::chat_gpt_api_config::CHATGPT, gpt_vision::gpt_api_config::GPTVISION };
pub use utilities::get_dotenvs_vars;
pub use model::ai_module_model::AiModules;
pub use payloads::{ chatgpt_payload::CHATGPTPAYLOAD, gpt_vision_payload::GPTVISIONPAYLOAD };
