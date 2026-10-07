//! # ZPanl ⚡🌐
//!
//! Sovereign, ultra-lightweight Linux web panel for static websites and PHP-FPM hosting.
//! Powered 100% by the ZeroRust ecosystem (`zero-sys`, `zero-fastcgi`, `zero-caddy`, `zero-vfs`).

pub mod auth;
pub mod cli;
pub mod cron;
pub mod database;
pub mod deploy;
pub mod filemgr;
pub mod server;
pub mod services;
pub mod site;
pub mod telemetry;
pub mod ui;

use cli::Cli;

fn main() {
    Cli::run();
}
