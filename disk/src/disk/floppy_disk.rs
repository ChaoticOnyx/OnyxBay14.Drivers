use floppy_drive::{FloppyDrive, FloppyDriveError, FloppyDriveOp};

use super::{Disk, DiskError};

pub struct FloppyDisk {
    floppy: FloppyDrive,
}

impl FloppyDisk {
    #[inline]
    pub fn new(floppy: FloppyDrive) -> Self {
        Self { floppy }
    }

    #[inline]
    fn write_inner(&mut self, address: usize, src: &[u8]) -> Result<usize, DiskError> {
        unsafe {
            self.floppy.call_op(FloppyDriveOp::BulkWrite {
                address,
                size: src.len(),
                src_address: src.as_ptr() as usize,
            })
        }
        .map_err(|err| match err {
            FloppyDriveError::InvalidAddress => DiskError::InvalidAddress,
            FloppyDriveError::InvalidSize => DiskError::InvalidSize,
            FloppyDriveError::FloppyDriveIsEmpty => DiskError::DiskIsMissing,
            FloppyDriveError::Unknown => DiskError::Unknown,
        })?;

        Ok(src.len())
    }

    #[inline]
    fn read_inner(&mut self, address: usize, dst: &mut [u8]) -> Result<usize, DiskError> {
        unsafe {
            self.floppy.call_op(FloppyDriveOp::BulkRead {
                address,
                size: dst.len(),
                dst_address: dst.as_ptr() as usize,
            })
        }
        .map_err(|err| match err {
            FloppyDriveError::InvalidAddress => DiskError::InvalidAddress,
            FloppyDriveError::InvalidSize => DiskError::InvalidSize,
            FloppyDriveError::FloppyDriveIsEmpty => DiskError::DiskIsMissing,
            FloppyDriveError::Unknown => DiskError::Unknown,
        })?;

        Ok(dst.len())
    }
}

impl Disk for FloppyDisk {
    #[inline]
    fn write(&mut self, address: usize, src: &[u8]) -> Result<usize, DiskError> {
        let mut bytes_wrote = 0;

        for chunk in src.chunks(floppy_drive::MAX_READ_WRITE_SIZE) {
            bytes_wrote += self.write_inner(address, chunk)?;
        }

        Ok(bytes_wrote)
    }

    #[inline]
    fn read(&mut self, address: usize, dst: &mut [u8]) -> Result<usize, DiskError> {
        let mut bytes_read = 0;

        for chunk in dst.chunks_mut(floppy_drive::MAX_READ_WRITE_SIZE) {
            bytes_read += self.read_inner(address, chunk)?;
        }

        Ok(bytes_read)
    }

    #[inline]
    fn size(&self) -> usize {
        unsafe { self.floppy.size() as usize }
    }
}
