use core::fmt::Debug;

use mmio::Mmio;

use crate::{PciBus, UUID_LENGTH};

#[derive(Clone)]
pub struct PciDevice {
    pub device_id: u16,
    pub vendor_id: u16,
    pub irq_pin: u8,
    pub uuid: [u8; UUID_LENGTH],
    pub mmio: Mmio,
}

impl Debug for PciDevice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Stupid linter
        #[allow(clippy::let_unit_value)]
        let mmio = self.mmio.fmt(f)?;
        let uuid = &self.uuid;

        let a = u32::from_be_bytes([uuid[0], uuid[1], uuid[2], uuid[3]]);
        let b = u16::from_be_bytes([uuid[4], uuid[5]]);
        let c = u16::from_be_bytes([uuid[6], uuid[7]]);
        let d = u16::from_be_bytes([uuid[8], uuid[9]]);
        let e = u64::from_be_bytes([
            0, 0, uuid[10], uuid[11], uuid[12], uuid[13], uuid[14], uuid[15],
        ]);

        f.debug_struct("PciDevice")
            .field("vendor_id", &format_args!("{:#016X}", &self.vendor_id))
            .field("device_id", &format_args!("{:#016X}", self.device_id))
            .field("irq_pin", &format_args!("{}", self.irq_pin))
            .field(
                "uuid",
                &format_args!("{:#X}-{:#X}-{:#X}-{:#X}-{:#X}", a, b, c, d, e),
            )
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
