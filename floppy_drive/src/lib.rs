#![no_std]

mod floppy_drive_argument;
mod floppy_drive_error;
mod floppy_drive_op;

pub use floppy_drive_argument::FloppyDriveArgument;
pub use floppy_drive_error::FloppyDriveError;
pub use floppy_drive_op::FloppyDriveOp;
use pci::PciDevice;

pub const DEVICE_ID: u16 = 0x6D;
pub const MAX_READ_WRITE_SIZE: usize = 65536;
const GET_SIZE: usize = 0x1;
const CALL_OP: usize = 0x0;
const EJECT_DISK: usize = 0x1;
const OP_RESULT: usize = 0x0;

pub struct FloppyDrive {
    pub device: PciDevice,
}

impl FloppyDrive {
    pub unsafe fn arg(&self, arg: FloppyDriveArgument) -> f64 {
        self.device.mmio.read_f64(arg.offset())
    }

    pub unsafe fn set_arg(&mut self, arg: FloppyDriveArgument, value: f64) {
        self.device.mmio.write_f64(value, arg.offset());
    }

    pub unsafe fn size(&self) -> u32 {
        self.device.mmio.read_u32(GET_SIZE)
    }

    pub unsafe fn eject_disk(&mut self) {
        self.device.mmio.write_u32(1, EJECT_DISK);
    }

    pub unsafe fn call_op(&mut self, op: FloppyDriveOp) -> Result<f64, FloppyDriveError> {
        match op {
            FloppyDriveOp::BulkRead {
                address,
                size,
                dst_address,
            }
            | FloppyDriveOp::BulkWrite {
                address,
                size,
                src_address: dst_address,
            } => {
                self.set_arg(FloppyDriveArgument::Arg0, address as f64);
                self.set_arg(FloppyDriveArgument::Arg1, size as f64);
                self.set_arg(FloppyDriveArgument::Arg2, dst_address as f64);
            }
        }

        self.device.mmio.write_u32(op.id(), CALL_OP);
        let ret = self.device.mmio.read_f64(OP_RESULT);

        if (ret as i64) < 0 {
            Err(FloppyDriveError::from(ret as i64))
        } else {
            Ok(ret)
        }
    }
}

impl From<PciDevice> for FloppyDrive {
    fn from(device: PciDevice) -> Self {
        Self { device }
    }
}
