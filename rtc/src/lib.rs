#![no_std]

use core::time::Duration;

use mmio::Mmio;

pub const MMIO_ADDRESS: usize = 0x1000;

const TIME_REGISTER: usize = 0x0;
const SCHEDULE_INTERRUPT_REGISTER: usize = 0x0;

#[derive(Debug, Clone, Copy)]
pub struct Rtc {
    pub mmio: Mmio,
}

impl Rtc {
    pub unsafe fn now(&self) -> Duration {
        Duration::from_millis(self.mmio.read_u64(TIME_REGISTER))
    }

    pub unsafe fn schedule_interrupt(&mut self, target: Duration) {
        self.mmio
            .write_u64(target.as_millis() as u64, SCHEDULE_INTERRUPT_REGISTER);
    }

    pub unsafe fn clear_interrupt(&mut self) {
        self.mmio.write_u64(0, SCHEDULE_INTERRUPT_REGISTER);
    }
}

impl Default for Rtc {
    fn default() -> Self {
        Self {
            mmio: Mmio::new(MMIO_ADDRESS),
        }
    }
}
