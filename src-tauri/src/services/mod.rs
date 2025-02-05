pub mod ai_model {
  pub mod models_manager;
  pub mod model_list;
}

pub mod api {
  pub mod config {
    pub mod gpt_vision {
      pub mod gpt_api_config;
    }
    pub mod chatgpt {
      pub mod chat_gpt_api_config;
    }
  }
  pub mod client {
    pub mod client;
    pub mod payloads {
      pub mod gpt_vision_payload;
      pub mod chatgpt_payload;
    }
  }
  pub mod traits {
    pub mod ai_configs;
    pub mod ai_clients;
    pub mod ai_payloads;
    pub mod ai_model_list;
  }

  pub mod http {
    pub mod http_manager;
    pub mod http_responses;
  }
}
