//! Git-Ops deployment and webhook API handlers.

use crate::deploy::DeployStorage;
use crate::server::response::{parse_query_param, send_response};
use crate::site::SiteDatabase;
use serde::Deserialize;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
pub struct TriggerDeployPayload {
    pub domain: String,
}

#[derive(Deserialize)]
pub struct RollbackDeployPayload {
    pub domain: String,
    pub release_id: String,
}

pub fn handle_get_deploy_config(
    stream: &mut TcpStream,
    query: &str,
    deploy_db: &Arc<Mutex<DeployStorage>>,
) -> std::io::Result<()> {
    let domain = parse_query_param(query, "domain").unwrap_or_default();
    let guard = deploy_db.lock().unwrap();
    let cfg = guard
        .get_config(&domain)
        .cloned()
        .unwrap_or_else(|| crate::deploy::DeployConfig {
            domain: domain.clone(),
            ..Default::default()
        });
    let json = serde_json::to_vec(&cfg).unwrap_or_default();
    send_response(stream, 200, "application/json", &json)
}

pub fn handle_save_deploy_config(
    stream: &mut TcpStream,
    body: &[u8],
    deploy_db: &Arc<Mutex<DeployStorage>>,
    deploy_db_path: &str,
) -> std::io::Result<()> {
    let config: crate::deploy::DeployConfig = match serde_json::from_slice(body) {
        Ok(c) => c,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let mut guard = deploy_db.lock().unwrap();
    guard.save_config(config);
    let _ = guard.save(deploy_db_path);
    send_response(stream, 200, "application/json", b"{\"status\":\"saved\"}")
}

pub fn handle_trigger_deploy(
    stream: &mut TcpStream,
    body: &[u8],
    db: &Arc<Mutex<SiteDatabase>>,
    deploy_db: &Arc<Mutex<DeployStorage>>,
    deploy_db_path: &str,
) -> std::io::Result<()> {
    let payload: TriggerDeployPayload = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let site_root = {
        let guard = db.lock().unwrap();
        match guard.find_by_domain(&payload.domain) {
            Some(s) => s.root_path.clone(),
            None => return send_response(stream, 404, "text/plain", b"Site not found"),
        }
    };

    let mut guard = deploy_db.lock().unwrap();
    match guard.trigger_deploy(&payload.domain, &site_root, "manual") {
        Ok(release) => {
            let _ = guard.save(deploy_db_path);
            let json = serde_json::to_vec(&release).unwrap_or_default();
            send_response(stream, 200, "application/json", &json)
        }
        Err(err) => {
            let _ = guard.save(deploy_db_path);
            send_response(stream, 500, "text/plain", err.as_bytes())
        }
    }
}

pub fn handle_deploy_history(
    stream: &mut TcpStream,
    query: &str,
    deploy_db: &Arc<Mutex<DeployStorage>>,
) -> std::io::Result<()> {
    let domain = parse_query_param(query, "domain").unwrap_or_default();
    let guard = deploy_db.lock().unwrap();
    let history = guard.list_history(&domain);
    let json = serde_json::to_vec(&history).unwrap_or_default();
    send_response(stream, 200, "application/json", &json)
}

pub fn handle_rollback_deploy(
    stream: &mut TcpStream,
    body: &[u8],
    db: &Arc<Mutex<SiteDatabase>>,
    deploy_db: &Arc<Mutex<DeployStorage>>,
    deploy_db_path: &str,
) -> std::io::Result<()> {
    let payload: RollbackDeployPayload = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let site_root = {
        let guard = db.lock().unwrap();
        match guard.find_by_domain(&payload.domain) {
            Some(s) => s.root_path.clone(),
            None => return send_response(stream, 404, "text/plain", b"Site not found"),
        }
    };

    let mut guard = deploy_db.lock().unwrap();
    match guard.rollback(&payload.domain, &site_root, &payload.release_id) {
        Ok(release) => {
            let _ = guard.save(deploy_db_path);
            let json = serde_json::to_vec(&release).unwrap_or_default();
            send_response(stream, 200, "application/json", &json)
        }
        Err(err) => send_response(stream, 500, "text/plain", err.as_bytes()),
    }
}

pub fn handle_webhook_deploy(
    stream: &mut TcpStream,
    query: &str,
    db: &Arc<Mutex<SiteDatabase>>,
    deploy_db: &Arc<Mutex<DeployStorage>>,
    deploy_db_path: &str,
) -> std::io::Result<()> {
    let domain = parse_query_param(query, "domain").unwrap_or_default();
    let site_root = {
        let guard = db.lock().unwrap();
        match guard.find_by_domain(&domain) {
            Some(s) => s.root_path.clone(),
            None => return send_response(stream, 404, "text/plain", b"Site not found"),
        }
    };

    let mut guard = deploy_db.lock().unwrap();
    match guard.trigger_deploy(&domain, &site_root, "webhook") {
        Ok(release) => {
            let _ = guard.save(deploy_db_path);
            let json = serde_json::to_vec(&release).unwrap_or_default();
            send_response(stream, 200, "application/json", &json)
        }
        Err(err) => {
            let _ = guard.save(deploy_db_path);
            send_response(stream, 500, "text/plain", err.as_bytes())
        }
    }
}
