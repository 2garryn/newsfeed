use std::io::{Read, Write, Result};
use std::string::ToString;
use crate::store::StoreCall;

#[derive(Debug, Copy, Clone)]
pub struct ActivityId(pub u128);

#[derive(Debug, Copy, Clone)]
pub struct Published(pub u64);

#[derive(Debug, Copy, Clone)]
pub struct Filter(pub u16);

#[derive(Debug, Copy, Clone)]
pub struct FeedId(pub u128);

#[derive(Debug, Copy, Clone)]
pub struct Actor(pub u128);




#[derive(Debug, Copy, Clone)]
pub struct Activity {
    pub id: ActivityId,
    pub onwer_feed: FeedId,
    pub actor: Actor,
    pub published: Published,
    pub filter: Filter,
}


impl StoreCall<Activity> for Activity {
    fn read_from_store(reader: &mut Read) -> Result<Activity> {
        Ok(Activity {
            id: ActivityId::read_from_store(reader)?,
            onwer_feed: FeedId::read_from_store(reader)?,
            actor: Actor::read_from_store(reader)?,
            published: Published::read_from_store(reader)?,
            filter: Filter::read_from_store(reader)?,
        })
    }
    fn write_to_store(&self, writer: &mut Write) -> Result<()> {
        self.id.write_to_store(writer)?;
        self.onwer_feed.write_to_store(writer)?;
        self.actor.write_to_store(writer)?;
        self.published.write_to_store(writer)?;
        self.filter.write_to_store(writer)?;
        Ok(())
    }
    fn byte_size() -> u8 {
        ActivityId::byte_size() +
        FeedId::byte_size() +
        Actor::byte_size() + 
        Published::byte_size() + 
        Filter::byte_size()
    }
}

impl StoreCall<ActivityId> for ActivityId {
    fn read_from_store(reader: &mut Read) -> Result<ActivityId> {
        let mut id_b: [u8; 16] = [0; 16];
        reader.read(&mut id_b)?;
        Ok(ActivityId(u128::from_be_bytes(id_b)))
    }
    fn write_to_store(&self, writer: &mut Write) -> Result<()> {
        writer.write(&self.0.to_be_bytes())?;
        Ok(())
    }
    fn byte_size() -> u8 {
        16
    }
}

impl StoreCall<FeedId> for FeedId {
    fn read_from_store(reader: &mut Read) -> Result<FeedId> {
        let mut id_b: [u8; 16] = [0; 16];
        reader.read(&mut id_b)?;
        Ok(FeedId(u128::from_be_bytes(id_b)))
    }
    fn write_to_store(&self, writer: &mut Write) -> Result<()> {
        writer.write(&self.0.to_be_bytes())?;
        Ok(())
    }
    fn byte_size() -> u8 {
        16
    }
}

impl StoreCall<Actor> for Actor {
    fn read_from_store(reader: &mut Read) -> Result<Actor> {
        let mut id_b: [u8; 16] = [0; 16];
        reader.read(&mut id_b)?;
        Ok(Actor(u128::from_be_bytes(id_b)))
    }
    fn write_to_store(&self, writer: &mut Write) -> Result<()> {
        writer.write(&self.0.to_be_bytes())?;
        Ok(())
    }
    fn byte_size() -> u8 {
        16
    }
}

impl StoreCall<Published> for Published {
    fn read_from_store(reader: &mut Read) -> Result<Published> {
        let mut published_b: [u8; 8] = [0; 8];
        reader.read(&mut published_b)?;
        Ok(Published(u64::from_be_bytes(published_b)))
    }
    fn write_to_store(&self, writer: &mut Write) -> Result<()> {
        writer.write(&self.0.to_be_bytes())?;
        Ok(())
    }
    fn byte_size() -> u8 {
        8
    }
}

impl StoreCall<Filter> for Filter {
    fn read_from_store(reader: &mut Read) -> Result<Filter> {
        let mut spec_b: [u8; 2] = [0; 2];
        reader.read(&mut spec_b)?;
        Ok(Filter(u16::from_be_bytes(spec_b)))
    }
    fn write_to_store(&self, writer: &mut Write) -> Result<()> {
        writer.write(&self.0.to_be_bytes())?;
        Ok(())
    }
    fn byte_size() -> u8 {
        2
    }
}

impl ActivityId {
    pub fn to_milliseconds(&self) -> u64 {
        (self.0 >> 80) as u64
    }
}




impl ToString for FeedId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}