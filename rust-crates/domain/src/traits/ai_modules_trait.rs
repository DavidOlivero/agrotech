use crate::models::ai_module_model::AiModules;

pub trait AiModulesTrait {
  fn initialize_ai_modules(&self) -> AiModules;
}
