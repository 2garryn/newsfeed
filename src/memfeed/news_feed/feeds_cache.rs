use crate::memfeed::news_feed::*;

pub struct NewsFeedCache{

}

impl NewsFeedCache {
    pub fn get_feed(&self, feed_id: NewsFeedId) -> Option<&NewsFeed> {
        None
    }
}