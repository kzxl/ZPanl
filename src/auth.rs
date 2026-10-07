use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// Standard FIPS 180-4 SHA-256 Implementation (100% pure Rust, zero external dependencies).
#[allow(clippy::chunks_exact_to_as_chunks)]
pub fn sha256(data: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    let bit_len = (data.len() as u64) * 8;
    let mut msg = Vec::with_capacity(data.len() + 64);
    msg.extend_from_slice(data);
    msg.push(0x80);

    while (msg.len() % 64) != 56 {
        msg.push(0x00);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut h_var = h[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h_var
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h_var = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(h_var);
    }

    let mut out = [0u8; 32];
    for (i, val) in h.iter().enumerate() {
        let bytes = val.to_be_bytes();
        out[i * 4..i * 4 + 4].copy_from_slice(&bytes);
    }
    out
}

pub fn to_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

/// Computes multi-iteration salted key-stretching hash for passwords.
pub fn hash_password(password: &str, salt: &str) -> String {
    let mut current = sha256(format!("{salt}:{password}").as_bytes());
    for i in 1u32..1000u32 {
        let mut buf = Vec::with_capacity(64);
        buf.extend_from_slice(&current);
        buf.extend_from_slice(salt.as_bytes());
        buf.extend_from_slice(&i.to_be_bytes());
        current = sha256(&buf);
    }
    to_hex(&current)
}

/// Generates a random cryptographic token using high-resolution entropy and SHA-256.
pub fn generate_random_token() -> String {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let count = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let ptr = &COUNTER as *const _ as usize;
    let entropy = format!(
        "{now}:{count}:{ptr}:{}:{}",
        std::process::id(),
        std::thread::current().name().unwrap_or("worker")
    );
    to_hex(&sha256(entropy.as_bytes()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminCredentials {
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Default for AdminCredentials {
    fn default() -> Self {
        let salt = generate_random_token();
        let default_pass = "zpanl@admin2026";
        Self {
            username: "admin".to_string(),
            password_hash: hash_password(default_pass, &salt),
            salt,
            created_at: 0,
            updated_at: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginLog {
    pub id: String,
    pub timestamp: u64,
    pub username: String,
    pub ip: String,
    pub user_agent: String,
    pub status: String, // "success", "failed", "locked"
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthStorage {
    pub admin: AdminCredentials,
    #[serde(default)]
    pub sessions: HashMap<String, u64>, // token -> expire_epoch_sec
    #[serde(default)]
    pub login_logs: Vec<LoginLog>,
    #[serde(skip)]
    pub failed_attempts: HashMap<String, (u32, u64)>, // ip -> (count, locked_until_sec)
}

impl AuthStorage {
    pub fn load_or_init(path: &str) -> (Self, Option<String>) {
        if Path::new(path).exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(storage) = serde_json::from_str::<AuthStorage>(&content) {
                    return (storage, None);
                }
            }
        }

        // Initialize new admin credentials
        let now_sec = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let initial_pass = "zpanl@admin2026";
        let salt = generate_random_token();
        let admin = AdminCredentials {
            username: "admin".to_string(),
            password_hash: hash_password(initial_pass, &salt),
            salt,
            created_at: now_sec,
            updated_at: now_sec,
        };

        let storage = AuthStorage {
            admin,
            sessions: HashMap::new(),
            login_logs: Vec::new(),
            failed_attempts: HashMap::new(),
        };

        let _ = storage.save(path);

        println!("\n╔═══════════════════════════════════════════════════════════════════╗");
        println!("║  🛡️  ZPANL SOVEREIGN ADMIN AUTHENTICATION INITIALIZED             ║");
        println!("╠═══════════════════════════════════════════════════════════════════╣");
        println!("║  Username: admin                                                  ║");
        println!("║  Password: {:<54} ║", initial_pass);
        println!("║  Security Notice: Log in and update your password immediately!   ║");
        println!("╚═══════════════════════════════════════════════════════════════════╝\n");

        (storage, Some(initial_pass.to_string()))
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        if let Some(parent) = Path::new(path).parent() {
            let _ = fs::create_dir_all(parent);
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(path, json)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let test_hash = hash_password(password, &self.admin.salt);
        test_hash == self.admin.password_hash
    }

    pub fn update_password(&mut self, new_pass: &str) {
        let new_salt = generate_random_token();
        self.admin.password_hash = hash_password(new_pass, &new_salt);
        self.admin.salt = new_salt;
        self.admin.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        // Revoke all sessions on password change
        self.sessions.clear();
    }

    pub fn update_username(&mut self, new_username: &str) {
        self.admin.username = new_username.trim().to_string();
        self.admin.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
    }

    pub fn authenticate(
        &mut self,
        username: &str,
        password: &str,
        ip: &str,
        user_agent: &str,
    ) -> Result<String, String> {
        let now_sec = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // 1. Check brute-force lock
        if let Some((_count, locked_until)) = self.failed_attempts.get(ip) {
            if now_sec < *locked_until {
                let remaining = locked_until - now_sec;
                self.record_log(username, ip, user_agent, "locked");
                return Err(format!(
                    "Too many failed login attempts. IP temporarily locked for {remaining}s"
                ));
            }
        }

        // 2. Verify username and password
        let username_ok = self.admin.username.eq_ignore_ascii_case(username.trim());
        let pass_ok = self.verify_password(password);

        if username_ok && pass_ok {
            // Success: clear failed attempts
            self.failed_attempts.remove(ip);
            self.record_log(username, ip, user_agent, "success");

            // Issue session token valid for 7 days (604,800 sec)
            let token = generate_random_token();
            let expiry = now_sec + 604_800;
            self.sessions.insert(token.clone(), expiry);
            Ok(token)
        } else {
            // Failed: update brute-force counter
            let (count, _) = self.failed_attempts.entry(ip.to_string()).or_insert((0, 0));
            *count += 1;
            let current_count = *count;

            if current_count >= 5 {
                // Lock IP for 15 minutes (900 seconds)
                let locked_until = now_sec + 900;
                self.failed_attempts
                    .insert(ip.to_string(), (current_count, locked_until));
                self.record_log(username, ip, user_agent, "locked");
                Err(
                    "Invalid credentials. Account locked for 15 minutes due to 5 failed attempts."
                        .to_string(),
                )
            } else {
                let remaining = 5 - current_count;
                self.record_log(username, ip, user_agent, "failed");
                Err(format!(
                    "Invalid username or password. {remaining} attempt(s) remaining before temporary lockout."
                ))
            }
        }
    }

    pub fn validate_session(&mut self, token: &str) -> bool {
        let now_sec = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // Prune expired sessions occasionally
        if self.sessions.len() > 100 {
            self.sessions.retain(|_, exp| *exp > now_sec);
        }

        if let Some(exp) = self.sessions.get(token) {
            *exp > now_sec
        } else {
            false
        }
    }

    pub fn revoke_session(&mut self, token: &str) {
        self.sessions.remove(token);
    }

    fn record_log(&mut self, username: &str, ip: &str, user_agent: &str, status: &str) {
        let now_sec = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let log = LoginLog {
            id: generate_random_token()[..12].to_string(),
            timestamp: now_sec,
            username: username.to_string(),
            ip: ip.to_string(),
            user_agent: user_agent.to_string(),
            status: status.to_string(),
        };

        self.login_logs.push(log);
        if self.login_logs.len() > 50 {
            self.login_logs.remove(0);
        }
    }

    pub fn list_logs(&self) -> Vec<&LoginLog> {
        self.login_logs.iter().rev().collect()
    }
}
