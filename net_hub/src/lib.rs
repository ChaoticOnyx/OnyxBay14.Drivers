#![allow(clippy::missing_safety_doc)]
#![no_std]

mod net_hub_argument;
mod net_hub_error;
mod net_hub_op;
mod port_mode;

pub use net_hub_argument::NetHubArgument;
pub use net_hub_error::NetHubError;
pub use net_hub_op::NetHubOp;
use pci::PciDevice;
pub use port_mode::PortMode;

pub const DEVICE_ID: u16 = 0x6B;

const CALL_OP: usize = 0x0;
const OP_RESULT: usize = 0x0;

#[derive(Debug, Clone)]
pub struct NetworkSwitch {
    pub device: PciDevice,
}

impl NetworkSwitch {
    pub unsafe fn arg(&self, arg: NetHubArgument) -> f64 {
        self.device.mmio.read_f64(arg.offset())
    }

    pub unsafe fn set_arg(&mut self, arg: NetHubArgument, value: f64) {
        self.device.mmio.write_f64(value, arg.offset());
    }

    pub unsafe fn call_op(&mut self, op: NetHubOp) -> Result<f64, NetHubError> {
        match op {
            NetHubOp::SetInputPortEnabled { port, state } => {
                self.set_arg(NetHubArgument::Arg0, port as f64);
                self.set_arg(NetHubArgument::Arg1, state as u8 as f64);
            }
            NetHubOp::IsInputPortConnected { port } => {
                self.set_arg(NetHubArgument::Arg0, port as f64);
            }
            NetHubOp::IsOutputPortConnected { port } => {
                self.set_arg(NetHubArgument::Arg0, port as f64);
            }
            NetHubOp::InvokeSimpleOutputPort { port } => {
                self.set_arg(NetHubArgument::Arg0, port as f64);
            }
            NetHubOp::IsInputPortPending { port } => {
                self.set_arg(NetHubArgument::Arg0, port as f64);
            }
            NetHubOp::ClaimSimpleInputPort { port } => {
                self.set_arg(NetHubArgument::Arg0, port as f64);
            }
            NetHubOp::GetInputPortMode { port } => {
                self.set_arg(NetHubArgument::Arg0, port as f64);
            }
            NetHubOp::GetOutputPortMode { port } => {
                self.set_arg(NetHubArgument::Arg0, port as f64);
            }
            NetHubOp::SetInputPortMode { port, mode } => {
                self.set_arg(NetHubArgument::Arg0, port as f64);
                self.set_arg(NetHubArgument::Arg1, mode as u8 as f64);
            }
            NetHubOp::SetOutputPortMode { port, mode } => {
                self.set_arg(NetHubArgument::Arg0, port as f64);
                self.set_arg(NetHubArgument::Arg1, mode as u8 as f64);
            }
            NetHubOp::InvokeComplexOutputPort {
                port,
                address,
                size,
            } => {
                self.set_arg(NetHubArgument::Arg0, port as f64);
                self.set_arg(NetHubArgument::Arg1, address as f64);
                self.set_arg(NetHubArgument::Arg2, size as f64);
            }
            NetHubOp::GetPendingInputPort => {}
            NetHubOp::ClaimComplexInputPort { port, address } => {
                self.set_arg(NetHubArgument::Arg0, port as f64);
                self.set_arg(NetHubArgument::Arg1, address as f64);
            }
        }

        self.device.mmio.write_u32(op.id(), CALL_OP);
        let ret = self.device.mmio.read_f64(OP_RESULT);

        if (ret as i64) < 0 {
            Err(NetHubError::from(ret as i64))
        } else {
            Ok(ret)
        }
    }
}

impl From<PciDevice> for NetworkSwitch {
    fn from(device: PciDevice) -> Self {
        Self { device }
    }
}
