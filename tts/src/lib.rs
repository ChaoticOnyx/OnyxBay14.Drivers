#![no_std]

use core::time::Duration;

use pci::PciDevice;

pub const DEVICE_ID: u16 = 0x64;

#[derive(Debug, Clone)]
pub struct Tts {
    pub device: PciDevice,
}

impl Tts {
    pub unsafe fn string_length(&self) -> u32 {
        self.device.mmio.read_u32(0x0)
    }

    pub unsafe fn speech_time(&self) -> Duration {
        Duration::from_millis(self.device.mmio.read_u64(0x4))
    }

    pub unsafe fn is_ready(&self) -> bool {
        self.device.mmio.read_u8(0x8) == 1
    }

    pub unsafe fn write_string(&mut self, text: &str) {
        for ch in text.as_bytes() {
            self.device.mmio.write_u8(*ch, 0x0)
        }
    }

    pub unsafe fn speech(&mut self) {
        self.device.mmio.write_u8(0, 0x4);
    }

    pub unsafe fn flush(&mut self) {
        self.device.mmio.write_u8(0, 0x8);
    }
}

impl From<PciDevice> for Tts {
    fn from(value: PciDevice) -> Self {
        Self { device: value }
    }
}
