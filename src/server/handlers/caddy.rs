//! Caddyfile & PHP FastCGI pool configuration inspection handlers.

use crate::server::response::{parse_query_param, send_response};
use crate::site::SiteDatabase;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub fn handle_caddyfile(
    stream: &mut TcpStream,
    db: &Arc<Mutex<SiteDatabase>>,
) -> std::io::Result<()> {
    let guard = db.lock().unwrap();
    let caddyfile = guard.generate_caddyfile();
    send_response(
        stream,
        200,
        "text/plain; charset=utf-8",
        caddyfile.as_bytes(),
    )
}

pub fn handle_php_pool(
    stream: &mut TcpStream,
    query: &str,
    db: &Arc<Mutex<SiteDatabase>>,
) -> std::io::Result<()> {
    let site_domain = parse_query_param(query, "domain").unwrap_or_default();
    let guard = db.lock().unwrap();
    match guard.generate_php_pool(&site_domain) {
        Some(ini) => send_response(stream, 200, "text/plain; charset=utf-8", ini.as_bytes()),
        None => send_response(
            stream,
            404,
            "text/plain",
            b"PHP Pool not found or site is not PHP",
        ),
    }
}
