use std::fs;
use std::process::Command;
use super::CPUInfo;

/*
fn cpufreq_path() -> Option<&'static Path> {
    let candidates = [
        Path::new("/sys/devices/system/cpu/cpufreq/policy0/scaling_max_freq"),
        Path::new("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq"),
    ];

    for candidate in candidates.iter() {
        if candidate.exists() {
            return Some(candidate.clone());
        }
    }

    None
}

pub fn cpu_max_frequency() -> Option<u64> {
    if let Some(path) = cpufreq_path() {
        let content = fs::read_to_string(path).ok()?;
        content.trim().parse::<u64>().ok().map(|v| v * 1000)
    } else {
        // parse output of `lscpu` command
        let output = Command::new("lscpu")
            .env("LC_ALL", "en_US.UTF-8")
            .output()
            .ok()?;

        let stdout = unsafe { String::from_utf8_unchecked(output.stdout) };
        for line in stdout.lines() {
            if line.starts_with("CPU max MHz:") {
                return line[12..]
                    .trim_start()
                    .parse::<f64>()
                    .ok()
                    .map(|v| (v * 1000000.0 + 0.5) as u64);
            }
        }

        None
    }
}
*/

fn cpu_model() -> Option<String> {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("model name\t") {
                let idx = line[11..].find(':')?;
                return Some(line[12 + idx..].trim().to_owned());
            }
        }
        None
    } else {
        let output = Command::new("lscpu")
            .env("LC_ALL", "en_US.UTF-8")
            .output()
            .ok()?;

        let stdout = unsafe { String::from_utf8_unchecked(output.stdout) };
        for line in stdout.lines() {
            if line.starts_with("Model name:") {
                return Some(line[11..].trim().to_owned());
            }
        }
        None
    }
}

pub(super) fn get_cpuinfo() -> CPUInfo {
    CPUInfo {
        cpu_model: cpu_model(),
    }
}
