//! Authentication API handlers.

use crate::auth::AuthStorage;
use crate::server::response::send_response;
use serde::Deserialize;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateCredentialsPayload {
    pub old_password: String,
    pub new_username: Option<String>,
    pub new_password: Option<String>,
}

pub fn handle_login(
    stream: &mut TcpStream,
    body: &[u8],
    auth_db: &Arc<Mutex<AuthStorage>>,
    auth_db_path: &str,
    client_ip: &str,
    user_agent: &str,
) -> std::io::Result<()> {
    let payload: LoginPayload = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let mut guard = auth_db.lock().unwrap();
    match guard.authenticate(&payload.username, &payload.password, client_ip, user_agent) {
        Ok(token) => {
            let _ = guard.save(auth_db_path);
            let username = guard.admin.username.clone();
            let resp = serde_json::json!({
                "status": "success",
                "token": token,
                "username": username
            });
            send_response(stream, 200, "application/json", resp.to_string().as_bytes())
        }
        Err(err_msg) => {
            let _ = guard.save(auth_db_path);
            let status_code = if err_msg.contains("locked") { 429 } else { 401 };
            let resp = serde_json::json!({
                "status": "error",
                "message": err_msg
            });
            send_response(
                stream,
                status_code,
                "application/json",
                resp.to_string().as_bytes(),
            )
        }
    }
}

pub fn handle_verify(
    stream: &mut TcpStream,
    auth_db: &Arc<Mutex<AuthStorage>>,
) -> std::io::Result<()> {
    let guard = auth_db.lock().unwrap();
    let username = guard.admin.username.clone();
    let resp = serde_json::json!({
        "authenticated": true,
        "username": username
    });
    send_response(stream, 200, "application/json", resp.to_string().as_bytes())
}

pub fn handle_logout(
    stream: &mut TcpStream,
    auth_db: &Arc<Mutex<AuthStorage>>,
    auth_db_path: &str,
    auth_token: &str,
) -> std::io::Result<()> {
    let mut guard = auth_db.lock().unwrap();
    guard.revoke_session(auth_token);
    let _ = guard.save(auth_db_path);
    send_response(
        stream,
        200,
        "application/json",
        b"{\"status\":\"logged_out\"}",
    )
}

pub fn handle_update_credentials(
    stream: &mut TcpStream,
    body: &[u8],
    auth_db: &Arc<Mutex<AuthStorage>>,
    auth_db_path: &str,
) -> std::io::Result<()> {
    let payload: UpdateCredentialsPayload = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let mut guard = auth_db.lock().unwrap();
    if !guard.verify_password(&payload.old_password) {
        return send_response(
            stream,
            400,
            "application/json",
            b"{\"error\":\"Current password incorrect\"}",
        );
    }

    if let Some(new_user) = payload.new_username {
        let trimmed = new_user.trim();
        if !trimmed.is_empty() {
            guard.update_username(trimmed);
        }
    }

    if let Some(new_pass) = payload.new_password {
        let trimmed = new_pass.trim();
        if !trimmed.is_empty() {
            if trimmed.len() < 6 {
                return send_response(
                    stream,
                    400,
                    "application/json",
                    b"{\"error\":\"New password must be at least 6 characters\"}",
                );
            }
            guard.update_password(trimmed);
        }
    }

    let _ = guard.save(auth_db_path);
    send_response(
        stream,
        200,
        "application/json",
        b"{\"status\":\"credentials_updated\"}",
    )
}

pub fn handle_logs(
    stream: &mut TcpStream,
    auth_db: &Arc<Mutex<AuthStorage>>,
) -> std::io::Result<()> {
    let guard = auth_db.lock().unwrap();
    let logs = guard.list_logs();
    let json = serde_json::to_vec(&logs).unwrap_or_default();
    send_response(stream, 200, "application/json", &json)
}
