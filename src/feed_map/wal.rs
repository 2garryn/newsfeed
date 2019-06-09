use std::fs::{File, OpenOptions, Metadata};
use std::io::{Result, Error, Seek, ErrorKind, SeekFrom, Write, Read};
use crate::activity::{FeedId, Activity, ActivityId};
use crate::store::StoreCall;
use std::path::{Path, PathBuf};

pub struct Wal {
    file: Option<File>,
    path: PathBuf
}

impl Wal {
    pub fn new(storage_path: &Path) -> Result<Wal> {
        let mut p = PathBuf::new().join(storage_path).join("wal.log");
        let mut f_opt = OpenOptions::new().read(true).append(true).open(&p);
        match f_opt {
            Ok(f) => 
                Ok(Wal{
                    file: Some(f),
                    path: p
                }),
            Err(ref e) if e.kind() == ErrorKind::NotFound => 
                Ok(Wal{
                    file: None,
                    path: p
                }),
            Err(e) => return Err(e)
        }
    }

    pub fn put(&mut self, activity: &Activity) -> Result<()> {
        self.ensure_file_created()?;
        ActionActivity::put(*activity)
            .write_to_store(&mut self.file.unwrap())?;
        self.file.unwrap().flush()
    }

    fn ensure_file_created(&mut self) -> Result<()> {
        match OpenOptions::new().read(true).append(true).open(self.path) {
            Ok(f) => {
                self.file = Some(f);
                Ok(())
            },
            Err(e) => 
                return Err(e)
        }
    }
}

#[repr(u8)]
enum ActionType {
    Put = 1,
    Delete = 2,
    Update = 3,
}


struct ActionActivity {
    action: ActionType,
    activity: Activity
}

impl ActionActivity {
    fn put(activity: Activity) -> ActionActivity {
        ActionActivity::new(ActionType::Put, activity)
    }

    fn delete(activity: Activity) -> ActionActivity {
        ActionActivity::new(ActionType::Delete, activity)
    }

    fn new(action_type: ActionType, activity: Activity) -> ActionActivity {
        ActionActivity{
            action: action_type,
            activity: activity
        }
    }

    fn store<W: Write>(&self, writer: &mut W) -> Result<()> {
        let a = self.action as u8;
        writer.write(&a.to_be_bytes())?;
        self.activity.write_to_store(writer)?;
        Ok(())
    }
}

impl StoreCall<ActionActivity> for ActionActivity {
    fn write_to_store<W: Write>(&self, writer: &mut W) -> Result<()> {
        let a = self.action as u8;
        writer.write(&a.to_be_bytes())?;
        self.activity.write_to_store(writer)?;
        Ok(())
    }

    fn read_from_store<R: Read>(reader: &mut R) -> Result<ActionActivity> {
        Err(Error::new(ErrorKind::Other, "Not Implemented"))
    }

    fn byte_size() -> u8 {
        1 + Activity::byte_size()
    }
}