use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use zero_caddy::VhostBuilder;
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
}

/// Metadata record for a hosted site.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteRecord {
    pub id: String,
    pub domain: String,
    pub root_path: String,
    pub kind: SiteKind,
    pub php_version: Option<String>,
    pub ssl_enabled: bool,
    pub created_at: u64,
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

    /// Generates complete Caddyfile containing all active virtual hosts.
    pub fn generate_caddyfile(&self) -> String {
        let mut out = String::from("# Generated automatically by ZPanl - Do not edit manually\n\n");
        out.push_str("{\n    # Global Caddy options\n    auto_https disable_redirects\n}\n\n");

        for site in &self.sites {
            let mut builder = VhostBuilder::new(&site.domain).root(&site.root_path);
            let sock;
            match site.kind {
                SiteKind::Static => {}
                SiteKind::SpaFallback => {
                    builder = builder.spa();
                }
                SiteKind::PhpFpm => {
                    let ver = site.php_version.as_deref().unwrap_or("8.2");
                    sock = format!("unix//run/php/php{ver}-fpm.sock");
                    builder = builder.php(&sock);
                }
            }

            let mut buf = [0u8; 2048];
            if let Ok(len) = builder.write_caddyfile(&mut buf) {
                if let Ok(block) = std::str::from_utf8(&buf[..len]) {
                    out.push_str(block);
                    out.push_str("\n\n");
                }
            }
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
