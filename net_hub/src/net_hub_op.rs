use crate::PortMode;

#[derive(Debug, Clone)]
pub enum NetHubOp {
    SetInputPortEnabled {
        port: u8,
        state: bool,
    },
    IsInputPortConnected {
        port: u8,
    },
    IsOutputPortConnected {
        port: u8,
    },
    InvokeSimpleOutputPort {
        port: u8,
    },
    IsInputPortPending {
        port: u8,
    },
    ClaimSimpleInputPort {
        port: u8,
    },
    GetInputPortMode {
        port: u8,
    },
    GetOutputPortMode {
        port: u8,
    },
    SetInputPortMode {
        port: u8,
        mode: PortMode,
    },
    SetOutputPortMode {
        port: u8,
        mode: PortMode,
    },
    InvokeComplexOutputPort {
        port: u8,
        address: usize,
        size: usize,
    },
    GetPendingInputPort,
    ClaimComplexInputPort {
        port: u8,
        address: usize,
    },
}

impl NetHubOp {
    pub fn id(&self) -> u32 {
        match self {
            NetHubOp::SetInputPortEnabled { .. } => 0x0,
            NetHubOp::IsInputPortConnected { .. } => 0x1,
            NetHubOp::IsOutputPortConnected { .. } => 0x2,
            NetHubOp::InvokeSimpleOutputPort { .. } => 0x3,
            NetHubOp::IsInputPortPending { .. } => 0x4,
            NetHubOp::ClaimSimpleInputPort { .. } => 0x5,
            NetHubOp::GetInputPortMode { .. } => 0x6,
            NetHubOp::GetOutputPortMode { .. } => 0x7,
            NetHubOp::SetInputPortMode { .. } => 0x8,
            NetHubOp::SetOutputPortMode { .. } => 0x9,
            NetHubOp::InvokeComplexOutputPort { .. } => 0xA,
            NetHubOp::GetPendingInputPort => 0xB,
            NetHubOp::ClaimComplexInputPort { .. } => 0xC,
        }
    }
}
