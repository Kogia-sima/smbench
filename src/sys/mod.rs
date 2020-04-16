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
}

pub fn get_sysinfo() -> SysInfo {
    let mut info = SysInfo {
        os: None,
        architecture: None,
        cpu_model: cpu_model(),
    };

    if let Some(platform) = platforms::guess_current() {
        info.os = Some(platform.target_os);
        info.architecture = Some(platform.target_arch)
    }

    info
}
