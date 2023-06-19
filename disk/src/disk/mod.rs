mod floppy_disk;
mod hdd_disk;

pub use floppy_disk::FloppyDisk;
pub use hdd_disk::HddDisk;

use crate::partitions::{Partition, Partitionable};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiskError {
    InvalidAddress,
    InvalidSize,
    DiskIsMissing,
    Unknown,
}

pub trait Disk {
    fn write(&mut self, address: usize, src: &[u8]) -> Result<usize, DiskError>;

    fn read(&mut self, address: usize, dst: &mut [u8]) -> Result<usize, DiskError>;

    fn size(&self) -> usize;
}

impl<T> Partitionable for T
where
    T: Disk,
{
    #[inline]
    fn start(&self) -> usize {
        0
    }

    #[inline]
    fn end(&self) -> usize {
        self.size()
    }

    #[inline]
    fn as_partition(&self) -> Partition {
        Partition::new(self.start(), self.end())
    }
}
