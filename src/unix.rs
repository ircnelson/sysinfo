extern crate libc;
extern crate num_cpus;

use std::ptr;
use std::mem;
use std::io;
use self::libc::{ c_void, c_int, c_schar, c_uchar, size_t, uid_t, sysctl, sysctlnametomib };
use super::{ ResourceResult, Platform, DiskInfo, MemoryInfo, CpuInfo, PlatformStats };

#[cfg(target_os = "macos")]
pub static OS_TYPE : &'static str = "Darwin";
#[cfg(not(target_os = "macos"))]
pub static OS_TYPE : &'static str = "Linux";

macro_rules! sysctl {
    ($mib:expr, $dataptr:expr, $size:expr, $shouldcheck:expr) => {
        {
            let mut mib = &mut $mib;
            let mut size = $size;

            if unsafe { sysctl(&mut mib[0], mib.len() as u32,
                               $dataptr as *mut _ as *mut c_void,
                               &mut size,
                               ptr::null_mut(), 0) } != 0 && $shouldcheck {

                return Err(io::Error::new(io::ErrorKind::Other, "sysctl() failed"))

            }
            size
        }
    };
    ($mib:expr, $dataptr:expr, $size:expr) => {
        sysctl!($mib, $dataptr, $size, true)
    }
}

static KERN_OSRELEASE: [c_int; 2] = [1, 2];


impl PlatformStats for Platform {
    fn disk_stats<'a, T>(driver: T) -> ResourceResult<DiskInfo> {
        unimplemented!()
    }

    fn memory_stats<'a>() -> ResourceResult<MemoryInfo> { unimplemented!() }

    fn os_release<'a>() -> ResourceResult<String> {
        /*
        let mut active: usize = 0;

        sysctl!(&mut [1, 2], &mut active, mem::size_of::<usize>());

        println!("{}", String::from_utf16(&active));
        */

        let mut data : usize = 0;

        let mut len = mem::size_of::<usize>();

        let a = unsafe { sysctl(&mut 1, 20 as u32, &mut data as *mut _ as *mut libc::c_void, &mut len, ptr::null_mut(), 0) };

        println!("{}", a);

        unimplemented!()

    }

    fn os_type() -> String {
        OS_TYPE.to_string()
    }

    fn cpu_stats<'a>() -> ResourceResult<CpuInfo> {

        Ok(CpuInfo {
            num_of_processors: num_cpus::get()
        })
    }

    fn computer_name() -> Result<String, ()> {

        let mut buf = Vec::<u8>::with_capacity(0x100);
        unsafe {
            let capacity = buf.capacity();
            buf.set_len(capacity);
        }
        let err = unsafe {
            gethostname(buf.as_mut_ptr() as *mut libc::c_char,
                        buf.len() as libc::size_t)
        } as isize;
        match err {
            0 => {
                let mut i = 0;
                while i < buf.len() {
                    if buf[i] == 0 {
                        buf.resize(i, 0);
                        break;
                    }
                    i += 1;
                }
                Ok(String::from_utf8_lossy(&buf).into_owned())
            }
            _ => Err(()),
        }

    }
}

extern "C" {
    fn gethostname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int;

    //fn sysctl(name : *mut libc::c_int, namelen : libc::c_uint, oldp : *mut libc::c_void, oldlenp : *mut libc::size_t, newp : *mut libc::c_void, newlen : libc::size_t) -> libc::c_int;
}