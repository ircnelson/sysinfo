#[derive(Debug)]
pub enum ResourceError<'a> {
    IO(std::io::Error),
    Generic(&'a str)
}

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
    pub num_of_processors: u32
}

pub struct Platform;

type ResourceResult<'a, T> = Result<T, ResourceError<'a>>;

trait PlatformStats {
    fn disk_stats<'a, T>(driver : T) -> ResourceResult<'a, DiskInfo> where T : Into<String>;
    fn memory_stats<'a>() -> ResourceResult<'a, MemoryInfo>;
    fn os_release<'a>() -> ResourceResult<'a, String>;
    fn os_type<'a>() -> &'a str;
    fn cpu_stats<'a>() -> ResourceResult<'a, CpuInfo>;
    fn computer_name<'a>() -> ResourceResult<'a, String>;
}

#[cfg(windows)]
#[path = "windows.rs"]
mod sys_info;

#[cfg(not(windows))]
#[path = "unix.rs"]
mod sys_info;