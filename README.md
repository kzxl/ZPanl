# ZPanl ⚡🌐

> **Sovereign, Ultra-Lightweight Linux Web Control Panel for Static Websites & PHP-FPM.**  
> Built in Pure Rust as part of the [`ZeroUniverse`](https://github.com/kzxl/ZeroUniverse) ecosystem.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Pure Rust](https://img.shields.io/badge/Rust-100%25-orange.svg)]()
[![Memory Footprint](https://img.shields.io/badge/RAM-%3C%2010%20MB-brightgreen.svg)]()

---

## 🌟 Overview

**ZPanl** is a sovereign alternative to heavy, monolithic hosting panels (aaPanel, cPanel, Plesk) engineered specifically for **static websites, modern SPAs, and dynamic PHP-FPM applications**.

Where traditional panels consume 500 MB – 1 GB of RAM running Python/Node/PHP daemon stacks, **ZPanl** compiles into a **single, self-contained binary (< 5 MB)** that consumes **under 10 MB of RAM** while providing:

- 📊 **Zero-Alloc Linux Telemetry**: Real-time CPU, RAM, and Network traffic parsed directly from `/proc/stat`, `/proc/meminfo`, and `/proc/net/dev`.
- 🌐 **Automated Caddy v2 Integration**: Automatic Let's Encrypt / ZeroSSL HTTPS, HTTP/3, and reverse proxy routing without manual config editing.
- 🐘 **Multi-Version PHP-FPM Engine**: Isolated worker pools (`pool.d/*.conf`) for PHP 8.1, 8.2, and 8.3 via fast Unix domain sockets.
- 📁 **Jailed Web File Manager**: Path Traversal attack prevention (`../../etc/passwd` rejection) with in-browser text editing and atomic file staging (`.tmp` + rename).
- ⚙️ **Direct Systemd Management**: Inspect, reload, and restart `caddy`, `php-fpm`, and `mariadb` services.
- 🖥️ **Embedded Single-Page Web UI**: High-speed, responsive Dark Theme dashboard embedded directly inside the binary (`0` external CDN or runtime dependencies).

---

## 🏛️ Architecture

```
┌────────────────────────────────────────────────────────────────────────┐
│                          Browser Client                                │
│        Embedded Dark Glassmorphism SPA (HTML5 / CSS / Vanilla JS)      │
└───────────────────────────────────▲────────────────────────────────────┘
                                    │ HTTP/1.1 REST & Static Assets
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                         ZPanl Daemon (`zpanl`)                         │
├───────────────────────────────────┬────────────────────────────────────┤
│           zero-sys                │              zero-caddy            │
│ • Zero-alloc `/proc` Telemetry    │ • Caddyfile & JSON Route Builders  │
│ • Systemd Service Actions         │ • Admin API Dynamic Reload         │
├───────────────────────────────────┼────────────────────────────────────┤
│         zero-fastcgi              │              zero-vfs              │
│ • FastCGI v1.0 Binary Framing     │ • Directory Traversal Jail Sandbox │
│ • PHP-FPM Worker Pool INI Engine  │ • Atomic Staging File Replacement  │
└─────────────────┬─────────────────┴──────────────────┬─────────────────┘
                  │                                    │
                  ▼                                    ▼
     ┌────────────────────────┐           ┌────────────────────────┐
     │   Caddy v2 Web Server  │           │   PHP-FPM Process Pool │
     │  Automatic HTTPS / QUIC│           │   php8.1 / 8.2 / 8.3   │
     └────────────────────────┘           └────────────────────────┘
```

---

## 🚀 Quick Start

### 1. Build and Run

```bash
# Clone and build
git clone https://github.com/kzxl/ZeroUniverse.git
cd ZeroUniverse/ZeroApps/ZPanl
cargo build --release

# Run panel daemon (default http://0.0.0.0:8888)
./target/release/zpanl run --bind 0.0.0.0:8888
```

Open your browser and navigate to `http://<server-ip>:8888`.

### 2. Command Line Interface (CLI)

```bash
# View real-time Linux telemetry
zpanl telemetry

# Inspect status of Caddy and PHP-FPM
zpanl services

# Create a virtual host
zpanl site add blog.example.com /var/www/blog.example.com php 8.2
zpanl site add app.example.com /var/www/app.example.com spa

# List registered sites
zpanl site list

# Export synchronized Caddyfile
zpanl export-caddy > /etc/caddy/Caddyfile
```

---

## 🐧 Systemd Daemon Setup (`zpanld.service`)

Create `/etc/systemd/system/zpanld.service`:

```ini
[Unit]
Description=ZPanl Sovereign Web Control Panel
After=network.target caddy.service

[Service]
Type=simple
User=root
WorkingDirectory=/etc/zpanl
ExecStart=/usr/local/bin/zpanl run --bind 0.0.0.0:8888 --db /etc/zpanl/sites.json
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl daemon-reload
sudo systemctl enable --now zpanld
```

---

## 📜 License

Licensed under the [MIT License](LICENSE).
