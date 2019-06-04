extern crate newsfeed;
use newsfeed::memfeed::common_types::FeedId;
use newsfeed::memfeed::onwer_feed::ActivityPages;
use newsfeed::memfeed::onwer_feed::NextContId;
use newsfeed::memfeed::activity::FilterSpec;
use newsfeed::memfeed::activity::ActivityId;
use newsfeed::memfeed::activity::Published;
use newsfeed::memfeed::activity::Activity;
use std::fs;


#[test]
fn test_something() {
    assert_eq!(1, 1);
    assert_eq!(newsfeed::add(1, 2), 3);
}

#[test]
fn test_own_feed_create() {
    cleanup_store();
    let feed_id = FeedId(12);
    let mut acts = generate_activities(500, feed_id, FilterSpec(2));
    let first_act = acts[215];
    let mut act_pages = ActivityPages::load_from_file(feed_id).unwrap();
    act_pages.add_many(acts).unwrap();
    let next_cont_id = NextContId::new(Box::new(first_act));

    let mut act_pages2 = ActivityPages::load_from_file(feed_id).unwrap();
    let next_cont_id = NextContId::new(Box::new(first_act));
    act_pages2.find_start_activity(&next_cont_id);


    let mut buf: Vec<Box<Activity>> = vec![];
    let res = act_pages2.get_feed_start(1, &mut buf);

    println!("buffer value {:?} Result {:?}", buf, res);
}

fn cleanup_store() {
    fs::remove_dir_all("/tmp/own_activities");
    fs::create_dir_all("/tmp/own_activities").unwrap();
}


fn generate_activities(n: u64, feed_id: FeedId, spec: FilterSpec) -> Vec<Activity> {
    let mut acts: Vec<Activity> = vec![];
    for i in 0..n {
        let act = new_activity(feed_id, i, spec);
        acts.push(act);
    }
    acts
}

fn new_activity(feed_id: FeedId, i: u64, spec: FilterSpec) -> Activity {
    Activity{
        id: ActivityId((1000 + i) as u128),
        onwer_feed: feed_id,
        published: Published(100000 + i),
        spec: spec
    }
}