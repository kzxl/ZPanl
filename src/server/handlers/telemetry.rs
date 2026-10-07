//! System telemetry and metrics sampling handler.

use crate::server::response::send_response;
use crate::telemetry::TelemetryCollector;
use std::net::TcpStream;
use std::sync::Arc;

pub fn handle_telemetry(
    stream: &mut TcpStream,
    telemetry: &Arc<TelemetryCollector>,
) -> std::io::Result<()> {
    let stats = telemetry.sample();
    let json = serde_json::to_vec(&stats).unwrap_or_default();
    send_response(stream, 200, "application/json", &json)
}
