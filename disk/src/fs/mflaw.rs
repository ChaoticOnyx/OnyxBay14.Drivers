use crate::{
    disk::{Disk, DiskError},
    fs::BLOCK_SIZE,
    partitions::Partitionable,
};

pub const MAGIC: u8 = 0xF1;

/// MFLAW - Most FLAWless file system
#[derive(Debug)]
pub struct MFlaw {
    superblock: MFlawSuperblock,
}

impl MFlaw {
    #[inline]
    pub fn new(partition: &impl Partitionable) -> Self {
        let superblock = MFlawSuperblock::new(partition);

        Self { superblock }
    }

    #[inline]
    pub fn try_parse(
        disk: &mut impl Disk,
        partition: &impl Partitionable,
    ) -> Option<Result<Self, DiskError>> {
        let superblock = match MFlawSuperblock::try_parse(disk, partition) {
            None => return None,
            Some(Err(err)) => return Some(Err(err)),
            Some(Ok(superblock)) => superblock,
        };

        Some(Ok(Self { superblock }))
    }

    #[inline]
    pub fn write(
        &self,
        disk: &mut impl Disk,
        partition: &impl Partitionable,
    ) -> Result<(), DiskError> {
        self.superblock.write(disk, partition)?;

        Ok(())
    }

    #[inline]
    pub fn superblock(&self) -> &MFlawSuperblock {
        &self.superblock
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct MFlawSuperblock {
    pub magic: u8,
    pub inodes: usize,
    pub zones: usize,
    pub inode_bitmap_blocks: usize,
    pub zone_bitmap_blocks: usize,
    pub data_blocks: usize,
    pub inodes_size: usize,
    pub zones_size: usize,
    pub data_size: usize,
    pub total_size: usize,
}

impl MFlawSuperblock {
    #[inline]
    pub fn new(partition: &impl Partitionable) -> Self {
        let total_size = partition.end() - partition.start();
        let data_size = total_size - core::mem::size_of::<Self>();
        let data_blocks_count = data_size / BLOCK_SIZE;

        // inodes
        let inodes = data_blocks_count - 1;
        let inodes_size = inodes / 8;
        let inode_bitmap_blocks = usize::max(1, inodes / 8 / BLOCK_SIZE);

        // zones
        let zones = data_blocks_count - inode_bitmap_blocks + 1;
        let zones_size = zones / 8;
        let zone_bitmap_blocks = usize::max(1, zones / 8 / BLOCK_SIZE);

        // data
        let data_blocks = data_blocks_count - (inode_bitmap_blocks + zone_bitmap_blocks);
        let data_size = data_size - ((inode_bitmap_blocks + zone_bitmap_blocks) * BLOCK_SIZE);

        Self {
            magic: MAGIC,
            inodes,
            zones,
            inode_bitmap_blocks,
            zone_bitmap_blocks,
            data_blocks,
            inodes_size,
            zones_size,
            data_size,
            total_size,
        }
    }

    #[inline]
    fn try_parse_inner(
        disk: &mut impl Disk,
        partition: &impl Partitionable,
    ) -> Result<Self, DiskError> {
        let mut bytes = [0u8; core::mem::size_of::<Self>()];
        disk.read(partition.start(), &mut bytes)?;

        unsafe {
            Ok(core::mem::transmute::<
                [u8; core::mem::size_of::<Self>()],
                MFlawSuperblock,
            >(bytes))
        }
    }

    #[inline]
    pub fn try_parse(
        disk: &mut impl Disk,
        partition: &impl Partitionable,
    ) -> Option<Result<Self, DiskError>> {
        let superblock = match Self::try_parse_inner(disk, partition) {
            Ok(header) => header,
            Err(err) => return Some(Err(err)),
        };

        if superblock.magic != MAGIC {
            return None;
        }

        Some(Ok(superblock))
    }

    #[inline]
    pub fn write(
        &self,
        disk: &mut impl Disk,
        partition: &impl Partitionable,
    ) -> Result<(), DiskError> {
        let bytes =
            unsafe { core::mem::transmute_copy::<Self, [u8; core::mem::size_of::<Self>()]>(self) };

        disk.write(partition.start(), &bytes)?;

        Ok(())
    }

    // #[inline]
    // pub fn inode_bitmap_block(&self, index: usize) -> Option<MFlawBlock> {
    //     if index >= self.inode_bitmap_blocks {
    //         return None;
    //     }

    //     let header_end = self.start + core::mem::size_of::<Self>();
    //     let block_offset = index * BLOCK_SIZE;

    //     Some(MFlawBlock::new(index, header_end + block_offset))
    // }

    // #[inline]
    // pub fn zone_bitmap_block(&self, index: usize) -> Option<MFlawBlock> {
    //     if index >= self.inode_bitmap_blocks {
    //         return None;
    //     }

    //     let header_end = self.start + core::mem::size_of::<Self>();
    //     let block_offset = index * BLOCK_SIZE;
    //     let inode_blocks_offset = self.inode_bitmap_blocks * BLOCK_SIZE + block_offset;

    //     Some(MFlawBlock::new(
    //         index,
    //         header_end + block_offset + inode_blocks_offset,
    //     ))
    // }
}

#[derive(Debug, Clone, Copy)]
pub struct MFlawBlock {
    index: usize,
    address: usize,
}

impl MFlawBlock {
    #[inline]
    pub(crate) fn new(index: usize, address: usize) -> Self {
        Self { index, address }
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    #[inline]
    pub fn address(&self) -> usize {
        self.address
    }
}
