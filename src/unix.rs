use std::io::Error;
use super::{ ResourceError, Platform, DiskInfo, PlatformStats };

impl PlatformStats for Platform {
    fn disk_stats(driver: &str) -> Result<DiskInfo, Error> {
        unimplemented!()
    }

    fn memory_stats() -> Result<MemoryInfo, ResourceError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod unix_tests {

    use super::{Platform, PlatformStats};

    #[test]
    fn disk() {
        unimplemented!();
    }
}