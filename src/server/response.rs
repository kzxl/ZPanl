//! HTTP response utilities and URL parsing.

use std::io::Write;
use std::net::TcpStream;

pub fn send_response(
    stream: &mut TcpStream,
    status_code: u16,
    content_type: &str,
    body: &[u8],
) -> std::io::Result<()> {
    send_response_with_headers(stream, status_code, content_type, body, &[])
}

pub fn send_cached_response(
    stream: &mut TcpStream,
    status_code: u16,
    content_type: &str,
    body: &[u8],
    max_age_secs: u32,
) -> std::io::Result<()> {
    let cache_hdr = format!("Cache-Control: public, max-age={max_age_secs}");
    send_response_with_headers(stream, status_code, content_type, body, &[&cache_hdr])
}

pub fn send_response_with_headers(
    stream: &mut TcpStream,
    status_code: u16,
    content_type: &str,
    body: &[u8],
    extra_headers: &[&str],
) -> std::io::Result<()> {
    let status_line = match status_code {
        200 => "HTTP/1.1 200 OK",
        400 => "HTTP/1.1 400 Bad Request",
        401 => "HTTP/1.1 401 Unauthorized",
        403 => "HTTP/1.1 403 Forbidden",
        404 => "HTTP/1.1 404 Not Found",
        413 => "HTTP/1.1 413 Payload Too Large",
        429 => "HTTP/1.1 429 Too Many Requests",
        500 => "HTTP/1.1 500 Internal Server Error",
        _ => "HTTP/1.1 200 OK",
    };

    let mut header = format!(
        "{status_line}\r\n\
         Content-Type: {content_type}\r\n\
         Content-Length: {}\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\n\
         Access-Control-Allow-Headers: Content-Type, Authorization\r\n\
         Connection: close\r\n",
        body.len()
    );

    for hdr in extra_headers {
        header.push_str(hdr);
        header.push_str("\r\n");
    }
    header.push_str("\r\n");

    stream.write_all(header.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()?;
    Ok(())
}

pub fn parse_query_param(query: &str, key: &str) -> Option<String> {
    for pair in query.split('&') {
        let mut parts = pair.split('=');
        if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
            if k == key {
                return Some(url_decode(v));
            }
        }
    }
    None
}

pub fn url_decode(s: &str) -> String {
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
