#[derive(Debug)]
pub struct DiskInfo {
    pub total: u64,
    pub free: u64
}

#[derive(Debug)]
pub struct MemoryInfo {
    pub total: u64,
    pub free: u64,
    pub avail: u64,
    pub buffers: u64,
    pub cached: u64,
    /// Total swap memory.
    pub swap_total: u64,
    pub swap_free: u64
}

#[derive(Debug)]
pub struct CpuInfo {
    pub num_of_processors: usize
}

pub struct Platform;

type ResourceResult<T> = Result<T, std::io::Error>;

trait PlatformStats {
    fn disk_stats<T>(driver : T) -> ResourceResult<DiskInfo> where T : Into<String>;
    fn memory_stats() -> ResourceResult<MemoryInfo>;
    fn os_release() -> ResourceResult<String>;
    fn os_type() -> String;
    fn cpu_stats() -> ResourceResult<CpuInfo>;
    fn computer_name() -> Result<String, ()>;
}

#[cfg(windows)]
#[path = "windows.rs"]
mod sys_info;

#[cfg(not(windows))]
#[path = "unix.rs"]
mod sys_info;

#[cfg(test)]
mod tests {

    use ::{ PlatformStats };
    use ::sys_info::{ OS_TYPE };

    #[cfg(windows)]
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
        let r = ::Platform::memory_stats();

        match r {
            Ok(m) => assert!(m.total > 0),
            Err(_) => assert!(false)
        };
    }

    #[test]
    fn os_release() {
        let r = ::Platform::os_release();

        match r {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        };
    }

    #[test]
    fn os_type() {
        let r = ::Platform::os_type();

        assert_eq!(r, OS_TYPE.to_string());
    }

    #[test]
    fn cpu() {
        let r = ::Platform::cpu_stats();

        match r {
            Ok(stat) => assert!(stat.num_of_processors > 0),
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn computer_name() {
        let r = ::Platform::computer_name();

        match r {
            Ok(name) => assert!(name.len() > 0),
            Err(_) => assert!(false)
        }
    }
}