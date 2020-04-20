// Copyright (c) 2017 Guillaume Gomez
use super::CPUInfo;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(super) fn get_cpuinfo() -> CPUInfo {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::__cpuid;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::__cpuid;

    fn add_u32(v: &mut Vec<u8>, i: u32) {
        let i = &i as *const u32 as *const u8;
        unsafe {
            v.push(*i);
            v.push(*i.offset(1));
            v.push(*i.offset(2));
            v.push(*i.offset(3));
        }
    }

    let mut cpuinfo = CPUInfo::default();

    // First, we try to get the complete name.
    let res = unsafe { __cpuid(0x80000000) };
    let n_ex_ids = res.eax;
    if n_ex_ids >= 0x80000004 {
        let mut extdata = Vec::with_capacity(5);

        for i in 0x80000000..=n_ex_ids {
            extdata.push(unsafe { __cpuid(i) });
        }

        let mut out = Vec::with_capacity(4 * 4 * 3); // 4 * u32 * nb_entries
        for i in 2..5 {
            add_u32(&mut out, extdata[i].eax);
            add_u32(&mut out, extdata[i].ebx);
            add_u32(&mut out, extdata[i].ecx);
            add_u32(&mut out, extdata[i].edx);
        }
        let mut pos = 0;
        for e in out.iter() {
            if *e == 0 {
                break;
            }
            pos += 1;
        }
        match ::std::str::from_utf8(&out[..pos]) {
            Ok(s) if !s.is_empty() => cpuinfo.cpu_model = Some(s.to_owned()),
            _ => {},
        }
    }

    cpuinfo
}

#[cfg(all(not(target_arch = "x86_64"), not(target_arch = "x86")))]
pub(super) fn get_cpuinfo() -> CPUInfo {
    CPUInfo::default()
}
