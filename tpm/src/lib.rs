#![allow(clippy::missing_safety_doc)]
#![no_std]

use mmio::Mmio;

pub const MMIO_ADDRESS: usize = 0x10010000;
const RANDOM_BYTE: usize = 0x1;

#[derive(Debug, Clone)]
pub struct Tpm {
    pub mmio: Mmio,
}

impl Tpm {
    pub unsafe fn random_byte(&self) -> u8 {
        self.mmio.read_u8(RANDOM_BYTE)
    }
}

impl Default for Tpm {
    fn default() -> Self {
        Self {
            mmio: Mmio::new(MMIO_ADDRESS),
        }
    }
}
