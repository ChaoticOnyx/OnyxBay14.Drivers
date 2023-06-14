#[derive(Debug, Clone)]
pub enum HddOp {
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

impl HddOp {
    pub fn id(&self) -> u32 {
        match self {
            HddOp::BulkRead { .. } => 0x0,
            HddOp::BulkWrite { .. } => 0x1,
        }
    }
}
