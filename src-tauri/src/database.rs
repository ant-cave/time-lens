use rusqlite::{params, Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    pub id: i64,
    pub app_name: String,
    pub window_title: String,
    pub exe_path: String,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub duration_seconds: i64,
    pub date: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AggregatedUsage {
    pub app_name: String,
    pub total_seconds: i64,
}

pub fn init(db_path: &Path) -> SqlResult<Connection> {
    let conn = Connection::open(db_path)?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS usage_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            app_name TEXT NOT NULL,
            window_title TEXT NOT NULL,
            exe_path TEXT NOT NULL DEFAULT '',
            start_time INTEGER NOT NULL,
            end_time INTEGER,
            duration_seconds INTEGER NOT NULL DEFAULT 0,
            date TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_usage_records_date ON usage_records(date);
        CREATE INDEX IF NOT EXISTS idx_usage_records_app ON usage_records(app_name);",
    )?;

    Ok(conn)
}

pub fn insert_record(conn: &Connection, record: &UsageRecord) -> SqlResult<i64> {
    conn.execute(
        "INSERT INTO usage_records (app_name, window_title, exe_path, start_time, end_time, duration_seconds, date)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            record.app_name,
            record.window_title,
            record.exe_path,
            record.start_time,
            record.end_time,
            record.duration_seconds,
            record.date,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_records_by_date(conn: &Connection, date: &str) -> SqlResult<Vec<UsageRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, app_name, window_title, exe_path, start_time, end_time, duration_seconds, date
         FROM usage_records WHERE date = ?1 ORDER BY start_time DESC",
    )?;

    let records = stmt
        .query_map(params![date], |row| {
            Ok(UsageRecord {
                id: row.get(0)?,
                app_name: row.get(1)?,
                window_title: row.get(2)?,
                exe_path: row.get(3)?,
                start_time: row.get(4)?,
                end_time: row.get(5)?,
                duration_seconds: row.get(6)?,
                date: row.get(7)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(records)
}

pub fn get_aggregated_by_date(conn: &Connection, date: &str) -> SqlResult<Vec<AggregatedUsage>> {
    let mut stmt = conn.prepare(
        "SELECT app_name, SUM(duration_seconds) as total_seconds
         FROM usage_records WHERE date = ?1
         GROUP BY app_name ORDER BY total_seconds DESC",
    )?;

    let records = stmt
        .query_map(params![date], |row| {
            Ok(AggregatedUsage {
                app_name: row.get(0)?,
                total_seconds: row.get(1)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(records)
}

pub fn get_records_by_date_range(
    conn: &Connection,
    start_date: &str,
    end_date: &str,
) -> SqlResult<Vec<UsageRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, app_name, window_title, exe_path, start_time, end_time, duration_seconds, date
         FROM usage_records WHERE date >= ?1 AND date <= ?2 ORDER BY start_time DESC",
    )?;

    let records = stmt
        .query_map(params![start_date, end_date], |row| {
            Ok(UsageRecord {
                id: row.get(0)?,
                app_name: row.get(1)?,
                window_title: row.get(2)?,
                exe_path: row.get(3)?,
                start_time: row.get(4)?,
                end_time: row.get(5)?,
                duration_seconds: row.get(6)?,
                date: row.get(7)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(records)
}

pub fn cleanup_old_records(conn: &Connection, older_than_days: i64) -> SqlResult<usize> {
    let deleted = conn.execute(
        "DELETE FROM usage_records WHERE date < date('now', ?1)",
        params![format!("-{} days", older_than_days)],
    )?;
    Ok(deleted)
}
