mod keyboard_key;

pub use keyboard_key::KeyboardKey;
use pci::PciDevice;

use crate::KeyState;

pub const DEVICE_ID: u16 = 0x69;
const EVENTS_REGISTER: usize = 0x0;
const LAST_CHANGED_KEY_REGISTER: usize = 0x1;

#[derive(Debug, Clone)]
pub struct Keyboard {
    pub device: PciDevice,
}

impl Keyboard {
    pub unsafe fn events_enabled(&self) -> bool {
        self.device.mmio.read_u8(EVENTS_REGISTER) != 0
    }

    pub unsafe fn set_events(&mut self, state: bool) {
        self.device
            .mmio
            .write_u8(if state { 1 } else { 0 }, EVENTS_REGISTER);
    }

    pub unsafe fn last_changed_key(&self) -> KeyboardKey {
        KeyboardKey::from(self.device.mmio.read_u8(LAST_CHANGED_KEY_REGISTER))
    }

    pub unsafe fn key_state(&self, key: KeyboardKey) -> KeyState {
        self.device.mmio.read_u8(key.offset()).into()
    }
}

impl From<PciDevice> for Keyboard {
    fn from(value: PciDevice) -> Self {
        Self { device: value }
    }
}
