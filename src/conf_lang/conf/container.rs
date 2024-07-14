use std::{path::{PathBuf, Path}, fmt::Debug, sync::Arc};

// use super::{Record, Value};
use super::{super::parser::*, Conf};

#[derive(Clone,Copy)]
pub struct RecordContainer<'a> {
    record_index : usize,
    // records : &'a Vec<Record>,
    // path : Option<&'a Path>,
    conf : &'a Conf,
    
    pub depth : usize, //root is depth 0, and root's children are depth 1, unlike WalkerRecord? which root's children are depth 0?
}
impl<'a> Debug for RecordContainer<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        
        self.conf.records.get(self.record_index).unwrap().fmt(f)
        // f.debug_struct("Record")
        //  .field("x", &self.x)
        //  .field("y", &self.y)
        //  .finish()
    }
}

impl<'a> PartialEq for RecordContainer<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.conf.path==other.conf.path && 
        self.value(0).unwrap().start_loc == other.value(0).unwrap().start_loc
    }
}

impl<'a> Eq for RecordContainer<'a> {}

impl<'a> std::hash::Hash for RecordContainer<'a> {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.conf.path.hash(hasher);
        self.value(0).unwrap().start_loc.hash(hasher);
    }
}
impl<'a> PartialOrd for RecordContainer<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.conf.path.partial_cmp(&other.conf.path) {
            Some(std::cmp::Ordering::Equal) => self.value(0).unwrap().start_loc.partial_cmp(&other.value(0).unwrap().start_loc),
            x => x
        }
    }
}

impl<'a> Ord for RecordContainer<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.conf.path.cmp(&other.conf.path) {
            std::cmp::Ordering::Equal => self.value(0).unwrap().start_loc.cmp(&other.value(0).unwrap().start_loc),
            x => x
        }
    }
}

impl<'a> RecordContainer<'a> {
    pub fn new_root(conf : &'a Conf) -> Self{ //records : &'a Vec<Record>,path : Option<&'a Path>
        Self { conf, record_index:0, depth:0 } //records, path 
    }
    // pub fn new(conf : &'a ConfLang, record_index : usize, ) -> Self{ //records : &'a Vec<Record>,path : Option<&'a Path>
    //     Self { conf, record_index, } //records, path 
    // }

    fn record(&self) -> &'a Record {
        &self.conf.records[self.record_index]
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn record_index(&self) -> usize {
        self.record_index
    }

    pub fn value(&self, value_index : usize) -> Option<&'a Value> {
        self.record().values.get(value_index)
    }
    
    pub fn value_count(&self) -> usize {
        self.record().values.len()
    }

    pub fn child(&self, child_index : usize) -> Option<RecordContainer<'a>>  { //-> &String
        if let Some(&child_record_index)=self.record().children.get(child_index) {
            Some(Self{conf:self.conf,record_index :child_record_index,depth:self.depth+1}) //records:self.records, , path:self.path
        } else {
            None
        }
    }

    pub fn child_count(&self) -> usize {
        self.record().children.len()
    }

    pub fn parent(&self) -> Option<RecordContainer<'a>> {
        if let Some(parent_record_index)= self.conf.records[self.record_index].parent {
            Some(Self{conf:self.conf, record_index :parent_record_index,depth:self.depth-1})
        } else {
            None
        }
    }

    pub fn ancestor(&self, ancestor_index : usize) -> Option<RecordContainer<'a>> {
        let mut cur = self.parent();
        let mut i = 0;

        while let Some(x) = cur {
            if i==ancestor_index {
                return cur;
            } else {
                cur = x.parent();
                i+=1;
            }
        }

        None
    }

    pub fn ancestor_iter(&self) -> AncestorIter<'a> {
        // AncestorIter {conf:self.conf,record_index:self.record_index}
        AncestorIter { record: Some(*self) }
    }
    
    pub fn child_iter(&self) -> ChildIter<'a> {
        // ChildIter::<'a> {record_index:self.record_index,child_index:0,conf:self.conf}
        ChildIter::<'a> {record:*self,child_index:0,child_back_index:self.child_count()}
    }

    pub fn value_iter(&self) -> ValueIter<'a> {
        ValueIter {record_index:self.record_index,value_index:0,conf:self.conf}
    }

    pub fn path(&self) -> Option<&'a Path> {
        self.conf.path.as_ref().and_then(|x|Some(x.as_path()))
    }

    pub fn src(&self) -> Option<&'a str> {
        self.conf.src.as_ref().and_then(|x|Some(x.as_str()))
    }
    
    // pub fn src(&self) -> Option<Arc<String>> {
    //     self.conf.get_src()
    // }
    
}

pub struct ChildIter<'a> {
    // record_index : usize,
    // child_index : usize,
    child_index : usize, //if 0, then 0 hasnt been traversed yet
    child_back_index : usize, //if last_ind then last_ind has been traversed
    // // records : &'a Vec<Record>,
    // // path : Option<&'a Path>,
    // conf :&'a ConfLang,

    record :RecordContainer<'a>,
}

impl<'a> Iterator for ChildIter<'a> {
    type Item = RecordContainer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // let record = &self.conf.records[self.record_index];

        // if let Some(&c) = record.children.get(self.child_index)
        if let Some(child_record)=self.record.child(self.child_index) {
            if self.child_index < self.child_back_index {
                self.child_index+=1;
                Some(child_record)
            } else {
                None
            }
        } else {
            None
        }

        // if self.child_index < record.children.len() {
        //     let c = record.children[self.child_index];
        //     self.child_index+=1;
        //     // Some(RecordContainer::new(self.conf,c))
        //     Some(RecordContainer { conf: self.conf, record_index: c })
        // } else {
        //     None
        // }
    }
}

impl<'a> DoubleEndedIterator for ChildIter<'a> {
    fn next_back(&mut self) -> Option<RecordContainer<'a>> {
        if self.child_back_index > self.child_index {
            self.child_back_index-=1;
            self.record.child(self.child_back_index)
        } else {
            None
        }


        // let record = &self.conf.records[self.record_index];
        
        // if self.child_index < record.children.len() {
        //     let c = record.children[record.children.len() - self.child_index - 1];
        //     self.child_index+=1;
        //     Some(RecordContainer::new(self.conf,c))
        // } else {
        //     None
        // }
    }
}

pub struct AncestorIter<'a> {
    // record_index : usize,
    // // records : &'a Vec<Record>,
    // // path : Option<&'a Path>,
    // conf : &'a ConfLang,

    record : Option<RecordContainer<'a>>,
}

impl<'a> Iterator for AncestorIter<'a> {
    type Item = RecordContainer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(record)=self.record {
            self.record=record.parent();
            self.record
        } else {
            None
        }
        // let record = &self.conf.records[self.record_index];

        // if let Some(p) = record.parent {
        //     self.record_index=p;
        //     Some(RecordContainer::new(self.conf,p))
        // } else {
        //     None
        // }
    }
}

pub struct ValueIter<'a> {
    record_index : usize,
    value_index : usize,
    conf : &'a Conf,
    //records : &'a Vec<Record>,
}

impl<'a> Iterator for ValueIter<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        let record = &self.conf.records[self.record_index];

        if let Some(v) = record.values.get(self.value_index) {
            self.value_index+=1;
            Some(v)
        } else {
            None
        }
    }
}
