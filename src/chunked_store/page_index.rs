
use crate::activity::{FeedId, Activity, ActivityId};

use std::io::{Read, Result};

pub struct PageIndex {
    first_id: ActivityId,
    n_acts: u32
}

impl PageIndex {
    pub fn new(reader: &mut R) -> Result<PageIndex> {
        PageIndex{}
    }
}