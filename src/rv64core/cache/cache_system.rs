

use crate::rv64core::{bus::Bus, traptype::RVmutex};

use super::{cpu_dcache::CpuDcache, cpu_icache::CpuIcache};

pub struct CacheSystem {
    pub icache: CpuIcache,
    pub dcache: CpuDcache,
    pub bus: RVmutex<Bus>,
}

impl CacheSystem {
    pub fn new(bus: RVmutex<Bus>) -> Self {
        let icache = CpuIcache::new(bus.clone());
        let dcache = CpuDcache::new(bus.clone());
        CacheSystem {
            icache,
            dcache,
            bus,
        }
    }
    pub fn show_perf(&self) {
        self.icache.show_perf();
        self.dcache.show_perf();
    }

    pub fn clear(&mut self) {
        self.icache.clear_inst();
        self.dcache.clear();
    }
}