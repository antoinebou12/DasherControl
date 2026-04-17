use nvml_wrapper::enum_wrappers::device::{Clock, ClockId};
use nvml_wrapper::Nvml;

use super::model::GpuInfo;

pub fn read_first_gpu(nvml: &Nvml) -> Option<GpuInfo> {
    let device = nvml.device_by_index(0).ok()?;
    let name = device.name().ok()?;
    let util = device.utilization_rates().ok()?;
    let mem = device.memory_info().ok()?;
    let graphics_clock_mhz = device.clock(Clock::Graphics, ClockId::Current).ok();
    let sm_clock_mhz = device.clock(Clock::SM, ClockId::Current).ok();
    let memory_clock_mhz = device.clock(Clock::Memory, ClockId::Current).ok();

    Some(GpuInfo {
        name,
        gpu_utilization_percent: util.gpu,
        memory_utilization_percent: util.memory,
        memory_used_bytes: mem.used,
        memory_total_bytes: mem.total,
        graphics_clock_mhz,
        sm_clock_mhz,
        memory_clock_mhz,
    })
}
