extern crate winapi;
extern crate kernel32;

use std::process::Command;
use std::mem::{ size_of };
use std::ptr;
use std::io::Error;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use self::winapi::{ MEMORYSTATUSEX, OSVERSIONINFOW, SYSTEM_INFO };
use self::kernel32::{ GetDiskFreeSpaceW, GlobalMemoryStatusEx, GetVersionExW, GetSystemInfo };
use super::{ ResourceResult, ResourceError, Platform, DiskInfo, MemoryInfo, CpuInfo, PlatformStats };

static KB : u64 = 1024;
static OS_TYPE : &'static str = "Windows";

impl PlatformStats for Platform {

    fn disk_stats<'a, T>(driver: T) -> ResourceResult<'a, DiskInfo> where T : Into<String> {
        let disk: Vec<u16> = OsStr::new(&driver.into()).encode_wide().chain(once(0)).collect();

        let mut cluster = 0;
        let mut sector = 0;
        let mut free = 0;
        let mut total = 0;

        let ret = unsafe { GetDiskFreeSpaceW(disk.as_ptr(), &mut cluster, &mut sector, &mut free, &mut total) == 1 };

        if ret {
            let tmp = (cluster * sector) as u64;

            let disk_free_space = DiskInfo {
                total: tmp * (total as u64),
                free: tmp * (free as u64)
            };

            Ok(disk_free_space)
        } else {
            Err(ResourceError::IO(Error::last_os_error()))
        }
    }

    fn memory_stats<'a>() -> ResourceResult<'a, MemoryInfo> {

        let len = size_of::<MEMORYSTATUSEX>() as u32;

        let mut stat = MEMORYSTATUSEX {
            dwLength: len,
            dwMemoryLoad: 0,
            ullTotalPhys: 0,
            ullAvailPhys: 0,
            ullTotalPageFile: 0,
            ullAvailPageFile: 0,
            ullTotalVirtual: 0,
            ullAvailVirtual: 0,
            ullAvailExtendedVirtual: 0
        };

        let ret = unsafe { GlobalMemoryStatusEx(&mut stat) == 1 };

        if ret {
            let mem_info = MemoryInfo {
                total: stat.ullTotalPhys / KB,
                avail: 0,
                free: stat.ullAvailPhys / KB,
                cached: 0,
                buffers: 0,
                swap_total: stat.ullTotalPageFile / KB,
                swap_free: stat.ullAvailPageFile / KB
            };

            Ok(mem_info)
        } else {
            Err(ResourceError::Generic("cannot get memory information"))
        }
    }

    fn os_release<'a>() -> ResourceResult<'a, String> {

        let len = size_of::<OSVERSIONINFOW>() as u32;

        let mut os_version = OSVERSIONINFOW {
            dwOSVersionInfoSize: len,
            dwMajorVersion: 0,
            dwMinorVersion: 0,
            dwBuildNumber: 0,
            dwPlatformId: 0,
            szCSDVersion: [0; 128]
        };

        let ret = unsafe { GetVersionExW(&mut os_version) == 1 };

        if ret {
            Ok(format!("{}.{}", os_version.dwMajorVersion, os_version.dwMinorVersion))
        } else {
            Err(ResourceError::Generic("cannot get OS version"))
        }
    }

    fn os_type<'a>() -> &'a str {
        OS_TYPE
    }

    fn cpu_stats<'a>() -> ResourceResult<'a, CpuInfo> {

        let mut sys_info = SYSTEM_INFO {
            wProcessorArchitecture: 0,
            wReserved: 0,
            dwPageSize: 0,
            lpMinimumApplicationAddress: ptr::null_mut(),
            lpMaximumApplicationAddress: ptr::null_mut(),
            dwActiveProcessorMask: 0,
            dwNumberOfProcessors: 0,
            dwProcessorType: 0,
            dwAllocationGranularity: 0,
            wProcessorLevel: 0,
            wProcessorRevision: 0,
        };

        unsafe { GetSystemInfo(&mut sys_info) };

        Ok(CpuInfo { num_of_processors: sys_info.dwNumberOfProcessors })
    }

    fn computer_name<'a>() -> ResourceResult<'a, String> {

        let output = match Command::new("hostname").output() {
            Ok(o) => o,
            Err(_) => return Err(ResourceError::Generic("cannot get computer name")),
        };

        let mut s = String::from_utf8(output.stdout).unwrap();
        s.pop();
        s.pop();

        Ok(s)
    }
}

#[cfg(test)]
mod windows_tests {

    use super::{Platform, PlatformStats};

    #[test]
    fn disk() {
        let r = Platform::disk_stats("C:\\");

        match r {
            Ok(stat) => assert!(stat.free > 0 && stat.total > 0),
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn memory() {
        let r = Platform::memory_stats();

        match r {
            Ok(m) => assert!(m.total > 0),
            Err(_) => assert!(false)
        };
    }

    #[test]
    fn os_release() {
        let r = Platform::os_release();

        match r {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        };
    }

    #[test]
    fn os_type() {
        let r = Platform::os_type();

        assert_eq!(r, super::OS_TYPE.to_string());
    }

    #[test]
    fn cpu() {
        let r = Platform::cpu_stats();

        match r {
            Ok(stat) => assert!(stat.num_of_processors > 0),
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn computer_name() {
        let r = Platform::computer_name();

        match r {
            Ok(name) => assert!(name.len() > 0),
            Err(_) => assert!(false)
        }
    }
}