use crate::filemgr::FileManager;
use crate::services::ServiceManager;
use crate::site::{SiteDatabase, SiteKind, SiteRecord};
use crate::telemetry::TelemetryCollector;
use crate::ui::INDEX_HTML;
use serde::Deserialize;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use zero_sys::service::ServiceAction;

pub struct HttpServer {
    bind_addr: String,
    db: Arc<Mutex<SiteDatabase>>,
    db_path: String,
    telemetry: Arc<TelemetryCollector>,
}

#[derive(Deserialize)]
struct CreateSitePayload {
    domain: String,
    root_path: String,
    kind: SiteKind,
    php_version: Option<String>,
    #[allow(dead_code)]
    ssl_enabled: Option<bool>,
    aliases: Option<Vec<String>>,
    port: Option<u16>,
    running_dir: Option<String>,
    proxy_upstream: Option<String>,
    rewrite_preset: Option<String>,
}

#[derive(Deserialize)]
struct UpdateSitePayload {
    domain: String,
    aliases: Option<Vec<String>>,
    port: Option<u16>,
    root_path: Option<String>,
    running_dir: Option<String>,
    kind: Option<SiteKind>,
    php_version: Option<String>,
    proxy_upstream: Option<String>,
    rewrite_preset: Option<String>,
    maintenance: Option<bool>,
    ssl_enabled: Option<bool>,
    custom_caddy: Option<String>,
}

#[derive(Deserialize)]
struct ServiceActionPayload {
    name: String,
    action: String,
}

#[derive(Deserialize)]
struct FileSavePayload {
    site: String,
    path: String,
    content: String,
}

#[derive(Deserialize)]
struct FileDeletePayload {
    site: String,
    path: String,
}

#[derive(Deserialize)]
struct FileCreatePayload {
    site: String,
    path: String,
    is_dir: bool,
}

impl HttpServer {
    pub fn new(bind_addr: &str, db_path: &str) -> Self {
        let db = SiteDatabase::load_or_default(db_path);
        Self {
            bind_addr: bind_addr.to_string(),
            db: Arc::new(Mutex::new(db)),
            db_path: db_path.to_string(),
            telemetry: Arc::new(TelemetryCollector::new()),
        }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.bind_addr)?;
        println!("🚀 ZPanl Web Daemon listening at http://{}", self.bind_addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let db = Arc::clone(&self.db);
                    let db_path = self.db_path.clone();
                    let telemetry = Arc::clone(&self.telemetry);

                    thread::spawn(move || {
                        if let Err(e) = handle_connection(stream, db, &db_path, telemetry) {
                            eprintln!("Error handling connection: {e}");
                        }
                    });
                }
                Err(e) => eprintln!("TCP accept error: {e}"),
            }
        }

        Ok(())
    }
}

