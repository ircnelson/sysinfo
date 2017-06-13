extern crate winapi;
extern crate kernel32;
extern crate num_cpus;

use std::mem::{ size_of };
use std::ptr;
use std::io::Error;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use self::winapi::{ DWORD, MEMORYSTATUSEX, OSVERSIONINFOW, SYSTEM_INFO };
use self::kernel32::{ GetComputerNameW, GetDiskFreeSpaceW, GlobalMemoryStatusEx, GetVersionExW, GetSystemInfo };
use super::{ ResourceResult, ResourceError, Platform, DiskInfo, MemoryInfo, CpuInfo, PlatformStats };

static KB : u64 = 1024;
pub static OS_TYPE : &'static str = "Windows";

impl PlatformStats for Platform {

    fn disk_stats<T>(driver: T) -> ResourceResult<DiskInfo> where T : Into<String> {
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
            Err(Error::last_os_error())
        }
    }

    fn memory_stats() -> ResourceResult<MemoryInfo> {

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
            Err(Error::last_os_error())
        }
    }

    fn os_release() -> ResourceResult<String> {

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
            Err(Error::last_os_error())
        }
    }

    fn os_type() -> String {
        OS_TYPE.to_string()
    }

    fn cpu_stats() -> ResourceResult<CpuInfo> {

        Ok(CpuInfo {
            num_of_processors: num_cpus::get()
        })
    }

    fn computer_name() -> Result<String, ()> {

        const MAX_COMPUTERNAME_LENGTH: usize = 31;

        let mut buf = Vec::<u16>::with_capacity(MAX_COMPUTERNAME_LENGTH + 1);
        unsafe {
            let capacity = buf.capacity();
            buf.set_len(capacity);

            let mut len: DWORD = buf.capacity() as DWORD - 1;
            if GetComputerNameW(buf.as_mut_ptr(), &mut len as *mut DWORD) == winapi::FALSE {
                return Err(());
            }
            buf.set_len(len as usize);
        };

        match String::from_utf16(&buf) {
            Ok(s) => Ok(s),
            Err(_) => Err(()),
        }
    }
}