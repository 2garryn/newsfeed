use std::io;
use std::io::Read;
use std::io::Write;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct FeedId(pub u128);

pub trait StorageWork<T> {
    fn read_from_store(reader: &mut Read) -> io::Result<T>;
    fn write_to_store(&self, writer: &mut Write) -> io::Result<()>;
}

impl StorageWork<FeedId> for FeedId {
    fn read_from_store(reader: &mut Read) -> io::Result<FeedId> {
        let mut id_b: [u8; 16] = [0; 16];
        reader.read_exact(&mut id_b)?;
        Ok(FeedId(u128::from_be_bytes(id_b)))
    }

    fn write_to_store(&self, writer: &mut Write) -> io::Result<()> {
        writer.write(&self.0.to_be_bytes())?;
        Ok(())
    }
}
