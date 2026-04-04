mod window_info;
mod main_backend;

use std::sync::Arc;
use window_info::get_foreground_window_info;
use main_backend::MainBackend;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let backend = Arc::new(MainBackend::new());
    

    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::time())
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .init();

    tracing::info!("应用启动");
    backend.clone().startMainLoop();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_foreground_window_info])
        .setup(|_app| {
            // 启动一个异步任务，每秒输出 hello
            tauri::async_runtime::spawn(async {
                loop {
                    tracing::info!("hello");
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
