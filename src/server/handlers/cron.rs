//! Scheduled tasks and crontab API handlers.

use crate::cron::{CronDatabase, CronJob};
use crate::server::response::{parse_query_param, send_response};
use serde::Deserialize;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
pub struct CreateCronPayload {
    pub name: String,
    pub schedule: String,
    pub command: String,
    pub site: Option<String>,
}

#[derive(Deserialize)]
pub struct CronActionPayload {
    pub id: String,
}

pub fn handle_list_cron(
    stream: &mut TcpStream,
    cron_db: &Arc<Mutex<CronDatabase>>,
) -> std::io::Result<()> {
    let guard = cron_db.lock().unwrap();
    let json = serde_json::to_vec(guard.list()).unwrap_or_default();
    send_response(stream, 200, "application/json", &json)
}

pub fn handle_create_cron(
    stream: &mut TcpStream,
    body: &[u8],
    cron_db: &Arc<Mutex<CronDatabase>>,
    cron_db_path: &str,
) -> std::io::Result<()> {
    let payload: CreateCronPayload = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let job = CronJob {
        id: format!("cron_{}", now),
        name: payload.name,
        schedule: payload.schedule,
        command: payload.command,
        site: payload.site,
        enabled: true,
        created_at: now,
        last_run_at: None,
        last_status: None,
        last_output: None,
    };

    let mut guard = cron_db.lock().unwrap();
    match guard.add(job) {
        Ok(()) => {
            let _ = guard.save(cron_db_path);
            send_response(stream, 200, "application/json", b"{\"status\":\"created\"}")
        }
        Err(err) => send_response(stream, 400, "text/plain", err.as_bytes()),
    }
}

pub fn handle_run_cron(
    stream: &mut TcpStream,
    body: &[u8],
    cron_db: &Arc<Mutex<CronDatabase>>,
    cron_db_path: &str,
) -> std::io::Result<()> {
    let payload: CronActionPayload = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let mut guard = cron_db.lock().unwrap();
    match guard.execute_job(&payload.id) {
        Ok(output) => {
            let _ = guard.save(cron_db_path);
            let resp = serde_json::json!({
                "status": "success",
                "output": output
            });
            let json = serde_json::to_vec(&resp).unwrap_or_default();
            send_response(stream, 200, "application/json", &json)
        }
        Err(err) => {
            let _ = guard.save(cron_db_path);
            send_response(stream, 500, "text/plain", err.as_bytes())
        }
    }
}

pub fn handle_toggle_cron(
    stream: &mut TcpStream,
    body: &[u8],
    cron_db: &Arc<Mutex<CronDatabase>>,
    cron_db_path: &str,
) -> std::io::Result<()> {
    let payload: CronActionPayload = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let mut guard = cron_db.lock().unwrap();
    match guard.toggle(&payload.id) {
        Some(enabled) => {
            let _ = guard.save(cron_db_path);
            let resp = serde_json::json!({ "enabled": enabled });
            let json = serde_json::to_vec(&resp).unwrap_or_default();
            send_response(stream, 200, "application/json", &json)
        }
        None => send_response(stream, 404, "text/plain", b"Job not found"),
    }
}

pub fn handle_delete_cron(
    stream: &mut TcpStream,
    query: &str,
    cron_db: &Arc<Mutex<CronDatabase>>,
    cron_db_path: &str,
) -> std::io::Result<()> {
    let id = match parse_query_param(query, "id") {
        Some(i) => i,
        None => return send_response(stream, 400, "text/plain", b"Missing 'id' parameter"),
    };

    let mut guard = cron_db.lock().unwrap();
    if guard.delete(&id) {
        let _ = guard.save(cron_db_path);
        send_response(stream, 200, "application/json", b"{\"status\":\"deleted\"}")
    } else {
        send_response(stream, 404, "text/plain", b"Job not found")
    }
}

pub fn handle_cron_logs(
    stream: &mut TcpStream,
    query: &str,
    cron_db: &Arc<Mutex<CronDatabase>>,
) -> std::io::Result<()> {
    let id = match parse_query_param(query, "id") {
        Some(i) => i,
        None => return send_response(stream, 400, "text/plain", b"Missing 'id' parameter"),
    };

    let guard = cron_db.lock().unwrap();
    match guard.get_logs(&id) {
        Some(logs) => send_response(stream, 200, "text/plain; charset=utf-8", logs.as_bytes()),
        None => send_response(
            stream,
            200,
            "text/plain; charset=utf-8",
            b"No logs recorded yet for this task.",
        ),
    }
}
