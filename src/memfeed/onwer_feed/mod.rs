
use crate::memfeed::common_types::*;
use crate::memfeed::activity::*;
use crate::memfeed::news_feed::*;
use std::path::Path;
use std::fs::{File, OpenOptions, Metadata};
use std::fs;
use std::io::{ErrorKind, Result, Write, Read};
use std::path::PathBuf;
use std::borrow::ToOwned;
use std::io::SeekFrom;
use std::io::prelude::*;

static NEWS_FEED_PATH: &str = "/tmp/followers";
static ACTIVITIES_PATH: &str = "/tmp/own_activities";
static CHUNK_SIZE: u64 = 100;

type ActivityCell = Vec<Box<Activity>>;

#[derive(Debug)]
pub struct ActivityPage {
    activities: ActivityCell,
    f_start: u64,
    acts_amount: u64
}

impl ActivityPage {
    pub fn read_from_position(prev_page_start: u64, myf: &mut File, n_acts: u64) -> Result<ActivityPage> {
        let act_size = Activity::size();
        let mut start: u64 = 0;
        let mut acts_amount = n_acts;
        let expect_size = n_acts * act_size;
        if prev_page_start < expect_size {
            acts_amount = prev_page_start / act_size;
        } else {
            start = prev_page_start - expect_size;
        };
        myf.seek(SeekFrom::Start(start))?;
        let mut acts: ActivityCell = Vec::with_capacity(acts_amount as usize);
        for _ in 0..acts_amount {
            match Activity::read_from_store(myf) {
                Ok(activity) => {
                    acts.push(Box::new(activity));
                },
                Err(ref e) if e.kind() == ErrorKind::Interrupted =>
                    continue,
                Err(e) => 
                    return Err(e)
            }
        }
        Ok(ActivityPage{
            activities: acts,
            f_start: start,
            acts_amount: acts_amount
        })
    }

    pub fn create_file(writer: &mut Write, new_acts: Vec<Activity>) -> Result<ActivityPage> {
        let acts_amount = new_acts.len();
        let mut acts: ActivityCell = Vec::with_capacity(acts_amount);
        for act in &new_acts {
            act.write_to_store(writer)?;
            acts.push(Box::new(*act));
        };
        writer.flush()?;
        Ok(ActivityPage{
            activities: acts,
            f_start: 0,
            acts_amount: acts_amount as u64
        })
    }

    pub fn add_many(&mut self, writer: &mut Write, new_acts: Vec<Activity>) -> Result<()> {
        let acts_amount = new_acts.len();
        for act in &new_acts {
            act.write_to_store(writer)?;
            self.activities.push(Box::new(*act));
        };
        writer.flush()?;
        self.acts_amount = self.acts_amount + (acts_amount as u64);
        Ok(())
    }

    pub fn get(&self, n: u64) -> Box<Activity> {
        self.activities[n as usize]
    }

    pub fn recent(&self) -> Box<Activity> {
        self.activities[self.activities.len()]
    }

    pub fn oldest(&self) -> Box<Activity> {
        self.activities[0]
    }

    pub fn maybe_contid_here(&self, cont_id: NextContId) -> bool {
        let recent = self.recent();
        let oldest = self.oldest();
        cont_id.published.0 >= oldest.published.0 &&
        cont_id.published.0 <= recent.published.0
    }

    pub fn read_older(&self, start_act_id: ActivityId, start_pub: Published, n: u64, acts: Vec<Box<Activity>>) -> u64 {
        1
    }

    pub fn read_newer(&self, n: u64, acts: Vec<Box<Activity>>) -> u64 {
        2
    }
}

#[derive(Debug)]
pub struct Activities {
    pages: Vec<ActivityPage>,
    owner_id: FeedId
}
impl Activities {
    pub fn load_from_file(owner_id: FeedId) -> Result<(Activities)> {
        let p = feed_path(owner_id, ACTIVITIES_PATH);
        let prev_page_start = match fs::metadata(&p) {
            Ok(metadata) => metadata.len(),
            Err(ref e) if e.kind() == ErrorKind::NotFound => 
                return Ok(Activities{
                    pages: Vec::with_capacity(0),
                    owner_id: owner_id
                }),
            Err(e) => return Err(e)
        };
        let mut f = OpenOptions::new().read(true).open(&p)?;
        let mut pages: Vec<ActivityPage> = Vec::with_capacity(1);
        let page = ActivityPage::read_from_position(prev_page_start, &mut f, CHUNK_SIZE)?;
        pages.push(page);
        Ok(Activities{
            pages: pages,
            owner_id: owner_id
        })
    }
    pub fn add_many(&mut self, acts: Vec<Activity>) -> Result<()> {
        match self.pages.len() {
            0 => self.create_and_add(acts)?,
            _ => self.just_add_many(acts)?
        };
        Ok(())
    }

    pub fn add_one(&mut self, act: Activity) -> Result<()> {
        self.add_many(vec!(act))
    }

    pub fn get_feed_start(&self, num: u64, acts: Vec<Box<Activity>>) -> Result<ContId> {
        if self.pages.len() == 0 {
            return Ok(None)
        };
        let recent_act: Box<Activity> = self.pages[0].last();
        let cont_id: NextContId = NextContId::new(recent_act, 0);
        self.get_feed_next(cont_id, num, acts)
    }

    pub fn get_feed_older(&self, cont_id: NextContId, num: u64, acts: Vec<Box<Activity>>) -> Result<ContId> {
        let start_act_id = cont_id.activity_id;
        let start_publ = cont_id.published;
        let mut num_read = num;
        for page in &self.pages{
            if page.maybe_contid_here(cont_id) {
                let received = page.read_older(start_act_id, start_publ, acts);
                

            }
        }
    }



    fn create_and_add(&mut self, acts: Vec<Activity>) -> Result<()> {
        let p = feed_path(self.owner_id, ACTIVITIES_PATH);
        let mut f = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&p)?;
        let page = ActivityPage::create_file(&mut f, acts)?;
        self.pages.push(page);
        Ok(())
    }

    fn just_add_many(&mut self, acts: Vec<Activity>) -> Result<()> {
        let p = feed_path(self.owner_id, ACTIVITIES_PATH);
        let mut f = OpenOptions::new()
            .append(true)
            .open(&p)?;
        self.pages[0].add_many(&mut f, acts)?;
        Ok(())
    }


}


type ContId  = Option<NextContId>;

pub struct NextContId {
    activity_id: ActivityId,
    published: Published,
    page_n: u64
}

impl NextContId {
    pub fn new(act: Box<Activity>, page_n: u64) -> NextContId {
        NextContId{
            activity_id: act.id,
            published: act.published,
            page_n: page_n
        }
    }
}




/*
#[derive(Debug, Copy, Clone)]
pub struct ContId {
    activity_id: ActivityId,
    published: Published,
    finished: bool,
    position: usize
}

impl ContId {
    pub fn new(activity: Activity, finished: bool, position: usize) -> ContId {
        ContId{
            activity_id: activity.id,
            published: activity.published,
            finished: finished,
            position: position
        }
    }
}
*/





fn feed_path(feed_id: FeedId, path: &'static str) -> PathBuf {
    Path::new(path).join((feed_id.0).to_string())
}





