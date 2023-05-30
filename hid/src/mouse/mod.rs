mod event_type;
mod mouse_key;

use pci::PciDevice;

pub use event_type::MouseEventType;
pub use mouse_key::MouseKey;

use crate::KeyState;

pub const DEVICE_ID: u16 = 0x68;
const EVENTS_REGISTER: usize = 0x0;
const LAST_EVENT_TYPE_REGISTER: usize = 0x1;
const LAST_CHANGED_KEY_REGISTER: usize = 0x2;
const POSITION_REGISTER: usize = 0x3;

#[derive(Debug, Clone)]
pub struct Mouse {
    pub device: PciDevice,
}

impl Mouse {
    pub unsafe fn events_enabled(&self) -> bool {
        self.device.mmio.read_u8(EVENTS_REGISTER) != 0
    }

    pub unsafe fn set_events(&mut self, state: bool) {
        self.device
            .mmio
            .write_u8(if state { 1 } else { 0 }, EVENTS_REGISTER);
    }

    pub unsafe fn position(&self) -> (u32, u32) {
        let position = self.device.mmio.read_u64(POSITION_REGISTER);
        let x = position as u32;
        let y = (position >> 32) as u32;

        (x, y)
    }

    pub unsafe fn last_event(&self) -> MouseEventType {
        self.device.mmio.read_u8(LAST_EVENT_TYPE_REGISTER).into()
    }

    pub unsafe fn last_changed_key(&self) -> MouseKey {
        self.device.mmio.read_u8(LAST_CHANGED_KEY_REGISTER).into()
    }

    pub unsafe fn key_state(&self, key: MouseKey) -> KeyState {
        self.device.mmio.read_u8(key.offset()).into()
    }
}

impl From<PciDevice> for Mouse {
    fn from(value: PciDevice) -> Self {
        Self { device: value }
    }
}
