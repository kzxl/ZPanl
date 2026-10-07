//! Systemd / Daemon service management API handlers.

use crate::server::response::send_response;
use crate::services::ServiceManager;
use serde::Deserialize;
use std::net::TcpStream;
use zero_sys::service::ServiceAction;

#[derive(Deserialize)]
pub struct ServiceActionPayload {
    pub name: String,
    pub action: String,
}

pub fn handle_list_services(stream: &mut TcpStream) -> std::io::Result<()> {
    let services = ServiceManager::list_services();
    let json = serde_json::to_vec(&services).unwrap_or_default();
    send_response(stream, 200, "application/json", &json)
}

pub fn handle_service_action(stream: &mut TcpStream, body: &[u8]) -> std::io::Result<()> {
    let payload: ServiceActionPayload = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let action = match payload.action.to_lowercase().as_str() {
        "restart" => ServiceAction::Restart,
        "reload" => ServiceAction::Reload,
        "stop" => ServiceAction::Stop,
        "start" => ServiceAction::Start,
        _ => return send_response(stream, 400, "text/plain", b"Invalid action"),
    };

    match ServiceManager::execute_action(&payload.name, action) {
        Ok(msg) => send_response(stream, 200, "text/plain", msg.as_bytes()),
        Err(err) => send_response(stream, 500, "text/plain", err.as_bytes()),
    }
}
