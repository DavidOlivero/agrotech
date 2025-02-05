pub mod ai_model {
  pub mod models_manager;
}

pub mod api {
  pub mod client {
    pub mod client;
  }
  pub mod traits {
    pub mod ai_clients;
  }

  pub mod http {
    pub mod http_manager;
    pub mod http_responses;
  }
}
