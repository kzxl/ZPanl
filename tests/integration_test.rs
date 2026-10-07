use std::fs;
use zpanl::cron::{CronDatabase, CronJob};
use zpanl::database::{DatabaseRecord, DatabaseStorage};
use zpanl::filemgr::FileManager;
use zpanl::services::ServiceManager;
use zpanl::site::{SiteDatabase, SiteKind, SiteRecord};
use zpanl::telemetry::TelemetryCollector;

#[test]
fn test_site_database_workflow() {
    let mut db = SiteDatabase::default();

    let site1 = SiteRecord {
        id: "s1".to_string(),
        domain: "app.example.com".to_string(),
        root_path: "/var/www/app.example.com".to_string(),
        kind: SiteKind::SpaFallback,
        php_version: None,
        ssl_enabled: true,
        created_at: 1000,
        ..Default::default()
    };

    let site2 = SiteRecord {
        id: "s2".to_string(),
        domain: "wp.example.com".to_string(),
        root_path: "/var/www/wp.example.com".to_string(),
        kind: SiteKind::PhpFpm,
        php_version: Some("8.3".to_string()),
        ssl_enabled: true,
        created_at: 2000,
        ..Default::default()
    };

    assert!(db.add(site1).is_ok());
    assert!(db.add(site2).is_ok());
    assert_eq!(db.list().len(), 2);

    // Duplicate rejection
    let dup = SiteRecord {
        id: "s3".to_string(),
        domain: "app.example.com".to_string(),
        root_path: "/var/www/other".to_string(),
        kind: SiteKind::Static,
        php_version: None,
        ssl_enabled: true,
        created_at: 3000,
        ..Default::default()
    };
    assert!(db.add(dup).is_err());

    // Caddyfile generation
    let caddyfile = db.generate_caddyfile();
    assert!(caddyfile.contains("app.example.com"));
    assert!(caddyfile.contains("try_files {path} /index.html"));
    assert!(caddyfile.contains("wp.example.com"));
    assert!(caddyfile.contains("php_fastcgi unix//run/php/php8.3-fpm.sock"));

    // PHP Pool generation
    let pool_ini = db.generate_php_pool("wp.example.com").unwrap();
    assert!(pool_ini.contains("[wp.example.com]"));
    assert!(pool_ini.contains("listen = /run/php/zpanl-wp_example_com-8.3.sock"));

    // Removal
    assert!(db.remove("app.example.com").is_ok());
    assert_eq!(db.list().len(), 1);
}

#[test]
fn test_filemgr_security_and_operations() {
    let temp_root = std::env::temp_dir().join("zpanl_test_vfs");
    let _ = fs::remove_dir_all(&temp_root);
    fs::create_dir_all(&temp_root).unwrap();

    let root_str = temp_root.to_string_lossy().to_string();

    // 1. Create file & atomic save
    assert!(FileManager::create_entry(&root_str, "index.php", false).is_ok());
    assert!(FileManager::save_file(&root_str, "index.php", "<?php phpinfo(); ?>").is_ok());

    // 2. Read file
    let (content, mime) = FileManager::read_file(&root_str, "index.php").unwrap();
    assert_eq!(content, "<?php phpinfo(); ?>");
    assert_eq!(mime, "application/x-httpd-php");

    // 3. Create subdirectory and nested file
    assert!(FileManager::create_entry(&root_str, "css", true).is_ok());
    assert!(FileManager::save_file(&root_str, "css/main.css", "body { color: #fff; }").is_ok());

    // 4. List directory
    let entries = FileManager::list_dir(&root_str, "").unwrap();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].name, "css");
    assert_eq!(entries[0].file_type, "Directory");
    assert_eq!(entries[1].name, "index.php");
    assert_eq!(entries[1].file_type, "File");

    // 5. Security Jail Traversal Attack Prevention
    assert!(FileManager::read_file(&root_str, "../../etc/passwd").is_err());
    assert!(FileManager::list_dir(&root_str, "../..").is_err());
    assert!(FileManager::save_file(&root_str, "css/../../evil.sh", "#!/bin/sh").is_err());

    // 6. Delete
    assert!(FileManager::delete_entry(&root_str, "css/main.css").is_ok());
    assert!(FileManager::delete_entry(&root_str, "css").is_ok());
    assert!(FileManager::delete_entry(&root_str, "index.php").is_ok());

    // Root directory deletion protection
    assert!(FileManager::delete_entry(&root_str, "").is_err());

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_telemetry_and_services() {
    let collector = TelemetryCollector::new();
    let stats = collector.sample();
    assert!(stats.ram_total_mb > 0);

    let services = ServiceManager::list_services();
    assert_eq!(services.len(), 4);
    assert_eq!(services[0].name, "caddy");
}

