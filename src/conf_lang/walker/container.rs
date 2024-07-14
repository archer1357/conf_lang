use std::{collections::HashMap, path::{Path, PathBuf}};

use crate::conf_lang::RecordContainer;

use super::val_parsing::ParsedVal;


pub struct WalkerRecord<'r> {
    pub record : RecordContainer<'r>,
    pub parsed_values : HashMap<usize,ParsedVal>,
    pub parsed_names : HashMap<usize,String>,
    pub node_vals_len : usize,
    pub has_tag : bool,
    pub has_repeat : bool,
    pub parent : Option<usize>,
    pub children : Vec<usize>,
    pub node_label : String,
    pub branch_label : String,
    pub depth : usize,
    pub order : usize,
}

#[derive(Clone,Copy)]
pub struct WalkerRecordContainer<'w,'r> {
    pub walker_record_index : usize,
    pub walker_records : &'w Vec<WalkerRecord<'r>>,
}

impl<'w,'r> WalkerRecordContainer<'w,'r> {
    pub fn label(&self) -> &str {
        self.walker_record().node_label.as_str()
    }

    pub fn branch(&self) -> &str {
        self.walker_record().branch_label.as_str()
    }

    fn walker_record(&self) -> &'w WalkerRecord<'r> {
        &self.walker_records[self.walker_record_index]
    }


    fn parent_walker_record(&self) -> Option<&'w WalkerRecord<'r>> {
        let parent=self.walker_record().parent;

        if let Some(parent)=parent {
            Some(&self.walker_records[parent])
        } else {
            None
        }
    }
    pub fn walker_record_index(&self) -> usize {
        self.walker_record_index
    }

    pub fn get_child(&self, child_index : usize) -> Option<WalkerRecordContainer<'w,'r>>  {
        if let Some(&child_record_index)=self.walker_record().children.get(child_index) {
            Some(Self{
                walker_records:self.walker_records, 
                walker_record_index :child_record_index
            })
        } else {
            None
        }
    }

    pub fn child_num(&self) -> usize {
        self.walker_record().children.len()
    }

    pub fn get_parent(&self) -> Option<WalkerRecordContainer<'w,'r>> {
        if let Some(parent_record_index)= self.walker_records[self.walker_record_index].parent {
            Some(Self{walker_records:self.walker_records, walker_record_index :parent_record_index})
        } else {
            None
        }
    }

    pub fn get_ancestor(&self, ancestor_index : usize) -> Option<WalkerRecordContainer<'w,'r>> {
        let mut cur = self.get_parent();
        let mut i = 0;

        while let Some(x) = cur {
            if i==ancestor_index {
                return cur;
            } else {
                cur = x.get_parent();
                i+=1;
            }
        }

        None
    }

    pub fn depth(&self) -> usize {
        self.walker_record().depth
    }

    pub fn path(&self) -> Option<&'r Path> {
        // self.walker_record().path
        // self.walker_record().path.as_path()
        self.walker_record().record.path()
    }

    pub fn order(&self) -> usize {
        self.walker_record().order
    }

    //

    // pub fn from_index(&self, index : usize) -> Option<WalkerRecordContainer<'w,'r>> { //grab another walker_record_container via this one
    //     if index < self.walker_records.len() {
    //         Some(WalkerRecordContainer {
    //             walker_record_index : index,
    //             walker_records : self.walker_records,
    //         })
    //     } else {
    //         None
    //     }
    // }

    // pub fn index(&self) -> usize {
    //     self.walker_record_index
    // }

    //

    // pub fn record(&self) -> RecordContainer<'r> {
    //     self.walker_record().record
    // }

    //

    // pub fn value_iter(&self) -> crate::conf_lang::ValueIter {
    //     self.walker_record().record.value_iter()
    // }

    // pub fn node_vals_len(&self) -> usize { //from walker node
    //     self.walker_record().node_vals_len
    // }

    // pub fn has_repeat(&self) -> bool {
    //     self.walker_record().has_repeat
    // }

    pub fn has_tag(&self) -> bool {
        self.walker_record().has_tag
    }

    pub fn tag(&self) -> &str {
        if self.has_tag() {
            // self.str(0).unwrap()
            self.get_tag_val().unwrap().extracted.as_str()
        } else {
            ""
        }
    }

    pub fn parent_tag(&self) -> &str {
        if let Some(parent_walker_record)=self.parent_walker_record() {
            if parent_walker_record.has_tag {
                return parent_walker_record.record.value(0).unwrap().extracted.as_str();
            }
        }

        ""
    }

    pub fn get_tag_val(&self) -> Option<&crate::conf_lang::Value> {
        if self.has_tag() {
            self.walker_record().record.value(0)
        } else {
            None
        }
    }
    pub fn get_val(&self, value_index : usize) -> Option<&crate::conf_lang::Value> {
        let value_index=if self.has_tag() {value_index+1} else {value_index};
        self.walker_record().record.value(value_index)
    }    
    pub fn val_num(&self) -> usize {
        let c=self.walker_record().record.value_count();
        if self.has_tag() {c-1} else {c}
    }

    // pub fn str(&self,value_index : usize) -> Option<&str> {
    //     // let value_index=if self.has_tag() {value_index+1} else {value_index};
    //     // self.walker_record().record.value(value_index).and_then(|x|Some(x.extracted.as_str()))
    //     self.val(value_index).and_then(|x|Some(x.extracted.as_str()))
    // }

    pub fn str(&self,value_index : usize) -> &str {
        self.get_str(value_index).unwrap_or_default()
    }
    
    pub fn get_str(&self,value_index : usize) -> Option<&str> {
        self.get_val(value_index).and_then(|x|Some(x.extracted.as_str()))
    }

    // pub fn parent_str(&self,value_index : usize) -> Option<&str> {
    //     let value_index=if self.has_tag() {value_index+1} else {value_index};
    //     self.parent_walker_record().and_then(|w|w.record.value(value_index)).map(|x|x.extracted.as_str())
    // }


    pub fn parent_str(&self,value_index : usize) -> &str {
        let value_index=if self.has_tag() {value_index+1} else {value_index};

        self.parent_walker_record()
            .and_then(|w|w.record.value(value_index))
            .map(|x|x.extracted.as_str())
            .unwrap_or("")
    }
    
    // pub fn parsed<T:Copy+'static+std::fmt::Debug+Default>(&self,value_index : usize) -> T {
    //     self.get_parsed(value_index).unwrap_or_default()
    // }

    pub fn get_parsed<T:Copy+'static+std::fmt::Debug>(&self,value_index : usize) -> Option<T> {
        let value_index=if self.has_tag() {value_index+1} else {value_index};
        self.walker_record().parsed_values.get(&value_index).and_then(|x|x.get())
    }

    pub fn parsed_name(&self,value_index : usize) -> Option<&str> {
        let value_index=if self.has_tag() {value_index+1} else {value_index};
        self.walker_record().parsed_names.get(&value_index).map(|x|x.as_str())
    }

    pub fn test(&self) {
        println!("test hastag={:?}, tag={:?}, valcount={}",self.has_tag(),self.tag(),self.val_num());
        println!("test parsed_values={:?}",self.walker_record().parsed_values);
        println!("test parsed_names={:?}",self.walker_record().parsed_names);
        println!("test vals={:?}",self.walker_record().record.value_iter().collect::<Vec<_>>());
    }
}
