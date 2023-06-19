mod partition_type;
pub mod sgpt;

pub use partition_type::PartitionType;

pub const MAX_PARTITIONS: usize = 32;

pub trait Partitionable {
    fn start(&self) -> usize;

    fn end(&self) -> usize;

    fn as_partition(&self) -> Partition;
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Partition {
    pub start: usize,
    pub end: usize,
}

impl Partition {
    #[inline]
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl Partitionable for Partition {
    #[inline]
    fn start(&self) -> usize {
        self.start
    }

    #[inline]
    fn end(&self) -> usize {
        self.end
    }

    fn as_partition(&self) -> Partition {
        Partition {
            start: self.start(),
            end: self.end(),
        }
    }
}
