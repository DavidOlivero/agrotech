#[macro_export]
macro_rules! make_ai_modules {
  ($key:expr) => {
    {
      let ai_modules: (Box<dyn crate::services::api::traits::ai_configs::AiConfigTrait>, Box<dyn crate::services::api::traits::ai_payloads::AiPayloadTrait>) = match format!("{:?}", $key).as_str() {
        "GPTVISION" => (
          Box::new(crate::services::api::config::gpt_vision::gpt_api_config::GPTVisionConfig::new().unwrap()),
          Box::new(crate::services::api::client::payloads::gpt_vision_payload::GPTVisionPayLoad {})
        ),
        "CHATGPT" => (
          Box::new(crate::services::api::config::chatgpt::chat_gpt_api_config::ChatGPTApiConfig::new().unwrap()),
          Box::new(crate::services::api::client::payloads::chatgpt_payload::ChatGptPayLoad {})
        ),
        _ => panic!("No se reconoce el modelo de IA: {:?}", $key)
      };

      let (ai_config, ai_payload) = ai_modules;
      crate::services::ai_model::models_manager::AiModules {
        ai_config,
        ai_payload
      }
    }
  };
}