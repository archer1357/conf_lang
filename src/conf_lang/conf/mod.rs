
mod container;

pub use container::*;
use std::{path::{PathBuf, Path}, fmt::Debug, sync::Arc};
use super::parser::*;

#[derive(Default, Debug)]
pub struct Conf { //<'a>
    records : Vec<Record>,
    path : Option<PathBuf>,
    // src : Option<Arc<String>>,
    src : Option<String>,
    
    // phantom:  std::marker::PhantomData<'a>,
}

impl std::fmt::Display for Conf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Conf")?;

        let mut stk = self.root().child_iter().rev().collect::<Vec<_>>();

        while let Some(record)=stk.pop() {
            stk.extend(record.child_iter().rev());

            let indent="    ".repeat(record.depth());
    
            let vals = record.value_iter()
                .map(|x|format!("{:?}",x.extracted.clone()))
                .collect::<Vec<_>>()
                .join(", ");

            writeln!(f, "{indent}{}",vals)?;
        }
        
        Ok(())
    }
}
impl Conf { //<'a>
    pub fn new(records : Vec<Record>, src : Option<&str>, path : Option<&Path>) -> Self {
        Self{
            records,
            // path:path.to_path_buf()
            path: path.and_then(|x|Some(x.to_path_buf())),
            // src : src.and_then(|x|Some(Arc::new(x.to_string()))),
            src : src.and_then(|x|Some(x.to_string())),
        }
    }
    pub fn root(&self) -> RecordContainer { //<'a>
        RecordContainer::new_root(self)
        // RecordContainer::new(
        //     self, //.records,
        //     0,
        //     // self.path.as_ref().and_then(|x|Some(x.as_path()))
        // ) //Some(self.path.as_path())
    }

    // pub fn get_src(&self) -> Option<Arc<String>> {
    //     self.src.clone()
    // }
    pub fn src(&self) -> Option<&str> {
        self.src.as_ref().map(|x|x.as_str())
    }
    pub fn path(&self) -> Option<&Path> {
        self.path.as_ref().map(|x|x.as_path())
    }
}

