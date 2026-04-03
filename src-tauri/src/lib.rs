mod window_info;

use window_info::get_foreground_window_info;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::time())
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .init();

    tracing::info!("应用启动");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_foreground_window_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
