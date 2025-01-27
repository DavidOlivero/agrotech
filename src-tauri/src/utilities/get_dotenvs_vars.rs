use dotenv::dotenv;
use std::env;

pub fn return_dotenv_var(key: &str) -> Result<String, env::VarError> {
  dotenv().ok();
  env::var(key)
}