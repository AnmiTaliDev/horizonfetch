use std::fs;
use sysinfo::{Disks, System};

pub struct SystemInfo {
    pub username: String,
    pub hostname: String,
    pub os_name: String,
    pub kernel: String,
    pub uptime: String,
    pub shell: String,
    pub de: String,
    pub screen: Option<String>,
    pub motherboard: Option<String>,
    pub cpu: String,
    pub gpu: Vec<String>,
    pub ram_used_gb: f64,
    pub ram_total_gb: f64,
    pub ram_percent: f64,
    pub swap_total_gb: f64,
    pub locale: String,
    pub disks: Vec<DiskInfo>,
}

pub struct DiskInfo {
    pub name: String,
    pub used_gb: u64,
    pub total_gb: u64,
    pub percent: u64,
}

impl SystemInfo {
    pub fn gather() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        SystemInfo {
            username: get_username(),
            hostname: get_hostname(),
            os_name: get_os_name(),
            kernel: get_kernel(),
            uptime: get_uptime(&sys),
            shell: get_shell(),
            de: get_de(),
            screen: get_screen_resolution(),
            motherboard: get_motherboard(),
            cpu: get_cpu(&sys),
            gpu: get_gpu(),
            ram_used_gb: get_ram_used(&sys),
            ram_total_gb: get_ram_total(&sys),
            ram_percent: get_ram_percent(&sys),
            swap_total_gb: get_swap_total(&sys),
            locale: get_locale(),
            disks: get_disks(),
        }
    }
}

fn get_username() -> String {
    std::env::var("USER").unwrap_or_else(|_| "unknown".to_string())
}

fn get_hostname() -> String {
    fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string()
}

fn get_os_name() -> String {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                return line
                    .trim_start_matches("PRETTY_NAME=")
                    .trim_matches('"')
                    .to_string();
            }
        }
    }
    "Linux".to_string()
}

fn get_kernel() -> String {
    fs::read_to_string("/proc/sys/kernel/osrelease")
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string()
}

fn get_uptime(_sys: &System) -> String {
    let uptime_sec = System::uptime();
    let uptime_min = uptime_sec / 60;
    let uptime_hr = uptime_min / 60;
    let uptime_days = uptime_hr / 24;

    if uptime_days > 0 {
        format!("{}d {}h {}m", uptime_days, uptime_hr % 24, uptime_min % 60)
    } else {
        format!("{}h {}m", uptime_hr % 24, uptime_min % 60)
    }
}

fn get_shell() -> String {
    std::env::var("SHELL")
        .ok()
        .and_then(|s| s.split('/').last().map(String::from))
        .unwrap_or_else(|| "unknown".to_string())
}

fn get_de() -> String {
    std::env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| std::env::var("DESKTOP_SESSION"))
        .or_else(|_| std::env::var("XDG_SESSION_DESKTOP"))
        .unwrap_or_else(|_| "Unknown".to_string())
}

fn get_screen_resolution() -> Option<String> {
    // Try to get resolution from xrandr
    if let Ok(output) = std::process::Command::new("xrandr").output() {
        if let Ok(text) = String::from_utf8(output.stdout) {
            for line in text.lines() {
                if line.contains("*") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if !parts.is_empty() {
                        return Some(parts[0].to_string());
                    }
                }
            }
        }
    }
    None
}

fn get_motherboard() -> Option<String> {
    fs::read_to_string("/sys/class/dmi/id/board_name")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && s != "Default string")
}

fn get_cpu(sys: &System) -> String {
    sys.cpus()
        .first()
        .map(|cpu| {
            let brand = cpu.brand().trim();
            let count = sys.cpus().len();
            if count > 1 {
                format!("{} ({} threads)", brand, count)
            } else {
                brand.to_string()
            }
        })
        .unwrap_or_else(|| "Unknown".to_string())
}

fn get_gpu() -> Vec<String> {
    let mut gpus = Vec::new();

    // Try lspci for GPU info
    if let Ok(output) = std::process::Command::new("lspci").output() {
        if let Ok(text) = String::from_utf8(output.stdout) {
            for line in text.lines() {
                if line.contains("VGA") || line.contains("3D") || line.contains("Display") {
                    if let Some(gpu_name) = line.split(':').nth(2) {
                        gpus.push(gpu_name.trim().to_string());
                    }
                }
            }
        }
    }

    if gpus.is_empty() {
        gpus.push("Unknown".to_string());
    }

    gpus
}

fn get_ram_used(sys: &System) -> f64 {
    (sys.used_memory() as f64) / 1_073_741_824.0
}

fn get_ram_total(sys: &System) -> f64 {
    (sys.total_memory() as f64) / 1_073_741_824.0
}

fn get_ram_percent(sys: &System) -> f64 {
    let total = sys.total_memory() as f64;
    if total > 0.0 {
        (sys.used_memory() as f64 * 100.0) / total
    } else {
        0.0
    }
}

fn get_swap_total(sys: &System) -> f64 {
    (sys.total_swap() as f64) / 1_073_741_824.0
}

fn get_locale() -> String {
    std::env::var("LANG").unwrap_or_else(|_| "en_US.UTF-8".to_string())
}

fn get_disks() -> Vec<DiskInfo> {
    let disks = Disks::new_with_refreshed_list();
    let mut result = Vec::new();

    for disk in disks.list() {
        let name = disk.mount_point().to_str().unwrap_or("/").to_string();
        let total_bytes = disk.total_space();
        let available_bytes = disk.available_space();
        let used_bytes = total_bytes.saturating_sub(available_bytes);

        let total_gb = total_bytes / 1_073_741_824;
        let used_gb = used_bytes / 1_073_741_824;
        let percent = if total_gb > 0 {
            (used_gb * 100) / total_gb
        } else {
            0
        };

        if total_gb > 0 {
            result.push(DiskInfo {
                name,
                used_gb,
                total_gb,
                percent,
            });
        }
    }

    result
}
