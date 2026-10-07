use crate::server::HttpServer;
use crate::services::ServiceManager;
use crate::site::{SiteDatabase, SiteKind, SiteRecord};
use crate::telemetry::TelemetryCollector;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Cli;

impl Cli {
    pub fn run() {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            Self::print_help();
            return;
        }

        let cmd = args[1].as_str();
        match cmd {
            "run" => {
                let bind = args
                    .iter()
                    .position(|a| a == "--bind" || a == "-b")
                    .and_then(|i| args.get(i + 1))
                    .map(|s| s.as_str())
                    .unwrap_or("0.0.0.0:8888");

                let db_path = args
                    .iter()
                    .position(|a| a == "--db")
                    .and_then(|i| args.get(i + 1))
                    .map(|s| s.as_str())
                    .unwrap_or("zpanl_sites.json");

                let server = HttpServer::new(bind, db_path);
                if let Err(e) = server.run() {
                    eprintln!("Failed to start ZPanl server: {e}");
                }
            }

            "site" => {
                let sub = args.get(2).map(|s| s.as_str()).unwrap_or("list");
                let db_path = "zpanl_sites.json";
                let mut db = SiteDatabase::load_or_default(db_path);

                match sub {
                    "list" => {
                        println!("\n🌐 ZPanl Virtual Hosts ({} sites):", db.sites.len());
                        println!("{:-<70}", "");
                        for s in &db.sites {
                            println!(
                                "• {:<25} | {:<12} | PHP {:<4} | {}",
                                s.domain,
                                format!("{:?}", s.kind),
                                s.php_version.as_deref().unwrap_or("-"),
                                s.root_path
                            );
                        }
                        println!("{:-<70}\n", "");
                    }

                    "add" => {
                        if args.len() < 6 {
                            eprintln!("Usage: zpanl site add <domain> <root_path> <static|spa|php> [php_version]");
                            return;
                        }
                        let domain = args[3].clone();
                        let root_path = args[4].clone();
                        let kind = match args[5].to_lowercase().as_str() {
                            "static" => SiteKind::Static,
                            "spa" => SiteKind::SpaFallback,
                            "php" => SiteKind::PhpFpm,
                            _ => {
                                eprintln!("Invalid kind. Choose from: static, spa, php");
                                return;
                            }
                        };
                        let php_ver = args.get(6).cloned();

                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .map(|d| d.as_secs())
                            .unwrap_or(0);

                        let record = SiteRecord {
                            id: format!("site_{now}"),
                            domain: domain.clone(),
                            root_path,
                            kind,
                            php_version: php_ver,
                            ssl_enabled: true,
                            created_at: now,
                            ..Default::default()
                        };

                        if let Err(e) = db.add(record) {
                            eprintln!("Error: {e}");
                        } else {
                            let _ = db.save(db_path);
                            println!("✅ Successfully created virtual host for '{domain}'");
                        }
                    }

                    "rm" | "delete" => {
                        let domain = match args.get(3) {
                            Some(d) => d,
                            None => {
                                eprintln!("Usage: zpanl site rm <domain>");
                                return;
                            }
                        };
                        if let Err(e) = db.remove(domain) {
                            eprintln!("Error: {e}");
                        } else {
                            let _ = db.save(db_path);
                            println!("🗑️ Removed site '{domain}'");
                        }
                    }

                    _ => eprintln!("Unknown site subcommand '{sub}'"),
                }
            }

            "telemetry" => {
                let collector = TelemetryCollector::new();
                let t = collector.sample();
                println!("\n📊 System Telemetry (Linux /proc: {}):", t.is_linux_proc);
                println!("• CPU Usage:     {:.1}%", t.cpu_usage_percent);
                println!(
                    "• RAM Usage:     {} / {} MB ({:.1}%)",
                    t.ram_used_mb, t.ram_total_mb, t.ram_usage_percent
                );
                println!(
                    "• Network In/Out: {} KB/s rx / {} KB/s tx",
                    t.net_rx_kbps, t.net_tx_kbps
                );
                println!("• Uptime:        {} seconds\n", t.uptime_seconds);
            }

            "services" => {
                let services = ServiceManager::list_services();
                println!("\n⚙️ Monitored Services:");
                for s in services {
                    let status_str = if s.is_active { "ACTIVE" } else { "INACTIVE" };
                    println!("• {:<20} [{:<8}] {}", s.name, status_str, s.display_name);
                }
                println!();
            }

            "export-caddy" => {
                let db = SiteDatabase::load_or_default("zpanl_sites.json");
                println!("{}", db.generate_caddyfile());
            }

            "help" | "--help" | "-h" => Self::print_help(),

            unknown => {
                eprintln!("Unknown command '{unknown}'");
                Self::print_help();
            }
        }
    }

    fn print_help() {
        println!(
            r#"
⚡ ZPanl - Sovereign Linux Web Control Panel v0.1.0

USAGE:
    zpanl <COMMAND> [OPTIONS]

COMMANDS:
    run                 Launch the ZPanl HTTP Web Daemon & embedded UI
                        Options: --bind <ip:port> (default: 0.0.0.0:8888)
                                 --db <path>      (default: zpanl_sites.json)

    site list           List all registered virtual hosts
    site add            Create a new virtual host
                        Args: <domain> <root_path> <static|spa|php> [php_ver]
    site rm             Remove a virtual host by domain
                        Args: <domain>

    telemetry           Display real-time Linux CPU, RAM, and Network metrics
    services            Inspect status of Caddy, PHP-FPM, and MariaDB
    export-caddy        Print generated /etc/caddy/Caddyfile to stdout
    help, -h            Show this help reference
"#
        );
    }
}
