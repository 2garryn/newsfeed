use std::io::{ErrorKind, Result, Write, Read, Result};

pub struct Followers;
pub struct Followed;

impl Followers {
    pub fn read_from_file(reader: &Read) -> Result<Followers> {
        Ok(Followers)
    }
}