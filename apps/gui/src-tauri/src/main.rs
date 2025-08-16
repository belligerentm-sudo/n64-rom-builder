#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
async fn build_project() -> Result<String, String> {
    n64_core::build_project_stub().await
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![build_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
