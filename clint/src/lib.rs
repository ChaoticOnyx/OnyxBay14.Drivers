#![allow(clippy::missing_safety_doc)]
#![no_std]

use mmio::Mmio;

pub const MMIO_ADDRESS: usize = 0x3000;

const GET_TIMER_REGISTER: usize = 0x0;
const GET_TIME_CMP_REGISTER: usize = 0x1;
const SET_TIME_CMP_REGISTER: usize = 0x0;
const MSOFTWARE_INTERRUPT: usize = 0x1;

pub struct Clint {
    mmio: Mmio,
}

impl Clint {
    pub unsafe fn timer(&self) -> usize {
        self.mmio.read_usize(GET_TIMER_REGISTER)
    }

    pub unsafe fn time_cmp(&self) -> usize {
        self.mmio.read_usize(GET_TIME_CMP_REGISTER)
    }

    pub unsafe fn set_time_cmp(&mut self, value: usize) {
        self.mmio.write_usize(value, SET_TIME_CMP_REGISTER);
    }

    pub unsafe fn set_msoftware_interrupt(&mut self, state: bool) {
        self.mmio
            .write_u8(if state { 1 } else { 0 }, MSOFTWARE_INTERRUPT);
    }
}

impl Default for Clint {
    fn default() -> Self {
        Self {
            mmio: Mmio::new(MMIO_ADDRESS),
        }
    }
}
