//! Database management API handlers.

use crate::database::{DatabaseRecord, DatabaseStorage};
use crate::server::response::{parse_query_param, send_response};
use serde::Deserialize;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
pub struct CreateDatabasePayload {
    pub name: String,
    pub engine: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
    pub collation: Option<String>,
    pub site: Option<String>,
}

pub fn handle_list_databases(
    stream: &mut TcpStream,
    database_db: &Arc<Mutex<DatabaseStorage>>,
) -> std::io::Result<()> {
    let guard = database_db.lock().unwrap();
    let json = serde_json::to_vec(guard.list()).unwrap_or_default();
    send_response(stream, 200, "application/json", &json)
}

pub fn handle_create_database(
    stream: &mut TcpStream,
    body: &[u8],
    database_db: &Arc<Mutex<DatabaseStorage>>,
    database_db_path: &str,
) -> std::io::Result<()> {
    let payload: CreateDatabasePayload = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let record = DatabaseRecord {
        id: format!("db_{}", now),
        name: payload.name,
        engine: payload.engine.unwrap_or_else(|| "mysql".to_string()),
        username: payload.username.unwrap_or_else(|| "root".to_string()),
        password: payload.password,
        host: payload.host.unwrap_or_else(|| "127.0.0.1".to_string()),
        collation: payload
            .collation
            .unwrap_or_else(|| "utf8mb4_unicode_ci".to_string()),
        site: payload.site,
        size_bytes: 1024 * 16,
        created_at: now,
    };

    let mut guard = database_db.lock().unwrap();
    match guard.add(record) {
        Ok(()) => {
            let _ = guard.save(database_db_path);
            send_response(stream, 200, "application/json", b"{\"status\":\"created\"}")
        }
        Err(err) => send_response(stream, 400, "text/plain", err.as_bytes()),
    }
}

pub fn handle_delete_database(
    stream: &mut TcpStream,
    query: &str,
    database_db: &Arc<Mutex<DatabaseStorage>>,
    database_db_path: &str,
) -> std::io::Result<()> {
    let name = match parse_query_param(query, "name") {
        Some(n) => n,
        None => return send_response(stream, 400, "text/plain", b"Missing 'name' parameter"),
    };

    let mut guard = database_db.lock().unwrap();
    if guard.delete(&name) {
        let _ = guard.save(database_db_path);
        send_response(stream, 200, "application/json", b"{\"status\":\"deleted\"}")
    } else {
        send_response(stream, 404, "text/plain", b"Database not found")
    }
}

pub fn handle_backup_database(
    stream: &mut TcpStream,
    query: &str,
    database_db: &Arc<Mutex<DatabaseStorage>>,
) -> std::io::Result<()> {
    let name = match parse_query_param(query, "name") {
        Some(n) => n,
        None => return send_response(stream, 400, "text/plain", b"Missing 'name' parameter"),
    };

    let guard = database_db.lock().unwrap();
    match guard.generate_dump(&name) {
        Ok(dump) => send_response(
            stream,
            200,
            "application/sql; charset=utf-8",
            dump.as_bytes(),
        ),
        Err(err) => send_response(stream, 404, "text/plain", err.as_bytes()),
    }
}
