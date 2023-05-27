use core::fmt::Debug;

use drivers_mmio::Mmio;

#[derive(Clone)]
pub struct PciDevice {
    pub vendor_id: u16,
    pub device_id: u16,
    pub irq_line: u8,
    pub irq_pin: u8,
    pub status: u16,
    pub command: u16,
    pub mmio: Mmio,
}

impl Debug for PciDevice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mmio = self.mmio.fmt(f)?;

        f.debug_struct("PciDevice")
            .field("vendor_id", &format_args!("{:#016X}", &self.vendor_id))
            .field("device_id", &format_args!("{:#016X}", self.device_id))
            .field("irq_line", &format_args!("{:#08X}", self.irq_line))
            .field("irq_pin", &format_args!("{:#08X}", self.irq_pin))
            .field("status", &format_args!("{:#016X}", self.status))
            .field("command", &format_args!("{:#016X}", self.command))
            .field("mmio", &mmio)
            .finish()
    }
}
