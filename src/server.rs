//! # High Performance Native HTTP Server
//!
//! Pure Rust HTTP 1.1 server handling API routes, authentication gates,
//! and static asset delivery with zero external server dependencies.

pub mod handlers;
pub mod response;

use crate::auth::AuthStorage;
use crate::cron::CronDatabase;
use crate::database::DatabaseStorage;
use crate::deploy::DeployStorage;
use crate::site::SiteDatabase;
use crate::telemetry::TelemetryCollector;
use crate::ui::{APP_JS, INDEX_HTML, STYLE_CSS};
use crate::waf::WafStorage;
use response::{send_response, send_response_with_headers};
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct HttpServer {
    bind_addr: String,
    db: Arc<Mutex<SiteDatabase>>,
    db_path: String,
    cron_db: Arc<Mutex<CronDatabase>>,
    cron_db_path: String,
    database_db: Arc<Mutex<DatabaseStorage>>,
    database_db_path: String,
    deploy_db: Arc<Mutex<DeployStorage>>,
    deploy_db_path: String,
    auth_db: Arc<Mutex<AuthStorage>>,
    auth_db_path: String,
    waf_db: Arc<Mutex<WafStorage>>,
    waf_db_path: String,
    telemetry: Arc<TelemetryCollector>,
}

impl HttpServer {
    pub fn new(bind_addr: &str, db_path: &str) -> Self {
        let db = SiteDatabase::load_or_default(db_path);
        let cron_path = format!("{}_cron.json", db_path.trim_end_matches(".json"));
        let database_path = format!("{}_databases.json", db_path.trim_end_matches(".json"));
        let deploy_path = format!("{}_deploy.json", db_path.trim_end_matches(".json"));
        let auth_path = format!("{}_auth.json", db_path.trim_end_matches(".json"));
        let waf_path = format!("{}_waf.json", db_path.trim_end_matches(".json"));
        let cron_db = CronDatabase::load_or_default(&cron_path);
        let database_db = DatabaseStorage::load_or_default(&database_path);
        let deploy_db = DeployStorage::load_or_default(&deploy_path);
        let (auth_db, _initial_pass) = AuthStorage::load_or_init(&auth_path);
        let waf_db = WafStorage::load_or_init(&waf_path);

        Self {
            bind_addr: bind_addr.to_string(),
            db: Arc::new(Mutex::new(db)),
            db_path: db_path.to_string(),
            cron_db: Arc::new(Mutex::new(cron_db)),
            cron_db_path: cron_path,
            database_db: Arc::new(Mutex::new(database_db)),
            database_db_path: database_path,
            deploy_db: Arc::new(Mutex::new(deploy_db)),
            deploy_db_path: deploy_path,
            auth_db: Arc::new(Mutex::new(auth_db)),
            auth_db_path: auth_path,
            waf_db: Arc::new(Mutex::new(waf_db)),
            waf_db_path: waf_path,
            telemetry: Arc::new(TelemetryCollector::new()),
        }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.bind_addr)?;
        println!(
            "🚀 ZPanl Control Panel running on http://{}",
            self.bind_addr
        );

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let db = Arc::clone(&self.db);
                    let db_path = self.db_path.clone();
                    let cron_db = Arc::clone(&self.cron_db);
                    let cron_db_path = self.cron_db_path.clone();
                    let database_db = Arc::clone(&self.database_db);
                    let database_db_path = self.database_db_path.clone();
                    let deploy_db = Arc::clone(&self.deploy_db);
                    let deploy_db_path = self.deploy_db_path.clone();
                    let auth_db = Arc::clone(&self.auth_db);
                    let auth_db_path = self.auth_db_path.clone();
                    let waf_db = Arc::clone(&self.waf_db);
                    let waf_db_path = self.waf_db_path.clone();
                    let telemetry = Arc::clone(&self.telemetry);

