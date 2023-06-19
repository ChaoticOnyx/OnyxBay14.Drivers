use uuid::Uuid;

use crate::disk::{Disk, DiskError};

use super::{Partition, Partitionable, MAX_PARTITIONS};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SGptPartitionType {
    Boot = 1,
    Unknown = 0xFFFFFFFF,
}

impl From<u32> for SGptPartitionType {
    fn from(value: u32) -> Self {
        if value >= Self::Boot as u32 && value < Self::Unknown as u32 {
            unsafe { core::mem::transmute(value) }
        } else {
            Self::Unknown
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SGptHeader {
    pub signature: u64,
    pub header_size: u32,
    pub uuid: Uuid,
    pub partitions: u32,
    pub partition_size: u32,
}

impl SGptHeader {
    #[inline]
    pub fn write(&self, disk: &mut impl Disk) -> Result<(), DiskError> {
        let mut offset = 0;

        offset += disk.write(offset, &self.signature.to_le_bytes())?;
        offset += disk.write(offset, &self.header_size.to_le_bytes())?;
        offset += disk.write(offset, self.uuid.as_bytes())?;
        offset += disk.write(offset, &self.partitions.to_le_bytes())?;
        disk.write(offset, &self.partition_size.to_le_bytes())?;

        Ok(())
    }

    #[inline]
    pub fn read(disk: &mut impl Disk) -> Result<Self, DiskError> {
        let mut offset = 0;

        let mut signature_bytes = [0u8; 8];
        offset += disk.read(offset, &mut signature_bytes)?;

        let mut header_size_bytes = [0u8; 4];
        offset += disk.read(offset, &mut header_size_bytes)?;

        let mut uuid_bytes = [0u8; 16];
        offset += disk.read(offset, &mut uuid_bytes)?;

        let mut partitions_bytes = [0u8; 4];
        offset += disk.read(offset, &mut partitions_bytes)?;

        let mut partitions_size_bytes = [0u8; 4];
        disk.read(offset, &mut partitions_size_bytes)?;

        let signature = u64::from_le_bytes(signature_bytes);
        let header_size = u32::from_le_bytes(header_size_bytes);
        let uuid = Uuid::from_bytes_le(uuid_bytes);
        let partitions = u32::from_le_bytes(partitions_bytes);
        let partition_size = u32::from_le_bytes(partitions_size_bytes);

        Ok(Self {
            signature,
            header_size,
            uuid,
            partitions,
            partition_size,
        })
    }
}

impl Default for SGptHeader {
    #[inline]
    fn default() -> Self {
        let signature = u64::from_be_bytes(*b"EFI PART");
        let header_size = core::mem::size_of::<SGptHeader>() as u32;
        let uuid = Uuid::new_v4();
        let partitions = MAX_PARTITIONS as u32;
        let partition_size = 128;

        Self {
            signature,
            header_size,
            uuid,
            partitions,
            partition_size,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SGptPartition {
    pub ty: SGptPartitionType,
    pub uuid: Uuid,
    pub start: u64,
    pub end: u64,
    pub name: [u8; 128],
}

impl SGptPartition {
    #[inline]
    pub fn read(index: u32, disk: &mut impl Disk) -> Result<Self, DiskError> {
        let mut offset = core::mem::size_of::<SGptHeader>() * index as usize;
        let mut type_bytes = [0u8; core::mem::size_of::<SGptPartitionType>()];
        offset += disk.read(offset, &mut type_bytes)?;

        let mut uuid_bytes = [0u8; 16];
        offset += disk.read(offset, &mut uuid_bytes)?;

        let mut start_bytes = [0u8; 8];
        offset += disk.read(offset, &mut start_bytes)?;

        let mut end_bytes = [0u8; 8];
        offset += disk.read(offset, &mut end_bytes)?;

        let mut name = [0u8; 128];
        disk.read(offset, &mut name)?;

        let ty = SGptPartitionType::from(u32::from_be_bytes(type_bytes));
        let uuid = Uuid::from_bytes_le(uuid_bytes);
        let start = u64::from_le_bytes(start_bytes);
        let end = u64::from_le_bytes(end_bytes);

        Ok(Self {
            ty,
            uuid,
            start,
            end,
            name,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.end == 0
    }

    #[inline]
    pub fn write(&self, index: u32, disk: &mut impl Disk) -> Result<(), DiskError> {
        let mut offset = core::mem::size_of::<SGptHeader>() * index as usize;

        offset += disk.write(offset, &(self.ty as u32).to_le_bytes())?;
        offset += disk.write(offset, self.uuid.as_bytes())?;
        offset += disk.write(offset, &self.start.to_le_bytes())?;
        offset += disk.write(offset, &self.end.to_le_bytes())?;
        disk.write(offset, &self.name)?;

        Ok(())
    }
}

impl Partitionable for SGptPartition {
    fn start(&self) -> usize {
        self.start as usize
    }

    fn end(&self) -> usize {
        self.end as usize
    }

    fn as_partition(&self) -> Partition {
        Partition::new(self.start(), self.end())
    }
}

#[derive(Debug)]
pub struct SgptPartitionIterator<'d, D>
where
    D: Disk,
{
    disk: &'d mut D,
    next: u32,
    max: u32,
}

impl<'d, D> Iterator for SgptPartitionIterator<'d, D>
where
    D: Disk,
{
    type Item = Result<SGptPartition, DiskError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == self.max {
            return None;
        }

        self.next += 1;

        Some(SGptPartition::read(self.next - 1, self.disk))
    }
}

/// sGPT - Simple GPT
#[derive(Debug, Clone)]
#[repr(C)]
pub struct SGpt {
    pub header: SGptHeader,
}

impl SGpt {
    #[inline]
    pub fn write(&self, disk: &mut impl Disk) -> Result<(), DiskError> {
        self.header.write(disk)?;

        Ok(())
    }

    #[inline]
    pub fn try_parse(disk: &mut impl Disk) -> Option<Result<Self, DiskError>> {
        let header = SGptHeader::read(disk);

        if let Err(err) = header {
            return Some(Err(err));
        }

        let header = header.unwrap();

        if header.signature != u64::from_le_bytes(*b"EFI PART") {
            return None;
        }

        if header.header_size != core::mem::size_of::<SGptHeader>() as u32 {
            return None;
        }

        Some(Ok(Self { header }))
    }

    #[inline]
    pub fn iter_partitions<'d, D>(&self, disk: &'d mut D) -> SgptPartitionIterator<'d, D>
    where
        D: Disk,
    {
        SgptPartitionIterator {
            disk,
            next: 0,
            max: self.header.partitions,
        }
    }
}

impl Default for SGpt {
    #[inline]
    fn default() -> Self {
        let header = SGptHeader::default();

        Self { header }
    }
}
