use std::fs;
use zpanl::waf::{WafRule, WafStorage};

#[test]
fn test_waf_threat_intelligence_workflow() {
    let temp_waf = "target/test_temp_waf.json";
    let _ = fs::remove_file(temp_waf);

    // 1. Initial Load & Default Telemetry Seed
    let mut storage = WafStorage::load_or_init(temp_waf);
    assert_eq!(storage.blacklist_ips.len(), 3);
    assert_eq!(storage.whitelist_ips.len(), 3);
    assert_eq!(storage.blocked_countries.len(), 2);
    assert_eq!(storage.custom_rules.len(), 2);
    assert_eq!(storage.attacks.len(), 6);

    // 2. Summary Generation
    let summary = storage.get_summary();
    assert_eq!(summary.today_requests, 765);
    assert_eq!(summary.malicious_requests, 37);
    assert_eq!(summary.yesterday_requests, 3280);
    assert_eq!(summary.yesterday_malicious, 65);
    assert_eq!(summary.hourly_traffic.len(), 24);
    assert!(!summary.top_attack_ips.is_empty());
    assert_eq!(summary.top_attack_ips[0].ip, "198.51.100.42");
    assert_eq!(summary.top_attack_ips[0].attack_count, 17);

    // 3. IP Rule Management
    assert!(storage.add_ip("203.0.113.55", true));
    assert!(storage.blacklist_ips.contains(&"203.0.113.55".to_string()));
    assert!(!storage.add_ip("203.0.113.55", true)); // Duplicate rejection

    assert!(storage.remove_ip("203.0.113.55", true));
    assert!(!storage.blacklist_ips.contains(&"203.0.113.55".to_string()));

    assert!(storage.add_ip("192.168.1.50", false));
    assert!(storage.whitelist_ips.contains(&"192.168.1.50".to_string()));
    assert!(storage.remove_ip("192.168.1.50", false));

    // 4. Geo-blocking Toggle
    assert!(storage.blocked_countries.contains(&"RU".to_string()));
    let toggled_off = storage.toggle_country("RU");
    assert!(!toggled_off);
    assert!(!storage.blocked_countries.contains(&"RU".to_string()));
    let toggled_on = storage.toggle_country("RU");
    assert!(toggled_on);
    assert!(storage.blocked_countries.contains(&"RU".to_string()));

    // 5. Custom Rules
    let new_rule = WafRule {
        id: "rule_test_1".to_string(),
        name: "Test Rule".to_string(),
        target: "URI".to_string(),
        operator: "contains".to_string(),
        pattern: "wp-login.php".to_string(),
        action: "block".to_string(),
        enabled: true,
    };
    storage.add_rule(new_rule);
    assert_eq!(storage.custom_rules.len(), 3);
    assert!(storage.delete_rule("rule_test_1"));
    assert_eq!(storage.custom_rules.len(), 2);
    assert!(!storage.delete_rule("non_existent"));

    // 6. Persistence & Reload
    assert!(storage.save(temp_waf).is_ok());
    let reloaded = WafStorage::load_or_init(temp_waf);
    assert_eq!(reloaded.blacklist_ips.len(), storage.blacklist_ips.len());
    assert_eq!(reloaded.custom_rules.len(), 2);

    let _ = fs::remove_file(temp_waf);
}
