#![allow(clippy::missing_safety_doc)]
#![no_std]

mod pci_device;

use mmio::Mmio;
pub use pci_device::PciDevice;
use pci_device::PciDeviceIterator;

const MMIO_ADDRESS: usize = 0x0FF80000;
const INFO_REGISTER: usize = 0x0;
const ADDRESS_REGISTER: usize = 0x1;
const UUID_REGISTER_OFFSET: usize = 0x2;
pub const UUID_LENGTH: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct PciBus {
    pub mmio: Mmio,
}

impl PciBus {
    pub fn new(address: usize) -> Self {
        Self {
            mmio: Mmio::new(address),
        }
    }

    pub unsafe fn device(&self, idx: u8) -> Option<PciDevice> {
        let info_bits = self.mmio.read_u64((idx as usize) << 8 | INFO_REGISTER);
        let device_address = self.mmio.read_usize((idx as usize) << 8 | ADDRESS_REGISTER);

        let vendor_id = (info_bits & 0xFFFF) as u16;
        let device_id = ((info_bits >> 16) & 0xFFFF) as u16;
        let irq_pin = ((info_bits >> 32) & 0xFF) as u8;

        if vendor_id == 0xFFFF || device_id == 0xFFFF {
            return None;
        }

        let mut uuid = [0; UUID_LENGTH];

        for (i, b) in uuid.iter_mut().enumerate().take(UUID_LENGTH) {
            *b = self.mmio.read_u8(UUID_REGISTER_OFFSET + i);
        }

        let mmio = Mmio::new(device_address);

        Some(PciDevice {
            vendor_id,
            device_id,
            irq_pin,
            uuid,
            mmio,
        })
    }

    pub fn iter(&self) -> PciDeviceIterator<'_> {
        PciDeviceIterator {
            next_id: 0,
            bus: self,
        }
    }

    pub fn find_by_id(&self, id: u16) -> Option<PciDevice> {
        self.iter().find(|d| d.device_id == id)
    }
}

impl Default for PciBus {
    fn default() -> Self {
        Self {
            mmio: Mmio::new(MMIO_ADDRESS),
        }
    }
}
