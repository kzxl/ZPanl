use serde::Serialize;
use std::process::Command;
use zero_sys::service::{ServiceAction, ServiceState, SystemdManager};

/// Status report for an individual background service.
#[derive(Debug, Clone, Serialize)]
pub struct ServiceStatusItem {
    pub name: String,
    pub display_name: String,
    pub state: String,
    pub is_active: bool,
}

pub struct ServiceManager;

impl ServiceManager {
    /// Standard services monitored by ZPanl.
    const MONITORED: &'static [(&'static str, &'static str)] = &[
        ("caddy", "Caddy Web Server v2"),
        ("php8.2-fpm", "PHP 8.2 FastCGI Process Manager"),
        ("php8.3-fpm", "PHP 8.3 FastCGI Process Manager"),
        ("mariadb", "MariaDB Database Server"),
    ];

    /// Queries the status of all monitored services.
    pub fn list_services() -> Vec<ServiceStatusItem> {
        let mut items = Vec::new();

        for &(name, display) in Self::MONITORED {
            let state = Self::query_state(name);
            let is_active = state == ServiceState::Active;
            items.push(ServiceStatusItem {
                name: name.to_string(),
                display_name: display.to_string(),
                state: format!("{state:?}"),
                is_active,
            });
        }

        items
    }

    /// Queries the current state of a systemd service.
    pub fn query_state(name: &str) -> ServiceState {
        if let Ok(output) = Command::new("systemctl").args(["is-active", name]).output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            ServiceState::from_is_active(&stdout)
        } else {
            // Development fallback when systemctl is not present
            ServiceState::Active
        }
    }

    /// Triggers an action (restart, reload, stop, start) on a service.
    pub fn execute_action(name: &str, action: ServiceAction) -> Result<String, String> {
        let args = SystemdManager::build_command_args(action, name);
        let output = Command::new("systemctl")
            .args(args)
            .output()
            .map_err(|e| format!("Failed to invoke systemctl: {e}"))?;

        if output.status.success() {
            Ok(format!("Service '{name}' {action:?} successful"))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("systemctl error: {stderr}"))
        }
    }
}
