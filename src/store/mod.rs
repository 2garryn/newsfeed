
use std::io::{Read, Write, Result};
use crate::activity::{Activity, ActivityId, FeedId, Filter};

pub struct PaginationOffset(pub u32);
pub struct PaginationLimit(pub u8);
pub type ActivityList = Vec<Activity>;

pub struct PaginationRequest{
    pub offset: PaginationOffset,
    pub limit: Option<PaginationLimit>,
    pub filter: Option<Filter>
}

pub struct RelativeRequest{
    pub activity_id: ActivityId,
    pub limit: Option<PaginationLimit>,
    pub filter: Option<Filter>
}

pub trait StoreCall<T> {
    fn read_from_store<R: Read>(reader: &mut R) -> Result<T>;
    fn write_to_store<W: Write>(&self, writer: &mut W) -> Result<()>;
    fn byte_size() -> u8;
}

/*
pub trait ActivityStorage<ImplStorage> {
    fn new(feed_id: FeedId, storage_path: &String) -> ImplStorage;

    fn put_activity(&mut self, activity: &Activity) -> Result<()>;
    fn get_activity(&self, id: ActivityId) -> Result<Activity>;
    fn delete_activity(&mut self, id: ActivityId) -> Result<()>;

    fn pagination(&self, request: PaginationRequest, acts: &mut ActivityList) -> Result<()>;

    fn gte(&self, request: RelativeRequest, acts: &mut ActivityList) -> Result<()>;
    fn gt(&self, request: RelativeRequest, acts: &mut ActivityList) -> Result<()>;

    fn lte(&self, request: RelativeRequest, acts: &mut ActivityList) -> Result<()>;
    fn lt(&self, request: RelativeRequest, acts: &mut ActivityList) -> Result<()>;

}
*/