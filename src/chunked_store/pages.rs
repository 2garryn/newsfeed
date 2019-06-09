use std::io::Result;
use crate::activity::{FeedId, Activity, ActivityId};
use std::path::{Path, PathBuf};


pub struct Pages {
}


impl Pages {
    pub fn new(feed_id: FeedId, storage_path: &Path) -> Result<Pages> {
        Ok(Pages{})
    }
}