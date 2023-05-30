#![no_std]

use pci::PciDevice;

pub const DEVICE_ID: u16 = 0x67;

pub struct Screen {
    pub device: PciDevice,
}

impl Screen {
    pub unsafe fn width(&self) -> u32 {
        self.device.mmio.read_u32(0x0)
    }

    pub unsafe fn height(&self) -> u32 {
        self.device.mmio.read_u32(0x4)
    }

    pub unsafe fn is_connected(&self) -> bool {
        self.device.mmio.read_u8(0x8) == 1
    }

    pub unsafe fn connect(&mut self, gpu_address: usize) {
        self.device.mmio.write_usize(gpu_address, 0x0);
    }
}

impl From<PciDevice> for Screen {
    fn from(value: PciDevice) -> Self {
        Self { device: value }
    }
}
