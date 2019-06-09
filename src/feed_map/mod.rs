


type BoxedOwnerCell = Box<OwnerCell>;
type BoxedFollowerCell = Box<FollowerCell>;
type BoxedOwner = Box<Onwer>;

pub struct Owner;

pub struct OwnerWal;

pub struct FollowerId;

pub struct OwnerCell {
    n: u32
//    owners: Vec<BoxedOwner>,
//    followers: 
//    wal: OwnerWal

}

impl OwnerCell {
    pub fn new(n: u32) {
        OwnerCell{
            n: n
        }
    }
}

pub struct FollowerCell {
    n: u32
}


impl FollowerCell {
    pub fn new(n: u32) -> FollowerCell {
        FollowerCell{
            n: u32
        }
    }

    pub fn start(&mut self) -> Result<()> {
        
    }
}


pub struct FeedMap {
    owner_cells: Vec<BoxedOwnerCell>,
    follower_cells: Vec<BoxedFollowerCell>,
    n: u32
}


impl FeedMap {
    pub fn new() -> FeedMap {
        let n = 101;
        FeedMap{
            n: n,
            owner_cells: Vec::with_capacity(n),
            follower_cells: Vec::with_capacity(n)
        }
    }

    pub fn start(&self) -> Result<()> {
        self.start_followers()?;
        self.start_owners()
    }


    fn start_owners(&mut self) -> Result<()> {
        for i in 1..self.n {
            let oc = OwnerCell::new(i);
            let bx = Box::new(oc);
            self.owner_cells.push(bx);
        }
    }

    fn start_followers(&mut self) -> Result<()> {
        for i in 1..self.n {
            let c = FollowerCell::new(i);
            let bx = Box::new(c);
            self.follower_cells.push(bx);
        }
    }
}

