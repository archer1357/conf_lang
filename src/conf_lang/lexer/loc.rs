
#[derive(Default,Debug,Copy,Clone, Hash,Eq,Ord)]
pub struct Loc {
    pub pos : usize,
    pub row : usize,
    pub col : usize,
    pub line_start_pos : usize,
}

impl PartialEq for Loc {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl PartialOrd for Loc {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.pos.partial_cmp(&other.pos)
    }
}

impl std::fmt::Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"line {}, column {}, position {}",self.row+1,self.col+1,self.pos+1)
    }
}