fn handle_connection(
    mut stream: TcpStream,
    db: Arc<Mutex<SiteDatabase>>,
    db_path: &str,
    telemetry: Arc<TelemetryCollector>,
) -> std::io::Result<()> {
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line)? == 0 {
        return Ok(());
    }

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return send_response(&mut stream, 400, "text/plain", b"Bad Request");
    }

    let method = parts[0];
    let full_path = parts[1];

    let (path, query) = match full_path.find('?') {
        Some(idx) => (&full_path[..idx], &full_path[idx + 1..]),
        None => (full_path, ""),
    };

    // Parse headers to get Content-Length
    let mut content_length = 0usize;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? <= 2 && (line == "\r\n" || line == "\n" || line.is_empty())
        {
            break;
        }
        let lower = line.to_lowercase();
        if lower.starts_with("content-length:") {
            if let Some(val) = line.split(':').nth(1) {
                content_length = val.trim().parse().unwrap_or(0);
            }
        }
    }

    // Read body
    let mut body = vec![0u8; content_length];
    if content_length > 0 {
        reader.read_exact(&mut body)?;
    }

    // CORS preflight
    if method == "OPTIONS" {
        return send_response(&mut stream, 200, "text/plain", b"OK");
    }

    // Route matching
    match (method, path) {
        ("GET", "/") => send_response(
            &mut stream,
            200,
            "text/html; charset=utf-8",
            INDEX_HTML.as_bytes(),
        ),

        ("GET", "/api/v1/telemetry") => {
            let stats = telemetry.sample();
            let json = serde_json::to_vec(&stats).unwrap_or_default();
            send_response(&mut stream, 200, "application/json", &json)
        }

        ("GET", "/api/v1/sites") => {
            let guard = db.lock().unwrap();
            let json = serde_json::to_vec(guard.list()).unwrap_or_default();
            send_response(&mut stream, 200, "application/json", &json)
        }

        ("POST", "/api/v1/sites") => {
            let payload: CreateSitePayload = match serde_json::from_slice(&body) {
                Ok(p) => p,
                Err(e) => {
                    return send_response(&mut stream, 400, "text/plain", e.to_string().as_bytes())
                }
            };

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);

            let record = SiteRecord {
                id: format!("site_{}", now),
                domain: payload.domain,
                aliases: payload.aliases.unwrap_or_default(),
                port: payload.port.unwrap_or(80),
                root_path: payload.root_path,
                running_dir: payload.running_dir,
                kind: payload.kind,
                php_version: payload.php_version,
                proxy_upstream: payload.proxy_upstream,
                rewrite_preset: payload.rewrite_preset,
                maintenance: false,
                ssl_enabled: true,
                custom_caddy: None,
                created_at: now,
            };

            let mut guard = db.lock().unwrap();
            match guard.add(record) {
                Ok(()) => {
                    let _ = guard.save(db_path);
                    send_response(
                        &mut stream,
                        200,
                        "application/json",
                        b"{\"status\":\"created\"}",
                    )
                }
                Err(err) => send_response(&mut stream, 400, "text/plain", err.as_bytes()),
            }
        }

        ("POST", "/api/v1/sites/update") | ("PUT", "/api/v1/sites") => {
            let payload: UpdateSitePayload = match serde_json::from_slice(&body) {
                Ok(p) => p,
                Err(e) => {
                    return send_response(&mut stream, 400, "text/plain", e.to_string().as_bytes())
                }
            };

            let mut guard = db.lock().unwrap();
            let mut record = match guard.find_by_domain(&payload.domain).cloned() {
                Some(r) => r,
                None => return send_response(&mut stream, 404, "text/plain", b"Site not found"),
            };

            if let Some(aliases) = payload.aliases {
                record.aliases = aliases;
            }
            if let Some(port) = payload.port {
                record.port = port;
            }
            if let Some(root_path) = payload.root_path {
                record.root_path = root_path;
            }
            if let Some(running_dir) = payload.running_dir {
                record.running_dir = if running_dir.trim().is_empty() {
                    None
                } else {
                    Some(running_dir)
                };
            }
            if let Some(kind) = payload.kind {
                record.kind = kind;
            }
            if let Some(php_version) = payload.php_version {
                record.php_version = if php_version.trim().is_empty() {
                    None
                } else {
                    Some(php_version)
                };
            }
            if let Some(proxy_upstream) = payload.proxy_upstream {
                record.proxy_upstream = if proxy_upstream.trim().is_empty() {
                    None
                } else {
                    Some(proxy_upstream)
                };
            }
            if let Some(rewrite_preset) = payload.rewrite_preset {
                record.rewrite_preset = if rewrite_preset.trim().is_empty() {
                    None
                } else {
                    Some(rewrite_preset)
                };
            }
            if let Some(maintenance) = payload.maintenance {
                record.maintenance = maintenance;
            }
            if let Some(ssl_enabled) = payload.ssl_enabled {
                record.ssl_enabled = ssl_enabled;
            }
            if let Some(custom_caddy) = payload.custom_caddy {
                record.custom_caddy = if custom_caddy.trim().is_empty() {
                    None
                } else {
                    Some(custom_caddy)
                };
            }

            let domain = payload.domain.clone();
            match guard.update(&domain, record) {
                Ok(()) => {
                    let _ = guard.save(db_path);
                    send_response(
                        &mut stream,
                        200,
                        "application/json",
                        b"{\"status\":\"updated\"}",
                    )
                }
                Err(err) => send_response(&mut stream, 400, "text/plain", err.as_bytes()),
            }
        }

        ("GET", "/api/v1/sites/vhost") => {
            let site_domain = parse_query_param(query, "domain").unwrap_or_default();
            let guard = db.lock().unwrap();
            match guard.find_by_domain(&site_domain) {
                Some(site) => {
                    let vhost_block = guard.generate_site_caddyfile(site);
                    send_response(
                        &mut stream,
                        200,
                        "text/plain; charset=utf-8",
                        vhost_block.as_bytes(),
                    )
                }
                None => send_response(&mut stream, 404, "text/plain", b"Site not found"),
            }
        }

        ("GET", "/api/v1/sites/detail") => {
            let site_domain = parse_query_param(query, "domain").unwrap_or_default();
            let guard = db.lock().unwrap();
            match guard.find_by_domain(&site_domain) {
                Some(site) => {
                    let json = serde_json::to_vec(site).unwrap_or_default();
                    send_response(&mut stream, 200, "application/json", &json)
                }
                None => send_response(&mut stream, 404, "text/plain", b"Site not found"),
            }
        }

        ("DELETE", "/api/v1/sites") => {
            let domain = parse_query_param(query, "domain").unwrap_or_default();
            let mut guard = db.lock().unwrap();
            match guard.remove(&domain) {
                Ok(_) => {
                    let _ = guard.save(db_path);
                    send_response(
                        &mut stream,
                        200,
                        "application/json",
                        b"{\"status\":\"deleted\"}",
                    )
                }
                Err(err) => send_response(&mut stream, 404, "text/plain", err.as_bytes()),
            }
        }

        ("GET", "/api/v1/services") => {
            let services = ServiceManager::list_services();
            let json = serde_json::to_vec(&services).unwrap_or_default();
            send_response(&mut stream, 200, "application/json", &json)
        }

        ("POST", "/api/v1/services/action") => {
            let payload: ServiceActionPayload = match serde_json::from_slice(&body) {
                Ok(p) => p,
                Err(e) => {
                    return send_response(&mut stream, 400, "text/plain", e.to_string().as_bytes())
                }
            };

            let action = match payload.action.to_lowercase().as_str() {
                "restart" => ServiceAction::Restart,
                "reload" => ServiceAction::Reload,
                "stop" => ServiceAction::Stop,
                "start" => ServiceAction::Start,
                _ => return send_response(&mut stream, 400, "text/plain", b"Invalid action"),
            };

            match ServiceManager::execute_action(&payload.name, action) {
                Ok(msg) => send_response(&mut stream, 200, "text/plain", msg.as_bytes()),
                Err(err) => send_response(&mut stream, 500, "text/plain", err.as_bytes()),
            }
        }

        ("GET", "/api/v1/caddy/caddyfile") => {
            let guard = db.lock().unwrap();
            let caddyfile = guard.generate_caddyfile();
            send_response(
                &mut stream,
                200,
                "text/plain; charset=utf-8",
                caddyfile.as_bytes(),
            )
        }

        ("GET", "/api/v1/php/pool") => {
            let site_domain = parse_query_param(query, "domain").unwrap_or_default();
            let guard = db.lock().unwrap();
            match guard.generate_php_pool(&site_domain) {
                Some(ini) => send_response(
                    &mut stream,
                    200,
                    "text/plain; charset=utf-8",
                    ini.as_bytes(),
                ),
                None => send_response(
                    &mut stream,
                    404,
                    "text/plain",
                    b"PHP Pool not found or site is not PHP",
                ),
            }
        }

        ("GET", "/api/v1/files/list") => {
            let site_domain = parse_query_param(query, "site").unwrap_or_default();
            let subpath = parse_query_param(query, "path").unwrap_or_default();

            let guard = db.lock().unwrap();
            let site = match guard.find_by_domain(&site_domain) {
                Some(s) => s,
                None => return send_response(&mut stream, 404, "text/plain", b"Site not found"),
            };
            let root = site.root_path.clone();
            drop(guard);

            match FileManager::list_dir(&root, &subpath) {
                Ok(entries) => {
                    let json = serde_json::to_vec(&entries).unwrap_or_default();
                    send_response(&mut stream, 200, "application/json", &json)
                }
                Err(err) => send_response(&mut stream, 400, "text/plain", err.as_bytes()),
            }
        }

        ("GET", "/api/v1/files/read") => {
            let site_domain = parse_query_param(query, "site").unwrap_or_default();
            let subpath = parse_query_param(query, "path").unwrap_or_default();

            let guard = db.lock().unwrap();
            let site = match guard.find_by_domain(&site_domain) {
                Some(s) => s,
                None => return send_response(&mut stream, 404, "text/plain", b"Site not found"),
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
                    send_response(&mut stream, 200, "application/json", &json)
                }
                Err(err) => send_response(&mut stream, 400, "text/plain", err.as_bytes()),
            }
        }

        ("POST", "/api/v1/files/save") => {
            let payload: FileSavePayload = match serde_json::from_slice(&body) {
                Ok(p) => p,
                Err(e) => {
                    return send_response(&mut stream, 400, "text/plain", e.to_string().as_bytes())
                }
            };

            let guard = db.lock().unwrap();
            let site = match guard.find_by_domain(&payload.site) {
                Some(s) => s,
                None => return send_response(&mut stream, 404, "text/plain", b"Site not found"),
            };
            let root = site.root_path.clone();
            drop(guard);

            match FileManager::save_file(&root, &payload.path, &payload.content) {
                Ok(()) => send_response(
                    &mut stream,
                    200,
                    "application/json",
                    b"{\"status\":\"saved\"}",
                ),
                Err(err) => send_response(&mut stream, 400, "text/plain", err.as_bytes()),
            }
        }

        ("POST", "/api/v1/files/delete") => {
            let payload: FileDeletePayload = match serde_json::from_slice(&body) {
                Ok(p) => p,
                Err(e) => {
                    return send_response(&mut stream, 400, "text/plain", e.to_string().as_bytes())
                }
            };

            let guard = db.lock().unwrap();
            let site = match guard.find_by_domain(&payload.site) {
                Some(s) => s,
                None => return send_response(&mut stream, 404, "text/plain", b"Site not found"),
            };
            let root = site.root_path.clone();
            drop(guard);

            match FileManager::delete_entry(&root, &payload.path) {
                Ok(()) => send_response(
                    &mut stream,
                    200,
                    "application/json",
                    b"{\"status\":\"deleted\"}",
                ),
                Err(err) => send_response(&mut stream, 400, "text/plain", err.as_bytes()),
            }
        }

        ("POST", "/api/v1/files/create") => {
            let payload: FileCreatePayload = match serde_json::from_slice(&body) {
                Ok(p) => p,
                Err(e) => {
                    return send_response(&mut stream, 400, "text/plain", e.to_string().as_bytes())
                }
            };

            let guard = db.lock().unwrap();
            let site = match guard.find_by_domain(&payload.site) {
                Some(s) => s,
                None => return send_response(&mut stream, 404, "text/plain", b"Site not found"),
            };
            let root = site.root_path.clone();
            drop(guard);

            match FileManager::create_entry(&root, &payload.path, payload.is_dir) {
                Ok(()) => send_response(
                    &mut stream,
                    200,
                    "application/json",
                    b"{\"status\":\"created\"}",
                ),
                Err(err) => send_response(&mut stream, 400, "text/plain", err.as_bytes()),
            }
        }

        _ => send_response(&mut stream, 404, "text/plain", b"Not Found"),
    }
}

