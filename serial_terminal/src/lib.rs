#![allow(clippy::missing_safety_doc)]
#![no_std]

use core::fmt::Write;

use pci::PciDevice;

pub const DEVICE_ID: u16 = 0x65;
pub const BUFFER_SIZE: usize = 1024;

#[derive(Debug, Clone)]
pub struct SerialTerminal {
    pub device: PciDevice,
}

impl SerialTerminal {
    pub unsafe fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.device.mmio.write_u8(*byte, 0x0);
        }
    }

    pub unsafe fn read_bytes(&self) -> Option<[u8; BUFFER_SIZE]> {
        let mut buffer = [b'\0'; BUFFER_SIZE];
        let mut length = 0;

        while length < BUFFER_SIZE {
            let ch = self.device.mmio.read_u8(0x0);

            if ch == 0 {
                if length == 0 {
                    return None;
                }

                return Some(buffer);
            }

            buffer[length] = ch;
            length += 1;
        }

        None
    }

    pub unsafe fn out_len(&self) -> usize {
        self.device.mmio.read_usize(0x4)
    }

    pub unsafe fn in_len(&self) -> usize {
        self.device.mmio.read_usize(0x8)
    }
}

impl From<PciDevice> for SerialTerminal {
    fn from(value: PciDevice) -> Self {
        Self { device: value }
    }
}

impl Write for SerialTerminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            for ch in s.as_bytes() {
                self.device.mmio.write_u8(*ch, 0x0)
            }

            Ok(())
        }
    }
}
