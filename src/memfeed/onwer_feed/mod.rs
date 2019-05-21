
use crate::memfeed::common_types::*;
use crate::memfeed::activity::*;
use crate::memfeed::news_feed::*;
use std::path::Path;
use std::fs::File;
use std::io::{ErrorKind, Result, Write, Read};
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::borrow::ToOwned;

static NEWS_FEED_PATH: &str = "/tmp/followers";
static ACTIVITIES_PATH: &str = "/tmp/own_activities";
static DEFAULT_CAPACITY: usize = 300;
static DEFAULT_EMPTY_CAPACITY: usize = 30;

#[derive(Debug)]
pub struct Activities {
    activities: Vec<Box<Activity>>,
    owner_id: FeedId,
    count: u32
}
impl Activities {
    pub fn load_from_file(owner_id: FeedId) -> Result<(Activities)> {
        let p = feed_path(owner_id, ACTIVITIES_PATH);
        let mut count: u32 = 0;
        let mut f = match OpenOptions::new().read(true).open(&p) {
            Ok(mf) => mf,
            Err(ref e) if e.kind() == ErrorKind::NotFound => 
                return Ok(Activities{
                    activities: Vec::with_capacity(DEFAULT_EMPTY_CAPACITY),
                    owner_id: owner_id,
                    count: 0
                }),
            Err(e) => 
                return Err(e)
        };
        let mut acts: Vec<Box<Activity>> = Vec::with_capacity(DEFAULT_CAPACITY);
        //let m = DEFAULT_CAPACITY as u32;
        for _ in 1..(DEFAULT_CAPACITY + 1) {
            match Activity::read_from_store(&mut f) {
                Ok(activity) => {
                    count = count + 1;
                    acts.push(Box::new(activity));
                },
                Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => 
                    break,
                Err(ref e) if e.kind() == ErrorKind::Interrupted =>
                    continue,
                Err(e) => 
                    return Err(e)
            }
        }
        Ok(Activities{
            activities: acts,
            owner_id: owner_id,
            count: count
        })
    }

    pub fn add_many(&mut self, acts: Vec<Activity>) -> Result<()> {
        let path = feed_path(self.owner_id, ACTIVITIES_PATH);
        let mut f = OpenOptions::new()
            .append(true)
            .open(&path)
            .or_else(move |_| {
                OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .open(&path)
            })?;
        for act in &acts {
            act.write_to_store(&mut f)?;
            self.activities.push(Box::new(*act));
        };
        f.flush()?;
        
        self.count = self.count + 1;
        Ok(())
    }

    pub fn add_one(&mut self, act: Activity) -> Result<()> {
        self.add_many(vec!(act))
    }

    pub fn get_start(&self, n: u32, res_buf: &mut Vec<Activities>) -> Result<ContId> {
        assert!(n < DEFAULT_CAPACITY as u32, true);
        let end_r = if (self.count - n) <= 0 {
            
        }
        for i in self.count.. {

        }
    }
}


pub struct ContId {
    activity_id: ActivityId,
    published: Published,
    finished: bool
}

pub struct GetResult {
    activities: Vec<Activity>,
    cont_id: ContId,
    finished: bool
}


pub struct NewsFeeds <'a> {
    news_feeds: Vec<&'a NewsFeed<'a>>,
    news_feeds_cache: &'a NewsFeedCache
}

impl <'a> NewsFeeds <'a> {
    pub fn new(nf_cache: &'a NewsFeedCache, capacity: usize) -> Self {
        NewsFeeds{
            news_feeds: Vec::with_capacity(capacity),
            news_feeds_cache: nf_cache
        }
    }

    pub fn create_feed(feed_id: FeedId, path: &Path) -> Result<()> {
        let p = feed_path(feed_id, NEWS_FEED_PATH);
        File::create(p)?;
        Ok(())
    }

    pub fn load_from_file(&mut self, owner_id: FeedId, path: &Path) -> Result<()> {
        let p = feed_path(owner_id, NEWS_FEED_PATH);
        let mut f = match File::open(p) {
            Ok(file) => file,
            Err(ref e) if e.kind() == ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(e)
        };
        
        loop {
            match FeedId::read_from_store(&mut f) {
                Ok(feed_id) => {
                    let nw_feed_id = NewsFeedId(feed_id);
                    match self.news_feeds_cache.get_feed(nw_feed_id) {
                        Some(nw_feed) => self.news_feeds.push(nw_feed),
                        None => continue
                    };
                },
                Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => 
                    return Ok(()),
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {
                    continue;
                },
                Err(e) => 
                    return Err(e)
            }
        }
    }
}

fn feed_path(feed_id: FeedId, path: &'static str) -> PathBuf {
    Path::new(path).join((feed_id.0).to_string())
}





