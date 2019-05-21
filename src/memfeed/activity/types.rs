use std::io;
use std::io::Read;
use std::io::Result;
use std::io::Write;

use crate::memfeed::common_types::StorageWork;

#[derive(Debug)]
pub struct ActivityId(u128);

#[derive(Debug)]
pub struct Published(u64);

#[derive(Debug)]
pub struct FilterSpec(u16);

impl StorageWork<ActivityId> for ActivityId {
    fn read_from_store(reader: &mut Read) -> io::Result<ActivityId> {
        let mut id_b: [u8; 16] = [0; 16];
        reader.read(&mut id_b)?;
        Ok(ActivityId(u128::from_be_bytes(id_b)))
    }

    fn write_to_store(&self, writer: &mut Write) -> io::Result<()> {
        writer.write(&self.0.to_be_bytes())?;
        Ok(())
    }
}

impl StorageWork<Published> for Published {
    fn read_from_store(reader: &mut Read) -> io::Result<Published> {
        let mut published_b: [u8; 8] = [0; 8];
        reader.read(&mut published_b)?;
        Ok(Published(u64::from_be_bytes(published_b)))
    }

    fn write_to_store(&self, writer: &mut Write) -> io::Result<()> {
        writer.write(&self.0.to_be_bytes())?;
        Ok(())
    }
}

impl StorageWork<FilterSpec> for FilterSpec {
    fn read_from_store(reader: &mut Read) -> io::Result<FilterSpec> {
        let mut spec_b: [u8; 2] = [0; 2];
        reader.read(&mut spec_b)?;
        Ok(FilterSpec(u16::from_be_bytes(spec_b)))
    }

    fn write_to_store(&self, writer: &mut Write) -> io::Result<()> {
        writer.write(&self.0.to_be_bytes())?;
        Ok(())
    }
}
