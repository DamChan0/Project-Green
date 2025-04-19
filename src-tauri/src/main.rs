// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;
use sysinfo::{
    Disks, Networks, Pid, Process, System, MINIMUM_CPU_UPDATE_INTERVAL,
};

#[command]
fn get_system_info() -> serde_json::Value {
    let mut sys: System = System::new_all();
    sys.refresh_all();

    /* ─────────────── CPU ─────────────── */
    let cpu_usage: Vec<f32> = sys.cpus().iter().map(|c| c.cpu_usage()).collect();
    let avg_cpu = if !cpu_usage.is_empty() {
        cpu_usage.iter().sum::<f32>() / cpu_usage.len() as f32
    } else {
        0.0
    };
    // println!("[DEBUG] CPU avg usage: {:.1}%", avg_cpu);

    let cpu_info: serde_json::Value = serde_json::json!({
        "usage": cpu_usage,
        "name": sys.cpus().first().map(|c| c.brand()).unwrap_or("Unknown"),
        "core_count": sys.cpus().len(),
        "frequency": sys.cpus().first().map(|c| c.frequency()).unwrap_or(0),
    });

    /* ─────────────── Memory ─────────────── */
    let total_mem: u64 = sys.total_memory();
    let used_mem: u64  = sys.used_memory();
    let mem_percent = if total_mem > 0 {
        (used_mem as f64 / total_mem as f64 * 100.0) as u64
    } else {
        0
    };
    // println!("[DEBUG] Memory usage: {}/{} ({}%)", used_mem, total_mem, mem_percent);

    let memory_info: serde_json::Value = serde_json::json!({
        "total": total_mem,
        "used":  used_mem,
        "free":  total_mem - used_mem,
        "usage_percent": mem_percent,
    });

    /* ─────────────── Disks ─────────────── */
    let disks: Disks = Disks::new_with_refreshed_list();
    let disks_info: Vec<_> = disks.list().iter().map(|d| {
        let total: u64 = d.total_space();
        let avail: u64 = d.available_space();
        let usage_percent = if total > 0 {
            ((total - avail) as f64 / total as f64 * 100.0) as u64
        } else {
            0
        };
        // println!(
        //     "[DEBUG] Disk {:?} usage: {}/{} ({}%)",
        //     d.mount_point(),
        //     total - avail,
        //     total,
        //     usage_percent
        // );
        serde_json::json!({
            "name":   d.name().to_string_lossy(),
            "mount_point": d.mount_point().to_string_lossy(),
            "total_space": total,
            "available_space": avail,
            "usage_percent": usage_percent
        })
    }).collect();

    /* ─────────────── Networks ─────────────── */
    let networks: Networks = Networks::new_with_refreshed_list();
    let networks_info: Vec<_> = networks.iter().map(|(name, data)| {
        // println!(
        //     "[DEBUG] Network {} - RX: {}, TX: {}",
        //     name,
        //     data.total_received(),
        //     data.total_transmitted()
        // );
        serde_json::json!({
            "name": name,
            "received": data.total_received(),
            "transmitted": data.total_transmitted(),
            "rx_packets": data.total_packets_received(),
            "tx_packets": data.total_packets_transmitted(),
        })
    }).collect();

    /* ─────────────── Processes (Top‑10 by CPU) ─────────────── */
    let mut processes: Vec<(&Pid, &Process)> = sys.processes().iter().collect();
    processes.sort_by(|a: &(&Pid, &Process), b: &(&Pid, &Process)| {
        b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap()
    });

    let processes_info: Vec<_> = processes.iter().take(10).map(|(pid, p)| {
        // println!(
        //     "[DEBUG] Process PID={} Name={:?} CPU={:.1}%",
        //     pid,
        //     p.name(),
        //     p.cpu_usage()
        // );
        serde_json::json!({
            "pid": pid.to_string(),
            "name": p.name(),
            "cpu_usage": p.cpu_usage(),
            "memory_usage": p.memory(),
            "start_time": p.start_time(),
        })
    }).collect();

    /* ─────────────── Static system info ─────────────── */
    let system_info: serde_json::Value = serde_json::json!({
        "hostname":        System::host_name().unwrap_or_else(|| "Unknown".into()),
        "os_name":         System::name().unwrap_or_else(|| "Unknown".into()),
        "os_version":      System::os_version().unwrap_or_else(|| "Unknown".into()),
        "kernel_version":  System::kernel_version().unwrap_or_else(|| "Unknown".into()),
        "uptime":          System::uptime(),
    });

    serde_json::json!({
        "cpu":       cpu_info,
        "memory":    memory_info,
        "disks":     disks_info,
        "networks":  networks_info,
        "processes": processes_info,
        "system":    system_info,
    })
}

#[command]
fn get_realtime_stats() -> serde_json::Value {
    let mut sys = System::new_all();

    std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();
    let cpu_usage: Vec<f32> = sys.cpus().iter().map(|c| c.cpu_usage()).collect();
    let avg_cpu = if cpu_usage.is_empty() {
        0.0
    } else {
        cpu_usage.iter().sum::<f32>() / cpu_usage.len() as f32
    };

    sys.refresh_memory();
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let mem_percent = if total_mem > 0 {
        (used_mem as f64 / total_mem as f64 * 100.0) as u64
    } else {
        0
    };

    let mut nets = Networks::new_with_refreshed_list();
    std::thread::sleep(std::time::Duration::from_millis(10));
    nets.refresh(true);
    let network_info: Vec<_> = nets.iter().map(|(name, data)| {
        // println!(
        //     "[DEBUG] Network {} - RX: {}, TX: {}",
        //     name,
        //     data.received(),
        //     data.transmitted()
        // );
        serde_json::json!({
            "name": name,
            "received": data.received(),
            "transmitted": data.transmitted()
        })
    }).collect();

    // 🔽 로그 출력
    // println!("[DEBUG] avg CPU usage: {:.1}%", avg_cpu);
    // println!("[DEBUG] mem usage: {} / {} = {}%", used_mem, total_mem, mem_percent);

    serde_json::json!({
        "cpu_usage": avg_cpu,
        "detailed_cpu_usage": cpu_usage,
        "memory_usage_percent": mem_percent,
        "memory_used": used_mem,
        "memory_total": total_mem,
        "networks": network_info
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_system_info, get_realtime_stats])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
