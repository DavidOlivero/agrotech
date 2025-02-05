use super::super::traits::ai_config_trait::AiConfigTraitExample;

pub struct GPTVISION {
  pub api_headers: &'static str,
  pub url: &'static str,
}

pub struct CHATGPT {
  pub api_headers: &'static str,
  pub url: &'static str,
}

impl GPTVISION {
  pub fn new() -> GPTVISION {
    Self {
      api_headers: "Api header",
      url: "https://test.test"
    }
  }
}

impl AiConfigTraitExample for GPTVISION {
  fn get_api_header(&self) -> Vec<(&str, &str)> {
    vec![(self.api_headers, self.api_headers)]
  }

  fn get_url(&self) -> &str {
    self.url
  }
}

impl CHATGPT {
  pub fn new() -> CHATGPT {
    Self {
      api_headers: "Api header",
      url: "https://test.test"
    }
  }
}

impl AiConfigTraitExample for CHATGPT {
  fn get_api_header(&self) -> Vec<(&str, &str)> {
    vec![(self.api_headers, self.api_headers)]
  }

  fn get_url(&self) -> &str {
    self.url
  }
}

