use std::sync::Mutex;
use std::time::Instant;

use nvml_wrapper::Nvml;
use sysinfo::{
    CpuRefreshKind, Disks, MemoryRefreshKind, Networks, ProcessesToUpdate, RefreshKind, System,
};

use super::gpu::read_first_gpu;
use super::model::{
    CpuCoreInfo, DiskInfo, MetricsResponse, NetworkInterfaceInfo,
};

pub struct SystemMetricsState {
    inner: Mutex<Inner>,
}

struct Inner {
    system: System,
    networks: Networks,
    last_net_refresh: Instant,
    disks: Disks,
    nvml: Option<Nvml>,
}

impl Default for SystemMetricsState {
    fn default() -> Self {
        let mut networks = Networks::new_with_refreshed_list();
        networks.refresh();

        Self {
            inner: Mutex::new(Inner {
                system: System::new_with_specifics(
                    RefreshKind::new()
                        .with_cpu(CpuRefreshKind::everything())
                        .with_memory(MemoryRefreshKind::everything()),
                ),
                networks,
                last_net_refresh: Instant::now(),
                disks: Disks::new_with_refreshed_list(),
                nvml: Nvml::init().ok(),
            }),
        }
    }
}

impl SystemMetricsState {
    pub fn snapshot(&self) -> MetricsResponse {
        let mut g = self.inner.lock().expect("metrics mutex poisoned");
        g.collect()
    }
}

impl Inner {
    fn collect(&mut self) -> MetricsResponse {
        self.system.refresh_memory();
        self.system.refresh_cpu_usage();
        self.system.refresh_processes(ProcessesToUpdate::All);

        let now = Instant::now();
        let dt = (now - self.last_net_refresh).as_secs_f64().max(0.001);
        self.networks.refresh();
        self.last_net_refresh = now;

        let mut network_interfaces: Vec<NetworkInterfaceInfo> = self
            .networks
            .iter()
            .map(|(name, data)| NetworkInterfaceInfo {
                name: name.clone(),
                received_bytes_per_sec: data.received() as f64 / dt,
                transmitted_bytes_per_sec: data.transmitted() as f64 / dt,
                total_received_bytes: data.total_received(),
                total_transmitted_bytes: data.total_transmitted(),
            })
            .collect();
        network_interfaces.sort_by(|a, b| a.name.cmp(&b.name));

        self.disks.refresh();

        let disks: Vec<DiskInfo> = self
            .disks
            .list()
            .iter()
            .map(|d| {
                let total = d.total_space();
                let available = d.available_space();
                DiskInfo {
                    name: d.name().to_string_lossy().into_owned(),
                    mount_point: d.mount_point().display().to_string(),
                    file_system: d.file_system().to_string_lossy().into_owned(),
                    total_bytes: total,
                    available_bytes: available,
                    used_bytes: total.saturating_sub(available),
                    is_removable: d.is_removable(),
                }
            })
            .collect();

        let cpus: Vec<CpuCoreInfo> = self
            .system
            .cpus()
            .iter()
            .map(|c| CpuCoreInfo {
                name: c.name().to_string(),
                usage_percent: c.cpu_usage(),
                frequency_mhz: c.frequency(),
            })
            .collect();

        let load = System::load_average();

        let gpu = self.nvml.as_ref().and_then(read_first_gpu);

        MetricsResponse {
            cpu_usage_percent: self.system.global_cpu_usage(),
            cpus,
            memory_used_bytes: self.system.used_memory(),
            memory_total_bytes: self.system.total_memory(),
            swap_used_bytes: self.system.used_swap(),
            swap_total_bytes: self.system.total_swap(),
            load_average_one: load.one,
            load_average_five: load.five,
            load_average_fifteen: load.fifteen,
            process_count: self.system.processes().len(),
            host_name: System::host_name(),
            os_name: System::name(),
            kernel_version: System::kernel_version(),
            disks,
            network_interfaces,
            gpu,
        }
    }
}
