#![no_std]

use mmio::Mmio;

pub const MMIO_ADDRESS: usize = 0x2000;
const HAS_BATTERY_REGISTER: usize = 0x0;
const BATTERY_CAPACITY_REGISTER: usize = 0x1;
const BATTERY_CHARGE_REGISTER: usize = 0x2;
const SHUTDOWN_REGISTER: usize = 0x0;
const REBOOT_REGISTER: usize = 0x1;

#[derive(Debug, Clone, Copy)]
pub struct Apm {
    pub mmio: Mmio,
}

impl Apm {
    pub unsafe fn has_battery(&self) -> bool {
        self.mmio.read_u8(HAS_BATTERY_REGISTER) == 1
    }

    pub unsafe fn battery_capacity(&self) -> u32 {
        self.mmio.read_u32(BATTERY_CAPACITY_REGISTER)
    }

    pub unsafe fn battery_charge(&self) -> u32 {
        self.mmio.read_u32(BATTERY_CHARGE_REGISTER)
    }

    pub unsafe fn shutdown(&mut self) {
        self.mmio.write_u8(0x1, SHUTDOWN_REGISTER);
    }

    pub unsafe fn reboot(&mut self) {
        self.mmio.write_u8(0x1, REBOOT_REGISTER);
    }
}

impl Default for Apm {
    fn default() -> Self {
        Self {
            mmio: Mmio::new(MMIO_ADDRESS),
        }
    }
}
