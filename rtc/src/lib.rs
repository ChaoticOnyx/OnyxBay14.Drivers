#![no_std]

use core::time::Duration;

use drivers_mmio::Mmio;

pub const MMIO_ADDRESS: usize = 0x101000;

#[derive(Debug, Clone, Copy)]
pub struct Rtc {
    pub mmio: Mmio,
}

impl Rtc {
    pub unsafe fn now(&self) -> Duration {
        Duration::from_millis(self.mmio.read_u64(0))
    }
}

impl Default for Rtc {
    fn default() -> Self {
        Self {
            mmio: Mmio::new(MMIO_ADDRESS),
        }
    }
}
