#[derive(Debug, Clone)]
pub enum FloppyDriveOp {
    BulkRead {
        address: usize,
        size: usize,
        dst_address: usize,
    },
    BulkWrite {
        address: usize,
        size: usize,
        src_address: usize,
    },
}

impl FloppyDriveOp {
    pub fn id(&self) -> u32 {
        match self {
            FloppyDriveOp::BulkRead { .. } => 0x0,
            FloppyDriveOp::BulkWrite { .. } => 0x1,
        }
    }
}
