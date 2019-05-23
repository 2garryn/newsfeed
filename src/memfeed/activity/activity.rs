use std::io;
use std::io::Read;
use std::io::Result;
use std::io::Write;

use super::types::*;
use crate::memfeed::common_types::*;

#[derive(Debug, Copy, Clone)]
pub struct Activity {
    pub id: ActivityId,
    pub onwer_feed: FeedId,
    pub published: Published,
    pub spec: FilterSpec,
}



impl Activity {
    pub fn read_from_store(reader: &mut Read) -> io::Result<Activity> {
        Ok(Activity {
            id: ActivityId::read_from_store(reader)?,
            onwer_feed: FeedId::read_from_store(reader)?,
            published: Published::read_from_store(reader)?,
            spec: FilterSpec::read_from_store(reader)?,
        })
    }

    pub fn write_to_store(&self, writer: &mut Write) -> Result<()> {
        self.id.write_to_store(writer)?;
        self.onwer_feed.write_to_store(writer)?;
        self.published.write_to_store(writer)?;
        self.spec.write_to_store(writer)?;
        Ok(())
    }

    pub fn size() -> u64 {
        16 + 8 + 2 + 16
    }
}
