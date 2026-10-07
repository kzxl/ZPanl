use serde::Serialize;
use std::fs;
use std::sync::Mutex;
use std::time::Instant;
use zero_sys::proc::{CpuStats, MemStats, NetInterfaceStats};

/// High-level system telemetry report for ZPanl Dashboard.
#[derive(Debug, Clone, Serialize)]
pub struct SystemTelemetry {
    pub cpu_usage_percent: f32,
    pub ram_total_mb: u64,
    pub ram_used_mb: u64,
    pub ram_free_mb: u64,
    pub ram_usage_percent: f32,
    pub net_rx_kbps: u64,
    pub net_tx_kbps: u64,
    pub uptime_seconds: u64,
    pub is_linux_proc: bool,
}

/// Telemetry sampler maintaining state between intervals.
pub struct TelemetryCollector {
    prev_cpu: Mutex<Option<CpuStats>>,
    prev_rx_bytes: Mutex<u64>,
    prev_tx_bytes: Mutex<u64>,
    last_sample: Mutex<Instant>,
    start_time: Instant,
}

impl Default for TelemetryCollector {
    fn default() -> Self {
        Self {
            prev_cpu: Mutex::new(None),
            prev_rx_bytes: Mutex::new(0),
            prev_tx_bytes: Mutex::new(0),
            last_sample: Mutex::new(Instant::now()),
            start_time: Instant::now(),
        }
    }
}

impl TelemetryCollector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Samples current telemetry metrics.
    pub fn sample(&self) -> SystemTelemetry {
        let uptime_seconds = self.start_time.elapsed().as_secs();

        // 1. Try reading Linux /proc
        if let Ok(stat_bytes) = fs::read("/proc/stat") {
            if let Ok(curr_cpu) = CpuStats::parse_stat_bytes(&stat_bytes) {
                let mut prev_guard = self.prev_cpu.lock().unwrap();
                let cpu_usage_percent = if let Some(prev) = *prev_guard {
                    CpuStats::calculate_usage(&prev, &curr_cpu)
                } else {
                    0.0
                };
                *prev_guard = Some(curr_cpu);

                // RAM
                let (ram_total_mb, ram_used_mb, ram_free_mb, ram_usage_percent) =
                    if let Ok(mem_bytes) = fs::read("/proc/meminfo") {
                        if let Ok(mem) = MemStats::parse_meminfo_bytes(&mem_bytes) {
                            (
                                mem.total_kb / 1024,
                                mem.used_kb() / 1024,
                                mem.available_kb / 1024,
                                mem.used_percentage(),
                            )
                        } else {
                            (1024, 256, 768, 25.0)
                        }
                    } else {
                        (1024, 256, 768, 25.0)
                    };

                // Network
                let (net_rx_kbps, net_tx_kbps) = if let Ok(net_bytes) = fs::read("/proc/net/dev") {
                    let mut interfaces = [NetInterfaceStats::default(); 16];
                    let count = NetInterfaceStats::parse_net_dev(&net_bytes, &mut interfaces);

                    let mut total_rx = 0u64;
                    let mut total_tx = 0u64;
                    for item in interfaces.iter().take(count) {
                        let name = item.name_str();
                        if name != "lo" {
                            total_rx += item.rx_bytes;
                            total_tx += item.tx_bytes;
                        }
                    }

                    let mut last_sample_guard = self.last_sample.lock().unwrap();
                    let elapsed_sec = last_sample_guard.elapsed().as_secs_f64().max(0.1);
                    *last_sample_guard = Instant::now();

                    let mut prev_rx_guard = self.prev_rx_bytes.lock().unwrap();
                    let mut prev_tx_guard = self.prev_tx_bytes.lock().unwrap();

                    let rx_delta = total_rx.saturating_sub(*prev_rx_guard);
                    let tx_delta = total_tx.saturating_sub(*prev_tx_guard);

                    *prev_rx_guard = total_rx;
                    *prev_tx_guard = total_tx;

                    let rx_kbps = ((rx_delta as f64) / elapsed_sec / 1024.0) as u64;
                    let tx_kbps = ((tx_delta as f64) / elapsed_sec / 1024.0) as u64;

                    (rx_kbps, tx_kbps)
                } else {
                    (0, 0)
                };

                return SystemTelemetry {
                    cpu_usage_percent,
                    ram_total_mb,
                    ram_used_mb,
                    ram_free_mb,
                    ram_usage_percent,
                    net_rx_kbps,
                    net_tx_kbps,
                    uptime_seconds,
                    is_linux_proc: true,
                };
            }
        }

        // Non-Linux or development fallback: return clean synthetic status
        SystemTelemetry {
            cpu_usage_percent: 4.8,
            ram_total_mb: 4096,
            ram_used_mb: 512,
            ram_free_mb: 3584,
            ram_usage_percent: 12.5,
            net_rx_kbps: 42,
            net_tx_kbps: 18,
            uptime_seconds,
            is_linux_proc: false,
        }
    }
}
