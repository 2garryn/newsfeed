
use crate::memfeed::common_types::*;
use crate::memfeed::activity::*;
use crate::memfeed::news_feed::*;


pub struct OwnerFeedId(pub FeedId);

pub struct OwnerFeed <'a> {
    id: OwnerFeedId,
    news_feeds: Vec<&'a NewsFeed<'a>>,
    activities: Vec<&'a Activity>,
    activities_limit: u16
}


impl OwnerFeed {
    pub fn load(owner_id: OwnerFeedId, store_prefix: String, act_limit: u16) -> Result<OwnerFeed> {
        let nf = Vec::with_capacity<(act_limit as usize);
        load_news_feeds(owner_id, store_prefix, act_limit, &mut nf)?
        OwnerFeed{
            id: OwnerFeedId, 
            news_feeds: load_news_feeds(owner_id, store_prefix, act_limit)?,
            activities: load_feeds(owner_id, store_prefix),
            activities_limit: act_limit
        }
    }

    fn load_news_feeds(owner_id: OwnerFeedId, store_prefix: String, act_limit: u16, )
}

