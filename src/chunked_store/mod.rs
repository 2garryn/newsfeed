use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions, Metadata};
use std::io::{Result, Seek, ErrorKind, SeekFrom};
use std::ops::Drop;
use crate::store::{
    RelativeRequest, 
    PaginationRequest,
    ActivityList,
    StoreCall
};
use crate::activity::{FeedId, Activity, ActivityId};

mod page_index;
mod pages;
mod wal;
use page_index::PageIndex;
use pages::Pages;
use wal::Wal;

pub struct LStore {
    index: PageIndex,
    pages: Pages,
    wal: Wal,
    feed_id: FeedId
}


impl LStore {
    pub fn new(feed_id: FeedId, storage_path: &Path) -> Result<LStore> {
        let mut path = PathBuf::new().join(storage_path).join(feed_id.to_string());
        Ok(LStore {
            index: PageIndex::new(feed_id, &path)?,
            pages: Pages::new(feed_id, &path)?,
            wal: Wal::new(&path)?,
            feed_id: feed_id
        })
    }

    pub fn put_activity(&self, activity: &Activity) -> Result<()> {
        self.wal.put(activity)?;

        Ok(())
    }
}




/*


pub struct ChunkedStore {
    feed_path: PathBuf,
    feed_id: FeedId,
    fd: Option<File> 
}

impl ChunkedStore {
    fn ensure_end_file(&mut self) -> Result<()> {
        if let Some(f) = self.fd {
            f.seek(SeekFrom::End(0))?;
            return Ok(());
        };
        self.open_file()
    }

    fn open_file(&mut self) -> Result<()> {
        let mut f = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&self.feed_path)?;
        self.fd = Some(f);
        Ok(())
    }

    fn new(feed_id: FeedId, path: &String) -> ChunkedStore {
        ChunkedStore{
            feed_id: feed_id,
            feed_path: Path::new(path)
                .join(feed_id.to_string())
                .join("feeds"),
            fd: None
        }
    }

    fn put_activity(&mut self, activity: &Activity) -> Result<()> {
        self.ensure_end_file()?;
        activity.write_to_store(&mut self.fd.unwrap())

    }
    fn get_activity(&self, id: ActivityId) -> Result<Option<Activity>> {

    }
    fn delete_activity(&mut self, id: ActivityId) -> Result<()> {
        Ok(())
    }

    fn pagination(&self, request: PaginationRequest, acts: &mut ActivityList) -> Result<()> {
        Ok(())
    }

    fn gte(&self, request: RelativeRequest, acts: &mut ActivityList) -> Result<()> {
        Ok(())
    }
    fn gt(&self, request: RelativeRequest, acts: &mut ActivityList) -> Result<()> {
        Ok(())
    }

    fn lte(&self, request: RelativeRequest, acts: &mut ActivityList) -> Result<()> {
        Ok(())
    }
    fn lt(&self, request: RelativeRequest, acts: &mut ActivityList) -> Result<()> {
        Ok(())
    }


}

*/

