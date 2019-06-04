use std::io::Result;

pub struct Pages {
    feed_path: PathBuf,
    feed_id: FeedId,
    index: Option<PageIndex>,
    index_path: PathBuf,
    pages: Vec<Box<Page>>
}


impl Pages {
    pub fn new(feed_id: FeedId, path: &String) -> ChunkedStore {
        Pages{
            feed_id: feed_id,
            feed_path: Path::new(path).join(feed_id.to_string()).join("pages"),
            index_path: Path::new(path).join(feed_id.to_string()).join("index"),
            index: None,
            pages: Vec::new()
        }
    }

    pub fn put_activity(&mut self, act: Activity) -> Result<()> {
        self.read_pages()?;
        match self.pages.is_empty() {
            true -> create_first_page(&act)?,
            false -> add_to_exist_page(&act)?
        }
    }

    fn create_first_page(&mut self, act: &Activity) -> Result<()> {

    }

    fn read_pages() -> Result<bool> {

    }

    fn list_pages() -> 


}