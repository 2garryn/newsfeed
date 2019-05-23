mod memfeed;
use memfeed::activity::*;
use memfeed::common_types::FeedId;
use memfeed::onwer_feed::*;
use memfeed::news_feed::*;
use std::path::Path;
fn main() {
    let id = FeedId(12333666);
    let act = Activity{
        id: ActivityId(1233344432),
        onwer_feed: id,
        published: Published(1154645),
        spec: FilterSpec(9)
    };
    let mut mylist = Activities::load_from_file(id).unwrap();
    println!("{:?}", mylist);
    //mylist.add(act).unwrap();
    
    for x in 0..500 {
        let act1 = Activity{
        id: ActivityId(100 + x),
        onwer_feed: FeedId(1 + x),
        published: Published(1233),
        spec: FilterSpec(3)
        };
        mylist.add_one(act1).unwrap();
    };
    
}
