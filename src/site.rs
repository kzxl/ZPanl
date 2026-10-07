use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use zero_fastcgi::PhpPoolConfig;

/// Types of websites managed by ZPanl.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SiteKind {
    /// Static HTML/CSS/JS site.
    Static,
    /// Single Page Application with fallback to index.html.
    SpaFallback,
    /// Dynamic PHP site proxied to PHP-FPM socket.
    PhpFpm,
    /// Reverse proxy to upstream HTTP/WS server (Node, Go, Python, Docker).
    ReverseProxy,
}

fn default_port() -> u16 {
    80
}

fn default_redirect_code() -> u16 {
    301
}

/// URL Redirect mapping rule.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RedirectRule {
    pub source_path: String,
    pub target_url: String,
    #[serde(default = "default_redirect_code")]
    pub code: u16,
}

/// Metadata record for a hosted site.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteRecord {
    pub id: String,
    pub domain: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default = "default_port")]
    pub port: u16,
    pub root_path: String,
    #[serde(default)]
    pub running_dir: Option<String>,
    pub kind: SiteKind,
    #[serde(default)]
    pub php_version: Option<String>,
    #[serde(default)]
    pub proxy_upstream: Option<String>,
    #[serde(default)]
    pub rewrite_preset: Option<String>,
    #[serde(default)]
    pub maintenance: bool,
    pub ssl_enabled: bool,
    #[serde(default)]
    pub custom_caddy: Option<String>,
    #[serde(default)]
    pub ip_blacklist: Vec<String>,
    #[serde(default)]
    pub basic_auth_user: Option<String>,
    #[serde(default)]
    pub basic_auth_pass: Option<String>,
    #[serde(default)]
    pub hotlink_protection: bool,
    #[serde(default)]
    pub hotlink_extensions: Option<String>,
    #[serde(default)]
    pub redirects: Vec<RedirectRule>,
    pub created_at: u64,
}

impl Default for SiteRecord {
    fn default() -> Self {
        Self {
            id: String::new(),
            domain: String::new(),
            aliases: Vec::new(),
            port: 80,
            root_path: "/var/www/html".to_string(),
            running_dir: None,
            kind: SiteKind::Static,
            php_version: None,
            proxy_upstream: None,
            rewrite_preset: None,
            maintenance: false,
            ssl_enabled: true,
            custom_caddy: None,
            ip_blacklist: Vec::new(),
            basic_auth_user: None,
            basic_auth_pass: None,
            hotlink_protection: false,
            hotlink_extensions: None,
            redirects: Vec::new(),
            created_at: 0,
        }
    }
}

impl SiteRecord {
    pub fn new(
        id: impl Into<String>,
        domain: impl Into<String>,
        root_path: impl Into<String>,
        kind: SiteKind,
    ) -> Self {
        Self {
            id: id.into(),
            domain: domain.into(),
            root_path: root_path.into(),
            kind,
            ..Default::default()
        }
    }
}

/// In-memory and persistent site database.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SiteDatabase {
    pub sites: Vec<SiteRecord>,
}