fn send_response(
    stream: &mut TcpStream,
    status_code: u16,
    content_type: &str,
    body: &[u8],
) -> std::io::Result<()> {
    let status_line = match status_code {
        200 => "HTTP/1.1 200 OK",
        400 => "HTTP/1.1 400 Bad Request",
        403 => "HTTP/1.1 403 Forbidden",
        404 => "HTTP/1.1 404 Not Found",
        500 => "HTTP/1.1 500 Internal Server Error",
        _ => "HTTP/1.1 200 OK",
    };

    let header = format!(
        "{status_line}\r\n\
         Content-Type: {content_type}\r\n\
         Content-Length: {}\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Access-Control-Allow-Methods: GET, POST, DELETE, OPTIONS\r\n\
         Access-Control-Allow-Headers: Content-Type\r\n\
         Connection: close\r\n\r\n",
        body.len()
    );

    stream.write_all(header.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()?;
    Ok(())
}

fn parse_query_param(query: &str, key: &str) -> Option<String> {
    for pair in query.split('&') {
        let mut parts = pair.split('=');
        if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
            if k == key {
                // Quick URL decode for %20, %2F etc.
                return Some(url_decode(v));
            }
        }
    }
    None
}

fn url_decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(val) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(val as char);
                i += 3;
                continue;
            }
        } else if bytes[i] == b'+' {
            out.push(' ');
            i += 1;
            continue;
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}
