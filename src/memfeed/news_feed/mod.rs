pub mod feeds_cache;
pub use feeds_cache::*;

use crate::memfeed::common_types::*;
use crate::memfeed::onwer_feed::*;
use crate::memfeed::activity::Activity;

pub struct NewsFeedId(pub FeedId);

pub struct NewsFeed <'a> {
    id: NewsFeedId,
    //onwer_feeds: Vec<&'a OwnerFeed<'a>>,
    activities: Vec<&'a Activity>
}