impl SiteDatabase {
    /// Loads site database from JSON file, or creates an empty one if missing.
    pub fn load_or_default(path: &str) -> Self {
        if Path::new(path).exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(db) = serde_json::from_str(&content) {
                    return db;
                }
            }
        }
        Self::default()
    }

    /// Persists site database to JSON file.
    pub fn save(&self, path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize sites: {e}"))?;
        fs::write(path, json).map_err(|e| format!("Failed to write sites file {path}: {e}"))?;
        Ok(())
    }

    /// Lists all hosted sites.
    pub fn list(&self) -> &[SiteRecord] {
        &self.sites
    }

    /// Finds a site by its domain.
    pub fn find_by_domain(&self, domain: &str) -> Option<&SiteRecord> {
        self.sites
            .iter()
            .find(|s| s.domain.eq_ignore_ascii_case(domain))
    }

    /// Adds a site to the database. Rejects duplicate domains.
    pub fn add(&mut self, site: SiteRecord) -> Result<(), String> {
        if self.find_by_domain(&site.domain).is_some() {
            return Err(format!("Domain '{}' already exists in ZPanl", site.domain));
        }
        self.sites.push(site);
        Ok(())
    }

    /// Updates an existing site record in place.
    pub fn update(&mut self, domain: &str, updated: SiteRecord) -> Result<(), String> {
        if let Some(pos) = self
            .sites
            .iter()
            .position(|s| s.domain.eq_ignore_ascii_case(domain))
        {
            self.sites[pos] = updated;
            Ok(())
        } else {
            Err(format!("Domain '{domain}' not found in ZPanl"))
        }
    }

    /// Removes a site by domain name.
    pub fn remove(&mut self, domain: &str) -> Result<SiteRecord, String> {
        if let Some(pos) = self
            .sites
            .iter()
            .position(|s| s.domain.eq_ignore_ascii_case(domain))
        {
            Ok(self.sites.remove(pos))
        } else {
            Err(format!("Domain '{domain}' not found in ZPanl"))
        }
    }

    /// Generates Caddyfile server block for a single hosted site.
    pub fn generate_site_caddyfile(&self, site: &SiteRecord) -> String {
        let mut out = String::new();
        let port_suffix = match site.port {
            80 | 443 | 0 => String::new(),
            p => format!(":{}", p),
        };

        let base_proto = if !site.ssl_enabled { "http://" } else { "" };
        let mut hosts = Vec::new();
        hosts.push(format!("{}{}{}", base_proto, site.domain, port_suffix));
        for alias in &site.aliases {
            let a = alias.trim();
            if !a.is_empty() {
                hosts.push(format!("{}{}{}", base_proto, a, port_suffix));
            }
        }
        out.push_str(&hosts.join(", "));
        out.push_str(" {\n");

        if site.maintenance {
            out.push_str("    # Maintenance Mode Active (HTTP 503)\n");
            out.push_str("    respond 503 {\n");
            out.push_str("        body \"503 Service Unavailable - Site Under Maintenance\"\n");
            out.push_str("        close\n");
            out.push_str("    }\n");
            out.push_str("}\n");
            return out;
        }

        let effective_root = if let Some(ref sub) = site.running_dir {
            let trimmed = sub.trim_matches('/');
            if trimmed.is_empty() {
                site.root_path.clone()
            } else {
                format!("{}/{}", site.root_path.trim_end_matches('/'), trimmed)
            }
        } else {
            site.root_path.clone()
        };

        out.push_str(&format!("    root * {}\n", effective_root));
        out.push_str("    encode zstd gzip\n");

        // Access log output
        let clean_dom = site.domain.replace(':', "_");
        out.push_str(&format!(
            "    log {{\n        output file /var/log/zpanl/{clean_dom}.log\n    }}\n"
        ));

        // IP Blacklist access control
        let valid_ips: Vec<&str> = site
            .ip_blacklist
            .iter()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        if !valid_ips.is_empty() {
            out.push_str("    @blocked_ips {\n        remote_ip ");
            out.push_str(&valid_ips.join(" "));
            out.push_str("\n    }\n    respond @blocked_ips 403\n");
        }

        // HTTP Basic Authentication
        if let (Some(ref u), Some(ref p)) = (&site.basic_auth_user, &site.basic_auth_pass) {
            let user = u.trim();
            let pass = p.trim();
            if !user.is_empty() && !pass.is_empty() {
                out.push_str(&format!(
                    "    basicauth * {{\n        {user} {pass}\n    }}\n"
                ));
            }
        }

        // Hotlink / Anti-Leech Protection
        if site.hotlink_protection {
            let exts = site
                .hotlink_extensions
                .as_deref()
                .unwrap_or("*.jpg *.jpeg *.png *.webp *.gif *.svg");
            out.push_str(&format!(
                "    @hotlink {{\n        path {exts}\n        not header Referer *{}*\n        not header Referer \"\"\n    }}\n    respond @hotlink 403\n",
                site.domain
            ));
        }

        // Custom URL Redirects
        for r in &site.redirects {
            let src = r.source_path.trim();
            let tgt = r.target_url.trim();
            if !src.is_empty() && !tgt.is_empty() {
                out.push_str(&format!("    redir {src} {tgt} {}\n", r.code));
            }
        }

        if let Some(ref upstream) = site.proxy_upstream {
            let up = upstream.trim();
            if !up.is_empty() {
                out.push_str(&format!("    reverse_proxy {}\n", up));
            }
        }

        match site.kind {
            SiteKind::Static => {
                if let Some(preset) = site.rewrite_preset.as_deref() {
                    if preset == "spa" {
                        out.push_str("    try_files {path} /index.html\n");
                    }
                }
                out.push_str("    file_server\n");
            }
            SiteKind::SpaFallback => {
                out.push_str("    try_files {path} /index.html\n");
                out.push_str("    file_server\n");
            }
            SiteKind::PhpFpm => {
                let ver = site.php_version.as_deref().unwrap_or("8.2");
                let sock = format!("unix//run/php/php{ver}-fpm.sock");
                match site.rewrite_preset.as_deref() {
                    Some("laravel") | Some("symfony") => {
                        out.push_str(&format!("    php_fastcgi {}\n", sock));
                        out.push_str("    file_server\n");
                        out.push_str("    try_files {path} {path}/ /index.php?{query}\n");
                    }
                    Some("wordpress") => {
                        out.push_str(&format!("    php_fastcgi {}\n", sock));
                        out.push_str("    file_server\n");
                        out.push_str(
                            "    @blocked {\n        path /wp-config.php /xmlrpc.php\n    }\n",
                        );
                        out.push_str("    respond @blocked 403\n");
                    }
                    _ => {
                        out.push_str(&format!("    php_fastcgi {}\n", sock));
                        out.push_str("    file_server\n");
                    }
                }
            }
            SiteKind::ReverseProxy => {
                // If upstream is not set above, default to local port 3000
                if site.proxy_upstream.is_none() {
                    out.push_str("    reverse_proxy 127.0.0.1:3000\n");
                }
            }
        }

        if let Some(ref custom) = site.custom_caddy {
            let trimmed = custom.trim();
            if !trimmed.is_empty() {
                out.push_str("    # Custom directives\n");
                for line in trimmed.lines() {
                    out.push_str(&format!("    {}\n", line));
                }
            }
        }

        out.push_str("}\n");
        out
    }

    /// Generates complete Caddyfile containing all active virtual hosts.
    pub fn generate_caddyfile(&self) -> String {
        let mut out = String::from("# Generated automatically by ZPanl - Do not edit manually\n\n");
        out.push_str("{\n    # Global Caddy options\n    auto_https disable_redirects\n}\n\n");

        for site in &self.sites {
            out.push_str(&self.generate_site_caddyfile(site));
            out.push('\n');
        }

        out
    }

    /// Generates PHP-FPM pool configuration for a given site.
    pub fn generate_php_pool(&self, domain: &str) -> Option<String> {
        let site = self.find_by_domain(domain)?;
        if site.kind != SiteKind::PhpFpm {
            return None;
        }

        let ver = site.php_version.as_deref().unwrap_or("8.2");
        let sock = format!(
            "/run/php/zpanl-{}-{}.sock",
            site.domain.replace('.', "_"),
            ver
        );
        let pool = PhpPoolConfig {
            pool_name: &site.domain,
            listen_socket: &sock,
            ..Default::default()
        };

        let mut buf = [0u8; 2048];
        if let Ok(len) = pool.write_ini_config(&mut buf) {
            if let Ok(s) = std::str::from_utf8(&buf[..len]) {
                return Some(s.to_string());
            }
        }
        None
    }
}
