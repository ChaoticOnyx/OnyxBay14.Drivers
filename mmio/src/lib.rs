#![no_std]
#![no_main]
#![allow(clippy::missing_safety_doc)]

use core::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Mmio {
    address: usize,
}

impl Mmio {
    pub const fn new(address: usize) -> Self {
        Self { address }
    }

    pub fn address(&self) -> usize {
        self.address
    }

    gen_write_fn!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);

    gen_read_fn!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);
}

impl Debug for Mmio {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Mmio")
            .field("address", &format_args!("{:#016X}", self.address))
            .finish()
    }
}

#[macro_export]
macro_rules! gen_write_fn {
    ( $($t:ty),+ ) => {
        paste::item! {
            $( pub unsafe fn [< write_$t >](&self, value: $t, offset: usize) {
                let ptr = (self.address + offset) as *mut $t;

                ptr.write_volatile(value);
            } )+
        }
    };
}

#[macro_export]
macro_rules! gen_read_fn {
    ( $($t:ty),+ ) => {
        paste::item! {
            $( pub unsafe fn [< read_$t >](&self, offset: usize) -> $t {
                let ptr = (self.address + offset) as *const $t;

                ptr.read_volatile()
            } )+
        }
    };
}