                    thread::spawn(move || {
                        if let Err(e) = handle_connection(
                            stream,
                            db,
                            &db_path,
                            cron_db,
                            &cron_db_path,
                            database_db,
                            &database_db_path,
                            deploy_db,
                            &deploy_db_path,
                            auth_db,
                            &auth_db_path,
                            waf_db,
                            &waf_db_path,
                            telemetry,
                        ) {
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

#[allow(clippy::too_many_arguments)]
fn handle_connection(
    mut stream: TcpStream,
    db: Arc<Mutex<SiteDatabase>>,
    db_path: &str,
    cron_db: Arc<Mutex<CronDatabase>>,
    cron_db_path: &str,
    database_db: Arc<Mutex<DatabaseStorage>>,
    database_db_path: &str,
    deploy_db: Arc<Mutex<DeployStorage>>,
    deploy_db_path: &str,
    auth_db: Arc<Mutex<AuthStorage>>,
    auth_db_path: &str,
    waf_db: Arc<Mutex<WafStorage>>,
    waf_db_path: &str,
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

    // Parse headers to get Content-Length, Authorization, Cookie, User-Agent, X-Forwarded-For
    let peer_ip = stream
        .peer_addr()
        .map(|a| a.ip().to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    let mut client_ip = peer_ip;
    let mut user_agent = String::from("unknown");
    let mut auth_token = String::new();
    let mut content_length = 0usize;

    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? <= 2 && (line == "\r\n" || line == "\n" || line.is_empty())
        {
            break;
        }
        let lower = line.to_lowercase();
        if lower.starts_with("content-length:") {
            if let Some((_, val)) = line.split_once(':') {
                content_length = val.trim().parse().unwrap_or(0);
            }
        } else if lower.starts_with("authorization:") {
            if let Some((_, val)) = line.split_once(':') {
                let val = val.trim();
                if val.to_lowercase().starts_with("bearer ") {
                    auth_token = val[7..].trim().to_string();
                }
            }
        } else if lower.starts_with("cookie:") {
            if let Some((_, val)) = line.split_once(':') {
                for cookie in val.split(';') {
                    if let Some((k, v)) = cookie.split_once('=') {
                        if k.trim() == "zpanl_token" && auth_token.is_empty() {
                            auth_token = v.trim().to_string();
                        }
                    }
                }
            }
        } else if lower.starts_with("user-agent:") {
            if let Some((_, val)) = line.split_once(':') {
                user_agent = val.trim().to_string();
            }
        } else if lower.starts_with("x-forwarded-for:") {
            if let Some((_, val)) = line.split_once(':') {
                if let Some(first_ip) = val.split(',').next() {
                    let trimmed = first_ip.trim();
                    if !trimmed.is_empty() {
                        client_ip = trimmed.to_string();
                    }
                }
            }
        }
    }

    if content_length > 100 * 1024 * 1024 {
        return send_response(
            &mut stream,
            413,
            "text/plain",
            b"Payload Too Large: Max upload size is 100MB",
        );
    }
    let mut body = vec![0u8; content_length];
    if content_length > 0 {
        reader.read_exact(&mut body)?;
    }

    // CORS preflight
    if method == "OPTIONS" {
        return send_response(&mut stream, 200, "text/plain", b"OK");
    }

    // Sovereign Auth Gate: all /api/v1/ endpoints require a valid token, except login and deploy webhook
    if path.starts_with("/api/v1/")
        && path != "/api/v1/auth/login"
        && path != "/api/v1/deploy/webhook"
    {
        let mut guard = auth_db.lock().unwrap();
        if !guard.validate_session(&auth_token) {
            return send_response(
                &mut stream,
                401,
                "application/json",
                b"{\"error\":\"Unauthorized: Session token required or expired\"}",
            );
        }
    }

    // Route matching
    match (method, path) {
        // --- STATIC ASSETS ---
        ("GET", "/") => send_response(
            &mut stream,
            200,
            "text/html; charset=utf-8",
            INDEX_HTML.as_bytes(),
        ),
        ("GET", "/assets/style.css") => send_response_with_headers(
            &mut stream,
            200,
            "text/css; charset=utf-8",
            STYLE_CSS.as_bytes(),
            &["Cache-Control: no-cache, must-revalidate"],
        ),
        ("GET", "/assets/app.js") => send_response_with_headers(
            &mut stream,
            200,
            "application/javascript; charset=utf-8",
            APP_JS.as_bytes(),
            &["Cache-Control: no-cache, must-revalidate"],
        ),

        // --- AUTHENTICATION ---
        ("POST", "/api/v1/auth/login") => handlers::auth::handle_login(
            &mut stream,
            &body,
            &auth_db,
            auth_db_path,
            &client_ip,
            &user_agent,
        ),
        ("GET", "/api/v1/auth/verify") => handlers::auth::handle_verify(&mut stream, &auth_db),
        ("POST", "/api/v1/auth/logout") => {
            handlers::auth::handle_logout(&mut stream, &auth_db, auth_db_path, &auth_token)
        }
        ("POST", "/api/v1/auth/update_credentials") => {
            handlers::auth::handle_update_credentials(&mut stream, &body, &auth_db, auth_db_path)
        }
        ("GET", "/api/v1/auth/logs") => handlers::auth::handle_logs(&mut stream, &auth_db),

        // --- TELEMETRY ---
        ("GET", "/api/v1/telemetry") => {
            handlers::telemetry::handle_telemetry(&mut stream, &telemetry)
        }

        // --- VIRTUAL HOSTS / SITES ---
        ("GET", "/api/v1/sites") => handlers::sites::handle_list_sites(&mut stream, &db),
        ("POST", "/api/v1/sites") => {
            handlers::sites::handle_create_site(&mut stream, &body, &db, db_path)
        }
        ("POST", "/api/v1/sites/update") | ("PUT", "/api/v1/sites") => {
            handlers::sites::handle_update_site(&mut stream, &body, &db, db_path)
        }
        ("GET", "/api/v1/sites/vhost") => {
            handlers::sites::handle_vhost_caddyfile(&mut stream, query, &db)
        }
        ("GET", "/api/v1/sites/detail") => {
            handlers::sites::handle_site_detail(&mut stream, query, &db)
        }
        ("GET", "/api/v1/sites/logs") => handlers::sites::handle_site_logs(&mut stream, query),
        ("DELETE", "/api/v1/sites") => {
            handlers::sites::handle_delete_site(&mut stream, query, &db, db_path)
        }

        // --- SERVICES ---
        ("GET", "/api/v1/services") => handlers::services::handle_list_services(&mut stream),
        ("POST", "/api/v1/services/action") => {
            handlers::services::handle_service_action(&mut stream, &body)
        }

        // --- CADDY & PHP ---
        ("GET", "/api/v1/caddy/caddyfile") => handlers::caddy::handle_caddyfile(&mut stream, &db),
        ("GET", "/api/v1/php/pool") => handlers::caddy::handle_php_pool(&mut stream, query, &db),

        // --- FILE MANAGER ---
        ("GET", "/api/v1/files/list") => {
            handlers::files::handle_list_files(&mut stream, query, &db)
        }
        ("GET", "/api/v1/files/read") => handlers::files::handle_read_file(&mut stream, query, &db),
        ("POST", "/api/v1/files/save") => {
            handlers::files::handle_save_file(&mut stream, &body, &db)
        }
        ("POST", "/api/v1/files/delete") => {
            handlers::files::handle_delete_file(&mut stream, &body, &db)
        }
        ("POST", "/api/v1/files/create") => {
            handlers::files::handle_create_file(&mut stream, &body, &db)
        }
        ("POST", "/api/v1/files/upload") => {
            handlers::files::handle_upload_file(&mut stream, query, &body, &db)
        }

        // --- CRON JOBS ---
        ("GET", "/api/v1/cron") => handlers::cron::handle_list_cron(&mut stream, &cron_db),
        ("POST", "/api/v1/cron") => {
            handlers::cron::handle_create_cron(&mut stream, &body, &cron_db, cron_db_path)
        }
        ("POST", "/api/v1/cron/run") => {
            handlers::cron::handle_run_cron(&mut stream, &body, &cron_db, cron_db_path)
        }
        ("POST", "/api/v1/cron/toggle") => {
            handlers::cron::handle_toggle_cron(&mut stream, &body, &cron_db, cron_db_path)
        }
        ("DELETE", "/api/v1/cron") => {
            handlers::cron::handle_delete_cron(&mut stream, query, &cron_db, cron_db_path)
        }
        ("GET", "/api/v1/cron/logs") => {
            handlers::cron::handle_cron_logs(&mut stream, query, &cron_db)
        }

        // --- DATABASES ---
        ("GET", "/api/v1/databases") => {
            handlers::databases::handle_list_databases(&mut stream, &database_db)
        }
        ("POST", "/api/v1/databases") => handlers::databases::handle_create_database(
            &mut stream,
            &body,
            &database_db,
            database_db_path,
        ),
        ("DELETE", "/api/v1/databases") => handlers::databases::handle_delete_database(
            &mut stream,
            query,
            &database_db,
            database_db_path,
        ),
        ("GET", "/api/v1/databases/backup") => {
            handlers::databases::handle_backup_database(&mut stream, query, &database_db)
        }

        // --- GIT-OPS DEPLOYMENT ---
        ("GET", "/api/v1/deploy/config") => {
            handlers::deploy::handle_get_deploy_config(&mut stream, query, &deploy_db)
        }
        ("POST", "/api/v1/deploy/config") => handlers::deploy::handle_save_deploy_config(
            &mut stream,
            &body,
            &deploy_db,
            deploy_db_path,
        ),
        ("POST", "/api/v1/deploy/trigger") => handlers::deploy::handle_trigger_deploy(
            &mut stream,
            &body,
            &db,
            &deploy_db,
            deploy_db_path,
        ),
        ("GET", "/api/v1/deploy/history") => {
            handlers::deploy::handle_deploy_history(&mut stream, query, &deploy_db)
        }
        ("POST", "/api/v1/deploy/rollback") => handlers::deploy::handle_rollback_deploy(
            &mut stream,
            &body,
            &db,
            &deploy_db,
            deploy_db_path,
        ),
        ("POST", "/api/v1/deploy/webhook") => handlers::deploy::handle_webhook_deploy(
            &mut stream,
            query,
            &db,
            &deploy_db,
            deploy_db_path,
        ),

        // --- WAF & THREAT INTELLIGENCE ---
        ("GET", "/api/v1/waf/overview") => handlers::waf::handle_waf_overview(&mut stream, &waf_db),
        ("GET", "/api/v1/waf/rules") => handlers::waf::handle_waf_rules(&mut stream, &waf_db),
        ("POST", "/api/v1/waf/ip_rule") => {
            handlers::waf::handle_waf_ip_rule(&mut stream, &body, &waf_db, waf_db_path)
        }
        ("POST", "/api/v1/waf/geo_block") => {
            handlers::waf::handle_waf_geo_block(&mut stream, &body, &waf_db, waf_db_path)
        }
        ("POST", "/api/v1/waf/custom_rule") => {
            handlers::waf::handle_waf_custom_rule(&mut stream, &body, &waf_db, waf_db_path)
        }
        ("POST", "/api/v1/waf/delete_rule") => {
            handlers::waf::handle_waf_delete_rule(&mut stream, &body, &waf_db, waf_db_path)
        }

        _ => send_response(&mut stream, 404, "text/plain", b"Not Found"),
    }
}
