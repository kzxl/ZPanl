//! File manager API handlers.

use crate::filemgr::FileManager;
use crate::server::response::{parse_query_param, send_response};
use crate::site::SiteDatabase;
use serde::Deserialize;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
pub struct FileSavePayload {
    pub site: String,
    pub path: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct FileDeletePayload {
    pub site: String,
    pub path: String,
}

#[derive(Deserialize)]
pub struct FileCreatePayload {
    pub site: String,
    pub path: String,
    pub is_dir: bool,
}

pub fn handle_list_files(
    stream: &mut TcpStream,
    query: &str,
    db: &Arc<Mutex<SiteDatabase>>,
) -> std::io::Result<()> {
    let site_domain = parse_query_param(query, "site").unwrap_or_default();
    let subpath = parse_query_param(query, "path").unwrap_or_default();

    let guard = db.lock().unwrap();
    let site = match guard.find_by_domain(&site_domain) {
        Some(s) => s,
        None => return send_response(stream, 404, "text/plain", b"Site not found"),
    };
    let root = site.root_path.clone();
    drop(guard);

    match FileManager::list_dir(&root, &subpath) {
        Ok(entries) => {
            let json = serde_json::to_vec(&entries).unwrap_or_default();
            send_response(stream, 200, "application/json", &json)
        }
        Err(err) => send_response(stream, 400, "text/plain", err.as_bytes()),
    }
}

pub fn handle_read_file(
    stream: &mut TcpStream,
    query: &str,
    db: &Arc<Mutex<SiteDatabase>>,
) -> std::io::Result<()> {
    let site_domain = parse_query_param(query, "site").unwrap_or_default();
    let subpath = parse_query_param(query, "path").unwrap_or_default();

    let guard = db.lock().unwrap();
    let site = match guard.find_by_domain(&site_domain) {
        Some(s) => s,
        None => return send_response(stream, 404, "text/plain", b"Site not found"),
    };
    let root = site.root_path.clone();
    drop(guard);

    match FileManager::read_file(&root, &subpath) {
        Ok((content, mime)) => {
            let resp = serde_json::json!({
                "content": content,
                "mime": mime
            });
            let json = serde_json::to_vec(&resp).unwrap_or_default();
            send_response(stream, 200, "application/json", &json)
        }
        Err(err) => send_response(stream, 400, "text/plain", err.as_bytes()),
    }
}

pub fn handle_save_file(
    stream: &mut TcpStream,
    body: &[u8],
    db: &Arc<Mutex<SiteDatabase>>,
) -> std::io::Result<()> {
    let payload: FileSavePayload = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let guard = db.lock().unwrap();
    let site = match guard.find_by_domain(&payload.site) {
        Some(s) => s,
        None => return send_response(stream, 404, "text/plain", b"Site not found"),
    };
    let root = site.root_path.clone();
    drop(guard);

    match FileManager::save_file(&root, &payload.path, &payload.content) {
        Ok(()) => send_response(stream, 200, "application/json", b"{\"status\":\"saved\"}"),
        Err(err) => send_response(stream, 400, "text/plain", err.as_bytes()),
    }
}

pub fn handle_delete_file(
    stream: &mut TcpStream,
    body: &[u8],
    db: &Arc<Mutex<SiteDatabase>>,
) -> std::io::Result<()> {
    let payload: FileDeletePayload = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let guard = db.lock().unwrap();
    let site = match guard.find_by_domain(&payload.site) {
        Some(s) => s,
        None => return send_response(stream, 404, "text/plain", b"Site not found"),
    };
    let root = site.root_path.clone();
    drop(guard);

    match FileManager::delete_entry(&root, &payload.path) {
        Ok(()) => send_response(stream, 200, "application/json", b"{\"status\":\"deleted\"}"),
        Err(err) => send_response(stream, 400, "text/plain", err.as_bytes()),
    }
}

pub fn handle_create_file(
    stream: &mut TcpStream,
    body: &[u8],
    db: &Arc<Mutex<SiteDatabase>>,
) -> std::io::Result<()> {
    let payload: FileCreatePayload = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => return send_response(stream, 400, "text/plain", e.to_string().as_bytes()),
    };

    let guard = db.lock().unwrap();
    let site = match guard.find_by_domain(&payload.site) {
        Some(s) => s,
        None => return send_response(stream, 404, "text/plain", b"Site not found"),
    };
    let root = site.root_path.clone();
    drop(guard);

    match FileManager::create_entry(&root, &payload.path, payload.is_dir) {
        Ok(()) => send_response(stream, 200, "application/json", b"{\"status\":\"created\"}"),
        Err(err) => send_response(stream, 400, "text/plain", err.as_bytes()),
    }
}

pub fn handle_upload_file(
    stream: &mut TcpStream,
    query: &str,
    body: &[u8],
    db: &Arc<Mutex<SiteDatabase>>,
) -> std::io::Result<()> {
    let site_domain = match parse_query_param(query, "site") {
        Some(s) if !s.trim().is_empty() => s,
        _ => return send_response(stream, 400, "text/plain", b"Missing 'site' parameter"),
    };
    let filename = match parse_query_param(query, "filename") {
        Some(f) if !f.trim().is_empty() => f,
        _ => return send_response(stream, 400, "text/plain", b"Missing 'filename' parameter"),
    };
    let subpath = parse_query_param(query, "path").unwrap_or_default();

    let guard = db.lock().unwrap();
    let site = match guard.find_by_domain(&site_domain) {
        Some(s) => s,
        None => return send_response(stream, 404, "text/plain", b"Site not found"),
    };
    let root = site.root_path.clone();
    drop(guard);

    let clean_subpath = subpath.trim().trim_matches('/');
    let clean_filename = filename.trim().trim_matches('/');
    let full_rel_path = if clean_subpath.is_empty() {
        clean_filename.to_string()
    } else {
        format!("{clean_subpath}/{clean_filename}")
    };

    match FileManager::write_file_bytes(&root, &full_rel_path, body) {
        Ok(()) => {
            let resp = serde_json::json!({
                "status": "uploaded",
                "path": full_rel_path,
                "bytes": body.len()
            });
            let json = serde_json::to_vec(&resp).unwrap_or_default();
            send_response(stream, 200, "application/json", &json)
        }
        Err(err) => send_response(stream, 400, "text/plain", err.as_bytes()),
    }
}
