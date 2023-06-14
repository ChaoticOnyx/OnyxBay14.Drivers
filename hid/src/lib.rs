#![allow(clippy::missing_safety_doc)]
#![no_std]

mod key_state;
pub mod keyboard;
pub mod mouse;

pub use key_state::KeyState;
