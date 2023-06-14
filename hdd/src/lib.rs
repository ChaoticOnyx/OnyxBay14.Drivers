#![allow(clippy::missing_safety_doc)]
#![no_std]

mod hdd_argument;
mod hdd_error;
mod hdd_op;

pub use hdd_argument::HddArgument;
pub use hdd_error::HddError;
pub use hdd_op::HddOp;
use pci::PciDevice;

pub const DEVICE_ID: u16 = 0x6C;
pub const MAX_READ_WRITE_SIZE: usize = 65536;
const GET_SIZE: usize = 0x1;
const CALL_OP: usize = 0x0;
const OP_RESULT: usize = 0x0;

pub struct Hdd {
    pub device: PciDevice,
}

impl Hdd {
    pub unsafe fn arg(&self, arg: HddArgument) -> f64 {
        self.device.mmio.read_f64(arg.offset())
    }

    pub unsafe fn set_arg(&mut self, arg: HddArgument, value: f64) {
        self.device.mmio.write_f64(value, arg.offset());
    }

    pub unsafe fn size(&self) -> u32 {
        self.device.mmio.read_u32(GET_SIZE)
    }

    pub unsafe fn call_op(&mut self, op: HddOp) -> Result<f64, HddError> {
        match op {
            HddOp::BulkRead {
                address,
                size,
                dst_address,
            }
            | HddOp::BulkWrite {
                address,
                size,
                src_address: dst_address,
            } => {
                self.set_arg(HddArgument::Arg0, address as f64);
                self.set_arg(HddArgument::Arg1, size as f64);
                self.set_arg(HddArgument::Arg2, dst_address as f64);
            }
        }

        self.device.mmio.write_u32(op.id(), CALL_OP);
        let ret = self.device.mmio.read_f64(OP_RESULT);

        if (ret as i64) < 0 {
            Err(HddError::from(ret as i64))
        } else {
            Ok(ret)
        }
    }
}

impl From<PciDevice> for Hdd {
    fn from(device: PciDevice) -> Self {
        Self { device }
    }
}
