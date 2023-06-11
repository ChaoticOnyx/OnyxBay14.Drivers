use core::fmt::Debug;

use mmio::Mmio;

use crate::PciBus;

#[derive(Clone)]
pub struct PciDevice {
    pub device_id: u16,
    pub vendor_id: u16,
    pub irq_pin: u8,
    pub mmio: Mmio,
}

impl Debug for PciDevice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Stupid linter
        #[allow(clippy::let_unit_value)]
        let mmio = self.mmio.fmt(f)?;

        f.debug_struct("PciDevice")
            .field("vendor_id", &format_args!("{:#016X}", &self.vendor_id))
            .field("device_id", &format_args!("{:#016X}", self.device_id))
            .field("irq_pin", &format_args!("{}", self.irq_pin))
            .field("mmio", &mmio)
            .finish()
    }
}

pub struct PciDeviceIterator<'b> {
    pub(crate) next_id: u8,
    pub(crate) bus: &'b PciBus,
}

impl<'b> Iterator for PciDeviceIterator<'b> {
    type Item = PciDevice;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = unsafe { self.bus.device(self.next_id) };
        self.next_id += 1;

        ret
    }
}
