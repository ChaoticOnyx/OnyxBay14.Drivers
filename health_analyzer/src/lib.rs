#![no_std]

use pic::PciDevice;

pub const DEVICE_ID: u16 = 0x6A;

const DISEASE_REGISTER: usize = 0x0;
const DAMAGE_OFFSET: usize = 0x10;

pub struct HealthAnalyzer {
    pub device: PciDevice,
}

impl HealthAnalyzer {
    pub fn damage(&self, ty: DamageType) -> f64 {
        unsafe { self.device.mmio.read_f64(ty as usize + DAMAGE_OFFSET) }
    }

    pub fn has_disease(&self) -> bool {
        unsafe { self.device.mmio.read_u8(DISEASE_REGISTER) != 0 }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DamageTypeIterator {
    next: u8,
}

impl Iterator for DamageTypeIterator {
    type Item = DamageType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next > DamageType::Slash as u8 {
            return None;
        }

        let ret = unsafe { Some(core::mem::transmute::<u8, DamageType>(self.next)) };
        self.next += 1;

        ret
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DamageType {
    Asphyxiation = 0,
    Bloodloss = 1,
    Blunt = 2,
    Cellular = 3,
    Caustic = 4,
    Cold = 5,
    Heat = 6,
    Piercing = 7,
    Poison = 8,
    Radiation = 9,
    Shock = 10,
    Slash = 11,
}

impl From<PciDevice> for HealthAnalyzer {
    fn from(device: PciDevice) -> Self {
        Self { device }
    }
}
