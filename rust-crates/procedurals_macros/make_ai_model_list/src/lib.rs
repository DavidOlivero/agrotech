mod ai_models_setting;
mod utilities;
mod payloads;

pub use domain::traits::ai_modules_trait::AiModulesTrait;
pub use domain::traits::{ ai_config_trait::AiConfigTrait, ai_payloads_trait::AiPayloadTrait };
pub use ai_models_setting::{ chatgpt::chat_gpt_api_config::CHATGPT, gpt_vision::gpt_api_config::GPTVISION };
pub use utilities::get_dotenvs_vars;
pub use domain::models::ai_module_model::AiModules;
pub use payloads::{ chatgpt_payload::CHATGPTPAYLOAD, gpt_vision_payload::GPTVISIONPAYLOAD };
