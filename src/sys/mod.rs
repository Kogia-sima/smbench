#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use self::macos::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use self::windows::*;

#[cfg(all(unix, not(target_os = "macos")))]
mod linux;
#[cfg(all(unix, not(target_os = "macos")))]
use self::linux::*;

#[derive(Clone, Debug)]
pub struct SysInfo {
    pub os: Option<platforms::target::OS>,
    pub architecture: Option<platforms::target::Arch>,
    pub cpu_model: Option<String>,
    pub intel_turbo: Option<bool>,
}

#[derive(Default)]
struct CPUInfo {
    cpu_model: Option<String>,
    intel_turbo: Option<bool>,
}

pub fn get_sysinfo() -> SysInfo {
    let cpuinfo = get_cpuinfo();

    let mut info = SysInfo {
        os: None,
        architecture: None,
        cpu_model: cpuinfo.cpu_model,
        intel_turbo: cpuinfo.intel_turbo,
    };

    if let Some(platform) = platforms::guess_current() {
        info.os = Some(platform.target_os);
        info.architecture = Some(platform.target_arch)
    }

    info
}
