#![no_std]

mod irq;

pub use irq::Irq;

use mmio::Mmio;

pub const MMIO_ADDRESS: usize = 0x5000;
const THRESHOLD_REGISTER: usize = 0;
const PENDING_IRQ_REGISTER: usize = 1;
const IRQ_INFO_REGISTER: usize = 1;

const IRQS_OFFSET: usize = 0x10;

#[derive(Debug, Clone, Copy)]
pub struct Plic {
    pub mmio: Mmio,
}

impl Plic {
    pub fn new(address: usize) -> Self {
        Self {
            mmio: Mmio::new(address),
        }
    }

    pub unsafe fn threshold(&self) -> u8 {
        self.mmio.read_u8(THRESHOLD_REGISTER)
    }

    pub unsafe fn pending_irq(&self) -> Option<u8> {
        let irq = self.mmio.read_u8(PENDING_IRQ_REGISTER);

        if irq == 0 {
            None
        } else {
            Some(irq)
        }
    }

    pub unsafe fn irq(&self, irq_idx: u8) -> Option<Irq> {
        let irq_bits = self.mmio.read_u16(irq_idx as usize + IRQS_OFFSET);

        if irq_bits == 0 {
            return None;
        }

        let priority = irq_bits as u8;
        let is_enabled = ((irq_bits >> 8) & 1) != 0;
        let is_pending = ((irq_bits >> 9) & 1) != 0;

        Some(Irq {
            priority,
            is_enabled,
            is_pending,
        })
    }

    pub unsafe fn set_threshold(&mut self, value: u8) {
        self.mmio.write_u8(value, THRESHOLD_REGISTER);
    }

    unsafe fn set_value(&mut self, irq_idx: u8, value_type: IrqValueType, value: u8) {
        let irq_idx = irq_idx as u32;
        let type_bits = (value_type as u8 as u32) << 8;
        let value_bits = (value as u32) << 16;
        let value = value_bits | type_bits | irq_idx;

        self.mmio.write_u32(value, IRQ_INFO_REGISTER);
    }

    pub unsafe fn set_priority(&mut self, irq_idx: u8, priority: u8) {
        self.set_value(irq_idx, IrqValueType::Priority, priority);
    }

    pub unsafe fn set_enabled(&mut self, irq_idx: u8, enabled: bool) {
        self.set_value(
            irq_idx,
            IrqValueType::IsEnabled,
            if enabled { 1 } else { 0 },
        );
    }

    pub unsafe fn claim(&mut self, irq_idx: u8) {
        self.set_value(irq_idx, IrqValueType::Claim, 1);
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum IrqValueType {
    Priority = 0,
    IsEnabled = 1,
    Claim = 2,
}

impl Default for Plic {
    fn default() -> Self {
        Self {
            mmio: Mmio::new(MMIO_ADDRESS),
        }
    }
}
