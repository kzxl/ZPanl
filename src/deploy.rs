use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployConfig {
    pub domain: String,
    pub repo_url: String,
    pub branch: String,
    pub webhook_secret: String,
    pub build_script: Option<String>,
    pub symlink_deploy: bool,
    pub auto_deploy: bool,
}

impl Default for DeployConfig {
    fn default() -> Self {
        Self {
            domain: String::new(),
            repo_url: String::new(),
            branch: "main".to_string(),
            webhook_secret: String::new(),
            build_script: None,
            symlink_deploy: false,
            auto_deploy: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployRelease {
    pub id: String,
    pub domain: String,
    pub commit_hash: String,
    pub commit_message: String,
    pub status: String,       // "success", "failed"
    pub triggered_by: String, // "manual", "webhook", "rollback"
    pub output_log: String,
    pub deployed_at: u64,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeployStorage {
    pub configs: Vec<DeployConfig>,
    pub history: Vec<DeployRelease>,
}

impl DeployStorage {
    pub fn load_or_default(path: &str) -> Self {
        if Path::new(path).exists() {
            match fs::read_to_string(path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => Self::default(),
            }
        } else {
            Self::default()
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        if let Some(parent) = Path::new(path).parent() {
            let _ = fs::create_dir_all(parent);
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(path, json)
    }

    pub fn get_config(&self, domain: &str) -> Option<&DeployConfig> {
        self.configs
            .iter()
            .find(|c| c.domain.eq_ignore_ascii_case(domain))
    }

    pub fn save_config(&mut self, config: DeployConfig) {
        if let Some(pos) = self
            .configs
            .iter()
            .position(|c| c.domain.eq_ignore_ascii_case(&config.domain))
        {
            self.configs[pos] = config;
        } else {
            self.configs.push(config);
        }
    }

    pub fn list_history(&self, domain: &str) -> Vec<&DeployRelease> {
        self.history
            .iter()
            .filter(|h| h.domain.eq_ignore_ascii_case(domain))
            .rev()
            .collect()
    }

    pub fn trigger_deploy(
        &mut self,
        domain: &str,
        site_root: &str,
        triggered_by: &str,
    ) -> Result<DeployRelease, String> {
        let config = match self.get_config(domain) {
            Some(c) => c.clone(),
            None => {
                return Err(format!(
                    "No Git deployment configuration configured for domain '{domain}'"
                ))
            }
        };

        if config.repo_url.trim().is_empty() {
            return Err("Git repository URL is required before deploying".to_string());
        }

        let start_time = Instant::now();
        let now_sec = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let release_id = format!("rel_{}", now_sec);
        let branch = if config.branch.trim().is_empty() {
            "main"
        } else {
            config.branch.trim()
        };

        let mut combined_logs = String::new();
        combined_logs.push_str(&format!(
            "=== ZPanl Sovereign Git-Ops Deployment ===\nTarget Domain: {domain}\nRepository: {}\nBranch: {branch}\nMode: {}\nTimestamp: {now_sec}\n\n",
            config.repo_url,
            if config.symlink_deploy { "Atomic Symlink Release" } else { "In-place Fast Pull" }
        ));

        let exec_cmd = |cmd_str: &str, cwd: &str| -> (bool, String) {
            let output = if cfg!(target_os = "windows") {
                Command::new("powershell")
                    .args(["-NoProfile", "-Command", cmd_str])
                    .current_dir(cwd)
                    .output()
            } else {
                Command::new("sh")
                    .args(["-c", cmd_str])
                    .current_dir(cwd)
                    .output()
            };

            match output {
                Ok(out) => {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    let mut text = String::new();
                    if !stdout.is_empty() {
                        text.push_str(&stdout);
                    }
                    if !stderr.is_empty() {
                        if !text.is_empty() {
                            text.push('\n');
                        }
                        text.push_str(&stderr);
                    }
                    (out.status.success(), text)
                }
                Err(e) => (false, format!("Process spawn error: {e}")),
            }
        };

        let mut deploy_success = true;
        let mut commit_hash = String::from("head");
        let mut commit_msg = String::from("Deploy triggered via ") + triggered_by;

        let root_path = Path::new(site_root);
        let _ = fs::create_dir_all(root_path);

        if config.symlink_deploy {
            let releases_dir = root_path.join("releases");
            let target_release = releases_dir.join(&release_id);
            let _ = fs::create_dir_all(&target_release);

            let clone_cmd = format!(
                "git clone --depth 1 --branch {} \"{}\" .",
                branch, config.repo_url
            );
            combined_logs.push_str(&format!("> {clone_cmd}\n"));
            let (ok, out) = exec_cmd(&clone_cmd, &target_release.to_string_lossy());
            combined_logs.push_str(&out);
            combined_logs.push('\n');

            if !ok {
                deploy_success = false;
            } else {
                // Get commit info
                let (c_ok, c_hash) = exec_cmd(
                    "git rev-parse --short HEAD",
                    &target_release.to_string_lossy(),
                );
                if c_ok {
                    commit_hash = c_hash.trim().to_string();
                }
                let (m_ok, m_msg) =
                    exec_cmd("git log -1 --pretty=%B", &target_release.to_string_lossy());
                if m_ok && !m_msg.trim().is_empty() {
                    commit_msg = m_msg.trim().lines().next().unwrap_or(&m_msg).to_string();
                }

                // Run build script if provided
                if let Some(ref script) = config.build_script {
                    if !script.trim().is_empty() {
                        combined_logs.push_str("\n--- Executing Post-Deploy Build Script ---\n");
                        let (s_ok, s_out) = exec_cmd(script, &target_release.to_string_lossy());
                        combined_logs.push_str(&s_out);
                        combined_logs.push('\n');
                        if !s_ok {
                            deploy_success = false;
                        }
                    }
                }

                // Switch symlink
                if deploy_success {
                    let current_link = root_path.join("current");
                    match switch_atomic_symlink(&current_link, &target_release) {
                        Ok(()) => {
                            combined_logs.push_str(&format!(
                                "\n[OK] Atomic symlink switched to release {release_id} in < 1ms\n"
                            ));
                        }
                        Err(e) => {
                            combined_logs.push_str(&format!("[WARN] Symlink switch error: {e}\n"));
                        }
                    }
                }
            }
        } else {
            // In-place deployment
            let git_dir = root_path.join(".git");
            let pull_cmd = if git_dir.exists() {
                format!(
                    "git fetch origin {} && git checkout {} && git pull origin {}",
                    branch, branch, branch
                )
            } else {
                format!(
                    "git clone --depth 1 --branch {} \"{}\" .",
                    branch, config.repo_url
                )
            };

            combined_logs.push_str(&format!("> {pull_cmd}\n"));
            let (ok, out) = exec_cmd(&pull_cmd, site_root);
            combined_logs.push_str(&out);
            combined_logs.push('\n');

            if !ok {
                deploy_success = false;
            } else {
                let (c_ok, c_hash) = exec_cmd("git rev-parse --short HEAD", site_root);
                if c_ok {
                    commit_hash = c_hash.trim().to_string();
                }
                let (m_ok, m_msg) = exec_cmd("git log -1 --pretty=%B", site_root);
                if m_ok && !m_msg.trim().is_empty() {
                    commit_msg = m_msg.trim().lines().next().unwrap_or(&m_msg).to_string();
                }

                if let Some(ref script) = config.build_script {
                    if !script.trim().is_empty() {
                        combined_logs.push_str("\n--- Executing Post-Deploy Build Script ---\n");
                        let (s_ok, s_out) = exec_cmd(script, site_root);
                        combined_logs.push_str(&s_out);
                        combined_logs.push('\n');
                        if !s_ok {
                            deploy_success = false;
                        }
                    }
                }
            }
        }

        let duration_ms = start_time.elapsed().as_millis() as u64;
        let status = if deploy_success { "success" } else { "failed" };
        combined_logs.push_str(&format!(
            "\n=== Deployment Result: {} in {}ms ===\n",
            status.to_uppercase(),
            duration_ms
        ));

        if combined_logs.len() > 65536 {
            combined_logs.truncate(65536);
            combined_logs.push_str("\n... [Output truncated at 64KB]");
        }

        let release = DeployRelease {
            id: release_id,
            domain: domain.to_string(),
            commit_hash,
            commit_message: commit_msg,
            status: status.to_string(),
            triggered_by: triggered_by.to_string(),
            output_log: combined_logs,
            deployed_at: now_sec,
            duration_ms,
        };

        self.history.push(release.clone());
        if self.history.len() > 50 {
            self.history.remove(0);
        }

        if deploy_success {
            Ok(release)
        } else {
            Err("Deployment failed: view logs for details".to_string())
        }
    }

    pub fn rollback(
        &mut self,
        domain: &str,
        site_root: &str,
        release_id: &str,
    ) -> Result<DeployRelease, String> {
        let releases_dir = Path::new(site_root).join("releases");
        let target_release = releases_dir.join(release_id);

        if !target_release.exists() {
            return Err(format!("Release '{release_id}' directory not found"));
        }

        let start = Instant::now();
        let current_link = Path::new(site_root).join("current");
        let symlink_res = switch_atomic_symlink(&current_link, &target_release);
        let success = symlink_res.is_ok();
        let err_msg = symlink_res.err().unwrap_or_default();

        let now_sec = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let duration_ms = start.elapsed().as_millis() as u64;

        let release = DeployRelease {
            id: format!("rollback_{now_sec}"),
            domain: domain.to_string(),
            commit_hash: "rollback".to_string(),
            commit_message: format!("Rollback to release {release_id}"),
            status: if success {
                "success".to_string()
            } else {
                "failed".to_string()
            },
            triggered_by: "rollback".to_string(),
            output_log: format!(
                "Instant atomic symlink switched to release '{release_id}' in {duration_ms}ms"
            ),
            deployed_at: now_sec,
            duration_ms,
        };

        self.history.push(release.clone());
        if success {
            Ok(release)
        } else {
            Err(format!("Failed to switch release symlink: {err_msg}"))
        }
    }
}

/// Switches an atomic symbolic link / directory junction to a new release in < 1ms.
pub fn switch_atomic_symlink(link_path: &Path, target_path: &Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        if link_path.symlink_metadata().is_ok() {
            let _ = fs::remove_dir(link_path);
            let mut del_cmd = Command::new("cmd");
            del_cmd.raw_arg(format!("/C rmdir \"{}\"", link_path.display()));
            let _ = del_cmd.output();
        }
        let mut cmd = Command::new("cmd");
        cmd.raw_arg(format!(
            "/C mklink /J \"{}\" \"{}\"",
            link_path.display(),
            target_path.display()
        ));
        let out = cmd
            .output()
            .map_err(|e| format!("Failed to spawn cmd for junction: {e}"))?;
        if out.status.success() {
            Ok(())
        } else {
            let err = String::from_utf8_lossy(&out.stderr);
            let out_str = String::from_utf8_lossy(&out.stdout);
            Err(format!(
                "mklink failed (exit {:?}): stdout: {}, stderr: {}",
                out.status.code(),
                out_str,
                err
            ))
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let out = Command::new("ln")
            .args([
                "-sfn",
                &target_path.to_string_lossy(),
                &link_path.to_string_lossy(),
            ])
            .output()
            .map_err(|e| format!("Failed to spawn ln: {e}"))?;
        if out.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).to_string())
        }
    }
}
