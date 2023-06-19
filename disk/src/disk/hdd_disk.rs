use hdd::{Hdd, HddOp};

use super::{Disk, DiskError};

#[derive(Debug, Clone)]
pub struct HddDisk {
    hdd: Hdd,
}

impl HddDisk {
    #[inline]
    pub fn new(hdd: Hdd) -> Self {
        Self { hdd }
    }

    #[inline]
    fn write_inner(&mut self, address: usize, src: &[u8]) -> Result<usize, DiskError> {
        unsafe {
            self.hdd
                .call_op(HddOp::BulkWrite {
                    address,
                    size: src.len(),
                    src_address: src.as_ptr() as usize,
                })
                .map_err(|err| match err {
                    hdd::HddError::InvalidAddress => DiskError::InvalidAddress,
                    hdd::HddError::InvalidSize => DiskError::InvalidSize,
                    hdd::HddError::Unknown => DiskError::Unknown,
                })?;
        }

        Ok(src.len())
    }

    #[inline]
    fn read_inner(&mut self, address: usize, dst: &[u8]) -> Result<usize, DiskError> {
        unsafe {
            self.hdd.call_op(HddOp::BulkRead {
                address,
                size: dst.len(),
                dst_address: dst.as_ptr() as usize,
            })
        }
        .map_err(|err| match err {
            hdd::HddError::InvalidAddress => DiskError::InvalidAddress,
            hdd::HddError::InvalidSize => DiskError::InvalidSize,
            hdd::HddError::Unknown => DiskError::Unknown,
        })?;

        Ok(dst.len())
    }
}

impl Disk for HddDisk {
    #[inline]
    fn write(&mut self, address: usize, src: &[u8]) -> Result<usize, DiskError> {
        let mut bytes_wrote = 0;

        for chunk in src.chunks(hdd::MAX_READ_WRITE_SIZE) {
            bytes_wrote += self.write_inner(address, chunk)?;
        }

        Ok(bytes_wrote)
    }

    #[inline]
    fn read(&mut self, address: usize, dst: &mut [u8]) -> Result<usize, DiskError> {
        let mut bytes_read = 0;

        for chunk in dst.chunks_mut(hdd::MAX_READ_WRITE_SIZE) {
            bytes_read += self.read_inner(address, chunk)?;
        }

        Ok(bytes_read)
    }

    #[inline]
    fn size(&self) -> usize {
        unsafe { self.hdd.size() as usize }
    }
}
