mod database;
mod tracker;
mod window_info;
mod main_backend;

use database::{AggregatedUsage, UsageRecord};
use tracker::Tracker;
use window_info::get_foreground_window_info;
use main_backend::MainBackend;

use rusqlite::Connection;
use std::sync::{Arc, Mutex};

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager,
};

struct AppState {
    db: Arc<Mutex<Connection>>,
}

#[tauri::command]
fn get_today_records(state: tauri::State<'_, AppState>) -> Result<Vec<UsageRecord>, String> {
    let today = today_date_str();
    let db = state.db.lock().map_err(|e| e.to_string())?;
    database::get_records_by_date(&db, &today).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_today_aggregated(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<AggregatedUsage>, String> {
    let today = today_date_str();
    let db = state.db.lock().map_err(|e| e.to_string())?;
    database::get_aggregated_by_date(&db, &today).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_records_by_date(
    state: tauri::State<'_, AppState>,
    date: String,
) -> Result<Vec<UsageRecord>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    database::get_records_by_date(&db, &date).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_records_by_date_range(
    state: tauri::State<'_, AppState>,
    start_date: String,
    end_date: String,
) -> Result<Vec<UsageRecord>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    database::get_records_by_date_range(&db, &start_date, &end_date)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_aggregated_by_date(
    state: tauri::State<'_, AppState>,
    date: String,
) -> Result<Vec<AggregatedUsage>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    database::get_aggregated_by_date(&db, &date).map_err(|e| e.to_string())
}

fn today_date_str() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let dt = chrono::DateTime::from_timestamp(now, 0).unwrap_or_default();
    dt.format("%Y-%m-%d").to_string()
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
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;
            let db_path = app_data_dir.join("time-lens.db");
            tracing::info!("数据库路径：{:?}", db_path);

            let conn = database::init(&db_path)?;
            let db = Arc::new(Mutex::new(conn));

            // Cleanup records older than 90 days
            {
                let db_lock = db.lock().unwrap();
                let _ = database::cleanup_old_records(&db_lock, 90);
            }

            let handle = app.handle().clone();
            app.manage(AppState { db: db.clone() });

            // Start background tracker
            Tracker::start(handle.clone(), db);

            // System tray
            let show_item =
                MenuItemBuilder::with_id("show", "显示主窗口").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&show_item)
                .item(&quit_item)
                .build()?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().cloned().unwrap())
                .tooltip("Time Lens - 记录应用使用时间")
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if matches!(event, TrayIconEvent::DoubleClick { .. }) {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_foreground_window_info,
            get_today_records,
            get_today_aggregated,
            get_records_by_date,
            get_records_by_date_range,
            get_aggregated_by_date,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
