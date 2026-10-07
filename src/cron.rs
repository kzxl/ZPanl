use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronJob {
    pub id: String,
    pub name: String,
    pub schedule: String,
    pub command: String,
    pub site: Option<String>,
    pub enabled: bool,
    pub created_at: u64,
    pub last_run_at: Option<u64>,
    pub last_status: Option<String>,
    pub last_output: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CronDatabase {
    pub jobs: Vec<CronJob>,
}

impl CronDatabase {
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

    pub fn list(&self) -> &[CronJob] {
        &self.jobs
    }

    pub fn add(&mut self, job: CronJob) -> Result<(), String> {
        if job.name.trim().is_empty() {
            return Err("Task name cannot be empty".to_string());
        }
        if job.command.trim().is_empty() {
            return Err("Command cannot be empty".to_string());
        }
        if job.schedule.trim().is_empty() {
            return Err("Cron schedule expression cannot be empty".to_string());
        }
        self.jobs.push(job);
        Ok(())
    }

    pub fn delete(&mut self, id: &str) -> bool {
        let initial_len = self.jobs.len();
        self.jobs.retain(|j| j.id != id);
        self.jobs.len() < initial_len
    }

    pub fn find(&self, id: &str) -> Option<&CronJob> {
        self.jobs.iter().find(|j| j.id == id)
    }

    pub fn toggle(&mut self, id: &str) -> Option<bool> {
        if let Some(job) = self.jobs.iter_mut().find(|j| j.id == id) {
            job.enabled = !job.enabled;
            Some(job.enabled)
        } else {
            None
        }
    }

    pub fn execute_job(&mut self, id: &str) -> Result<String, String> {
        let job = match self.jobs.iter_mut().find(|j| j.id == id) {
            Some(j) => j,
            None => return Err("Job not found".to_string()),
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        job.last_run_at = Some(now);

        let output = if cfg!(target_os = "windows") {
            Command::new("powershell")
                .args(["-NoProfile", "-Command", &job.command])
                .output()
        } else {
            Command::new("sh").args(["-c", &job.command]).output()
        };

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);
                let mut combined = String::new();
                if !stdout.is_empty() {
                    combined.push_str(&stdout);
                }
                if !stderr.is_empty() {
                    if !combined.is_empty() {
                        combined.push('\n');
                    }
                    combined.push_str(&stderr);
                }

                if combined.len() > 65536 {
                    combined.truncate(65536);
                    combined.push_str("\n... [Output truncated at 64KB]");
                }

                if out.status.success() {
                    job.last_status = Some("success".to_string());
                    job.last_output = Some(combined.clone());
                    Ok(combined)
                } else {
                    let code = out.status.code().unwrap_or(-1);
                    job.last_status = Some(format!("failed (code {code})"));
                    job.last_output = Some(combined.clone());
                    Err(format!("Command exited with code {code}: {combined}"))
                }
            }
            Err(e) => {
                let err_msg = format!("Failed to spawn process: {e}");
                job.last_status = Some("error".to_string());
                job.last_output = Some(err_msg.clone());
                Err(err_msg)
            }
        }
    }

    pub fn get_logs(&self, id: &str) -> Option<String> {
        self.jobs
            .iter()
            .find(|j| j.id == id)
            .and_then(|j| j.last_output.clone())
    }
}
