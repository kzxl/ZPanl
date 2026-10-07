//! WAF & Threat Intelligence API request handlers.

use crate::server::response::send_response;
use crate::waf::{WafRule, WafStorage};
use serde::Deserialize;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
pub struct IpRulePayload {
    pub ip: String,
    pub is_blacklist: bool,
    pub action: String, // "add" or "remove"
}

#[derive(Deserialize)]
pub struct GeoBlockPayload {
    pub country_code: String,
}

#[derive(Deserialize)]
pub struct CustomRulePayload {
    pub name: String,
    pub target: String,
    pub operator: String,
    pub pattern: String,
    pub action: String,
}

#[derive(Deserialize)]
pub struct DeleteRulePayload {
    pub id: String,
}

pub fn handle_waf_overview(
    stream: &mut TcpStream,
    waf_db: &Arc<Mutex<WafStorage>>,
) -> std::io::Result<()> {
    let guard = waf_db.lock().unwrap();
    let summary = guard.get_summary();
    let json = serde_json::to_vec(&summary).unwrap_or_default();
    send_response(stream, 200, "application/json", &json)
}

pub fn handle_waf_rules(
    stream: &mut TcpStream,
    waf_db: &Arc<Mutex<WafStorage>>,
) -> std::io::Result<()> {
    let guard = waf_db.lock().unwrap();
    let resp = serde_json::json!({
        "blacklist_ips": guard.blacklist_ips,
        "whitelist_ips": guard.whitelist_ips,
        "blocked_countries": guard.blocked_countries,
        "custom_rules": guard.custom_rules
    });
    let json = serde_json::to_vec(&resp).unwrap_or_default();
    send_response(stream, 200, "application/json", &json)
}

pub fn handle_waf_ip_rule(
    stream: &mut TcpStream,
    body: &[u8],
    waf_db: &Arc<Mutex<WafStorage>>,
    waf_db_path: &str,
) -> std::io::Result<()> {
    let payload: IpRulePayload = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let mut guard = waf_db.lock().unwrap();
    let success = if payload.action == "remove" {
        guard.remove_ip(&payload.ip, payload.is_blacklist)
    } else {
        guard.add_ip(&payload.ip, payload.is_blacklist)
    };

    if success {
        let _ = guard.save(waf_db_path);
        send_response(stream, 200, "application/json", b"{\"status\":\"updated\"}")
    } else {
        send_response(stream, 400, "text/plain", b"Failed to update IP rule")
    }
}

pub fn handle_waf_geo_block(
    stream: &mut TcpStream,
    body: &[u8],
    waf_db: &Arc<Mutex<WafStorage>>,
    waf_db_path: &str,
) -> std::io::Result<()> {
    let payload: GeoBlockPayload = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let mut guard = waf_db.lock().unwrap();
    let blocked = guard.toggle_country(&payload.country_code);
    let _ = guard.save(waf_db_path);

    let resp = serde_json::json!({
        "status": "toggled",
        "blocked": blocked,
        "country": payload.country_code
    });
    let json = serde_json::to_vec(&resp).unwrap_or_default();
    send_response(stream, 200, "application/json", &json)
}

pub fn handle_waf_custom_rule(
    stream: &mut TcpStream,
    body: &[u8],
    waf_db: &Arc<Mutex<WafStorage>>,
    waf_db_path: &str,
) -> std::io::Result<()> {
    let payload: CustomRulePayload = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let rule = WafRule {
        id: format!("rule_{}", now),
        name: payload.name,
        target: payload.target,
        operator: payload.operator,
        pattern: payload.pattern,
        action: payload.action,
        enabled: true,
    };

    let mut guard = waf_db.lock().unwrap();
    guard.add_rule(rule);
    let _ = guard.save(waf_db_path);

    send_response(stream, 200, "application/json", b"{\"status\":\"created\"}")
}

pub fn handle_waf_delete_rule(
    stream: &mut TcpStream,
    body: &[u8],
    waf_db: &Arc<Mutex<WafStorage>>,
    waf_db_path: &str,
) -> std::io::Result<()> {
    let payload: DeleteRulePayload = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let mut guard = waf_db.lock().unwrap();
    if guard.delete_rule(&payload.id) {
        let _ = guard.save(waf_db_path);
        send_response(stream, 200, "application/json", b"{\"status\":\"deleted\"}")
    } else {
        send_response(stream, 404, "text/plain", b"Rule not found")
    }
}
