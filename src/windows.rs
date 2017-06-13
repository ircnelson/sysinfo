extern crate winapi;
extern crate kernel32;
extern crate num_cpus;

use std::os::raw::{ c_void };
use std::mem::{ size_of, size_of_val, uninitialized };
use std::io::Error;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

use self::winapi::{ DWORD, MEMORYSTATUSEX, OSVERSIONINFOW, LPFILETIME, SYSTEMTIME };
use self::kernel32::{ GetComputerNameW, GetDiskFreeSpaceW, GlobalMemoryStatusEx, GetVersionExW, GetProcessTimes, GetCurrentProcess, FileTimeToSystemTime };
use super::{ ResourceResult, Platform, DiskInfo, MemoryInfo, CpuInfo, PlatformStats };

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

        let mem_stat = unsafe {
            let mut mem_stat: MEMORYSTATUSEX = uninitialized();

            let len = size_of::<MEMORYSTATUSEX>() as u32;

            mem_stat.dwLength = len;

            if GlobalMemoryStatusEx(&mut mem_stat) == 1 {
                Some(mem_stat)
            } else {
                None
            }
        };

        match mem_stat {
            Some(res) => Ok(MemoryInfo {
                total: res.ullTotalPhys / KB,
                avail: 0,
                free: res.ullAvailPhys / KB,
                cached: 0,
                buffers: 0,
                swap_total: res.ullTotalPageFile / KB,
                swap_free: res.ullAvailPageFile / KB
            }),
            None => Err(Error::last_os_error())
        }
    }

    fn os_release() -> ResourceResult<String> {

        let os_version = unsafe {

            let mut os_version: OSVERSIONINFOW = uninitialized();

            let len = size_of::<OSVERSIONINFOW>() as u32;

            os_version.dwOSVersionInfoSize = len;

            if GetVersionExW(&mut os_version) == 1 {
                Some(os_version)
            } else {
                None
            }
        };

        match os_version {
            Some(res) => Ok(format!("{}.{}.{}", res.dwMajorVersion, res.dwMinorVersion, res.dwBuildNumber)),
            None => Err(Error::last_os_error())
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