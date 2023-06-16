#![no_std]

use mmio::Mmio;

pub const MMIO_ADDRESS: usize = 0x1FF80000;

pub struct Flash {
    mmio: Mmio,
    size: u32,
}

impl Flash {
    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn mmio(&self) -> Mmio {
        Mmio::new(self.mmio.address + core::mem::size_of::<u32>())
    }
}

impl Default for Flash {
    fn default() -> Self {
        let mmio = Mmio::new(MMIO_ADDRESS);
        let size = unsafe { mmio.read_u32(0x0) };

        Self { mmio, size }
    }
}
