pub trait AiConfigTrait {
  fn get_api_header(&self) -> Vec<(&str, &str)>;
  fn get_url(&self) -> &str;
}