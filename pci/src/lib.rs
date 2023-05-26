#![no_std]
#![no_main]

mod pci_device;

use drivers_mmio::Mmio;
pub use pci_device::PciDevice;

pub const PCI_DEVICE_ID_REG: usize = 0x0;
pub const PCI_DEVICE_STATUS_REG: usize = 0x4;
pub const PCI_DEVICE_BAR0_REG: usize = 0x10;
pub const PCI_DEVICE_IRQ_REG: usize = 0x3C;

#[derive(Debug, Clone, Copy)]
pub struct Pci {
    mmio: Mmio,
}

impl Pci {
    pub fn new(address: usize) -> Self {
        Self {
            mmio: Mmio::new(address),
        }
    }

    pub unsafe fn device(&self, device_id: u8) -> Option<PciDevice> {
        let base_offset = ((device_id as u32) << 15) as usize;
        let func_id = 0;

        let func_offset = base_offset | func_id << 8;
        let ids_offset = func_offset | PCI_DEVICE_ID_REG;

        let ids = self.mmio.read_u32(ids_offset);

        if ids == 0xFFFFFFFF {
            return None;
        }

        let statuses_offset = func_offset | PCI_DEVICE_STATUS_REG;
        let statuses = self.mmio.read_u32(statuses_offset);

        let irqs_offset = func_offset | PCI_DEVICE_IRQ_REG;
        let irqs = self.mmio.read_u32(irqs_offset);

        let reg = PCI_DEVICE_BAR0_REG;
        let mmio_address = self.mmio.read_u32(func_offset | reg) as usize;
        let mmio = Mmio::new(mmio_address);

        Some(PciDevice {
            address: base_offset,
            vendor_id: (ids >> 16) as u16,
            device_id: ids as u16,
            irq_line: irqs as u8,
            irq_pin: (irqs >> 8) as u8,
            status: (statuses >> 16) as u16,
            command: statuses as u16,
            mmio,
        })
    }
}

impl Default for Pci {
    fn default() -> Self {
        Self {
            mmio: Mmio::new(0x30000000),
        }
    }
}