#[test]
fn test_site_modification_and_advanced_features() {
    let mut db = SiteDatabase::default();

    let mut site = SiteRecord {
        id: "laravel_site".to_string(),
        domain: "myapi.com".to_string(),
        aliases: vec!["api.myapi.com".to_string(), "backend.internal".to_string()],
        port: 8080,
        root_path: "/var/www/myapi".to_string(),
        running_dir: Some("/public".to_string()),
        kind: SiteKind::PhpFpm,
        php_version: Some("8.2".to_string()),
        rewrite_preset: Some("laravel".to_string()),
        ssl_enabled: true,
        ..Default::default()
    };

    assert!(db.add(site.clone()).is_ok());

    // Generate Caddyfile and verify
    let caddyfile = db.generate_site_caddyfile(&site);
    assert!(caddyfile.contains("myapi.com:8080, api.myapi.com:8080, backend.internal:8080"));
    assert!(caddyfile.contains("root * /var/www/myapi/public"));
    assert!(caddyfile.contains("try_files {path} {path}/ /index.php?{query}"));

    // Test Maintenance Mode
    site.maintenance = true;
    let maint_caddyfile = db.generate_site_caddyfile(&site);
    assert!(maint_caddyfile.contains("respond 503"));

    // Test Update in Database
    site.maintenance = false;
    site.proxy_upstream = Some("127.0.0.1:3000".to_string());
    site.ip_blacklist = vec!["10.0.0.1".to_string(), "192.168.1.0/24".to_string()];
    site.basic_auth_user = Some("admin".to_string());
    site.basic_auth_pass = Some("secret".to_string());
    site.hotlink_protection = true;
    site.redirects = vec![zpanl::site::RedirectRule {
        source_path: "/old".to_string(),
        target_url: "/new".to_string(),
        code: 301,
    }];

    assert!(db.update("myapi.com", site.clone()).is_ok());
    let updated_caddy = db.generate_site_caddyfile(db.find_by_domain("myapi.com").unwrap());
    assert!(updated_caddy.contains("reverse_proxy 127.0.0.1:3000"));
    assert!(updated_caddy.contains("@blocked_ips"));
    assert!(updated_caddy.contains("remote_ip 10.0.0.1 192.168.1.0/24"));
    assert!(updated_caddy.contains("basicauth *"));
    assert!(updated_caddy.contains("admin secret"));
    assert!(updated_caddy.contains("@hotlink"));
    assert!(updated_caddy.contains("redir /old /new 301"));
    assert!(updated_caddy.contains("log {"));
}

#[test]
fn test_database_manager_workflow() {
    let mut storage = DatabaseStorage::default();

    let db1 = DatabaseRecord {
        id: "db-1".to_string(),
        name: "wordpress_prod".to_string(),
        engine: "mysql".to_string(),
        collation: "utf8mb4_unicode_ci".to_string(),
        username: "wp_user".to_string(),
        password: Some("secret_password".to_string()),
        host: "127.0.0.1".to_string(),
        site: Some("wp.example.com".to_string()),
        size_bytes: 1048576,
        created_at: 1700000000,
    };

    let db2 = DatabaseRecord {
        id: "db-2".to_string(),
        name: "analytics_db".to_string(),
        engine: "postgres".to_string(),
        collation: "utf8".to_string(),
        username: "pg_admin".to_string(),
        password: Some("pg_password".to_string()),
        host: "%".to_string(),
        site: None,
        size_bytes: 5242880,
        created_at: 1700001000,
    };

    assert!(storage.add(db1.clone()).is_ok());
    assert!(storage.add(db2.clone()).is_ok());
    assert_eq!(storage.list().len(), 2);

    // Duplicate rejection
    assert!(storage.add(db1.clone()).is_err());

    // SQL dump generation
    let dump = storage.generate_dump("wordpress_prod").unwrap();
    assert!(dump.contains("ZPanl Sovereign Database Dump"));
    assert!(dump.contains("wordpress_prod"));
    assert!(dump.contains("CREATE DATABASE IF NOT EXISTS `wordpress_prod`"));

    // Find and Deletion
    assert!(storage.find("wordpress_prod").is_some());
    assert!(storage.delete("wordpress_prod"));
    assert_eq!(storage.list().len(), 1);
    assert!(storage.generate_dump("wordpress_prod").is_err());
    assert!(storage.find("wordpress_prod").is_none());
}

#[test]
fn test_cron_manager_workflow() {
    let mut cron_db = CronDatabase::default();

    let job1 = CronJob {
        id: "job-1".to_string(),
        name: "Echo Test Task".to_string(),
        schedule: "* * * * *".to_string(),
        command: "echo 'ZPanl Cron Test'".to_string(),
        enabled: true,
        site: Some("app.example.com".to_string()),
        created_at: 1700000000,
        last_run_at: None,
        last_status: None,
        last_output: None,
    };

    assert!(cron_db.add(job1).is_ok());
    assert_eq!(cron_db.list().len(), 1);

    // Toggle job
    assert_eq!(cron_db.toggle("job-1"), Some(false));
    assert!(!cron_db.find("job-1").unwrap().enabled);
    assert_eq!(cron_db.toggle("job-1"), Some(true));
    assert!(cron_db.find("job-1").unwrap().enabled);

    // Execute job synchronously
    let result = cron_db.execute_job("job-1");
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("ZPanl Cron Test"));

    let job_after = cron_db.find("job-1").unwrap();
    assert!(job_after.last_run_at.is_some());
    assert_eq!(job_after.last_status.as_deref(), Some("success"));

    // Check logs retrieval
    let logs = cron_db.get_logs("job-1");
    assert!(logs.is_some());
    assert!(logs.unwrap().contains("ZPanl Cron Test"));

    // Delete job
    assert!(cron_db.delete("job-1"));
    assert_eq!(cron_db.list().len(), 0);
    assert!(cron_db.get_logs("job-1").is_none());
}
