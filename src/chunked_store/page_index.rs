
use crate::activity::{FeedId, Activity, ActivityId};
use std::path::{Path, PathBuf};
use std::io::{Read, Result};

pub struct PageIndex {
}

impl PageIndex {
    pub fn new(feed_id: FeedId, storage_path: &Path) -> Result<PageIndex> {
        Ok(PageIndex{})
    }
}