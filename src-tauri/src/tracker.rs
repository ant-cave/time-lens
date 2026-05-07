use crate::database::{self, UsageRecord};
use crate::window_info::get_foreground_window_info;
use rusqlite::Connection;
use serde::Serialize;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize)]
pub struct CurrentSession {
    pub app_name: String,
    pub window_title: String,
    pub exe_path: String,
    pub start_time: i64,
    pub elapsed_seconds: i64,
}

pub struct Tracker {
    pub running: Arc<AtomicBool>,
}

impl Tracker {
    pub fn start(app_handle: AppHandle, db: Arc<Mutex<Connection>>) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let flag = running.clone();

        std::thread::spawn(move || {
            let mut last_app_name = String::new();
            let mut last_title = String::new();
            let mut last_exe_path = String::new();
            let mut session_start: i64 = 0;
            let mut last_emit: i64 = 0;

            loop {
                if !flag.load(Ordering::Relaxed) {
                    break;
                }

                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as i64;

                let today = {
                    use chrono::DateTime;
                    let dt = DateTime::from_timestamp(now, 0).unwrap_or_default();
                    dt.format("%Y-%m-%d").to_string()
                };

                // Poll foreground window
                let (app_name, title, exe_path) =
                    match get_foreground_window_info() {
                        Some(info) if !info.exe_path.is_empty() => {
                            (extract_app_name(&info.exe_path), info.title, info.exe_path)
                        }
                        _ => (String::new(), String::new(), String::new()),
                    };

                let app_changed =
                    app_name != last_app_name
                        || (app_name.is_empty() && !last_app_name.is_empty());

                let title_changed = !app_name.is_empty()
                    && !last_app_name.is_empty()
                    && app_name == last_app_name
                    && title != last_title;

                if app_changed || title_changed {
                    // Save previous session
                    if !last_app_name.is_empty() && session_start > 0 {
                        let duration = now - session_start;
                        if duration >= 2 {
                            let record = UsageRecord {
                                id: 0,
                                app_name: last_app_name.clone(),
                                window_title: last_title.clone(),
                                exe_path: last_exe_path.clone(),
                                start_time: session_start,
                                end_time: Some(now),
                                duration_seconds: duration,
                                date: today.clone(),
                            };
                            if let Ok(db_lock) = db.lock() {
                                if let Err(e) = database::insert_record(&db_lock, &record) {
                                    tracing::error!("Failed to save record: {e}");
                                }
                            }
                        }
                    }

                    // Start new session
                    last_app_name = app_name.clone();
                    last_title = title.clone();
                    last_exe_path = exe_path.clone();
                    session_start = now;

                    if !app_name.is_empty() {
                        let _ = app_handle.emit(
                            "app-changed",
                            CurrentSession {
                                app_name: app_name.clone(),
                                window_title: title.clone(),
                                exe_path: exe_path.clone(),
                                start_time: now,
                                elapsed_seconds: 0,
                            },
                        );
                    }

                    // Emit aggregated data
                    if let Ok(db_lock) = db.lock() {
                        if let Ok(agg) = database::get_aggregated_by_date(&db_lock, &today) {
                            let _ = app_handle.emit("today-aggregated", &agg);
                        }
                    }

                    last_emit = now;
                }

                // Emit periodic updates
                if !app_name.is_empty() && now - last_emit >= 1 {
                    let elapsed = now - session_start;
                    let _ = app_handle.emit(
                        "app-update",
                        CurrentSession {
                            app_name: app_name.clone(),
                            window_title: title.clone(),
                            exe_path: exe_path.clone(),
                            start_time: session_start,
                            elapsed_seconds: elapsed,
                        },
                    );
                    
                    // Also emit aggregated data for real-time updates
                    if let Ok(db_lock) = db.lock() {
                        if let Ok(agg) = database::get_aggregated_by_date(&db_lock, &today) {
                            let _ = app_handle.emit("today-aggregated", &agg);
                        }
                    }
                    
                    last_emit = now;
                }

                std::thread::sleep(Duration::from_secs(1));
            }
        });

        Self { running }
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

fn extract_app_name(exe_path: &str) -> String {
    if exe_path.is_empty() {
        return String::new();
    }
    let path = std::path::Path::new(exe_path);
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    if stem.is_empty() {
        return String::new();
    }

    let mut chars = stem.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
