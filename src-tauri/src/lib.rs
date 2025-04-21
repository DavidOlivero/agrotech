pub mod services;

use services::{ ai_model::models_manager::AIModels, api::http::http_manager::HTTPManager };

#[tauri::command]
async fn ask_to_ai(ai_model: AIModels) -> String {
  HTTPManager::make_request(ai_model)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![ask_to_ai])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
