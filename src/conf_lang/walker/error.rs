use std::{path::{PathBuf, Path}, collections::HashMap, sync::Arc};

use crate::conf_lang::{Loc, error_line_src, RecordContainer};

use std::fmt::Write;
#[derive(Debug,Ord,Eq,PartialOrd,PartialEq,Clone)]
pub enum WalkerErrorType<E> {
    ParamsIncorrectNum,
    ParamParseError,
    ChildrenNotExpected,
    ChildrenExpected,  
    Custom(E),
    CustomInvalidValInd(usize,E),
    Unknown,// what is this for ?? no rules for input?
    ExpectedTagOnce,
    //ExpectedTag

}

pub struct WalkerErrorNode<E> {
    path : Option<PathBuf>,
    loc : Option<Loc>,
    nodes : Vec<WalkerErrorNode<E>>,
    error_type:Option<WalkerErrorType<E>>,
}
#[derive(Debug,Clone)]
pub struct WalkerError<E> {
    error_type:WalkerErrorType<E>,
    loc : Loc,
}
#[derive(Debug,Clone)]
pub struct WalkerErrors<E> {
    paths : Vec<Option<PathBuf>>,
    errors : Vec<Vec<WalkerError<E>>>,
    // srcs : Vec<Option<Arc<String>>>,
    // srcs : Vec<Option<&'a str>>,
}

impl<E:Ord+std::fmt::Debug> std::fmt::Display for WalkerErrors<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}",self.get_msg(|_|None))
    }
}

impl<E:Ord+std::fmt::Debug> std::error::Error for WalkerErrors<E> {
    fn description(&self) -> &str {
        "Walker Errors"
    }
}

impl<E:Ord+std::fmt::Debug> WalkerErrors<E> {
    pub fn eprint(&self) {
        eprint!("{}",self.get_msg(|_|None));
    }

    pub fn eprint_with_src<'s>(&self, get_src : impl Fn(&Path)->Option<&'s str>) {
        eprint!("{}",self.get_msg(get_src));
    }
    
    pub fn get_msg<'s>(&self, get_src : impl Fn(&Path)->Option<&'s str>) -> String {
        let mut s = String::new();

        for (path_ind,path) in self.paths.iter().enumerate() {
            let path_errors=&self.errors[path_ind];
            let src = path.as_ref().and_then(|p|get_src(p.as_path()));

            write!(s,"Walker error").unwrap();

            if path_errors.len() > 1 {
                write!(s,"s").unwrap();
            }

            if let Some(path)=path {
                write!(s," in config file: {:?}\n",path).unwrap();
            } else {
                write!(s,":\n").unwrap();
            }

            for (count,path_error) in path_errors.iter().enumerate() {
                write!(s,"  {}) {:?} at {}\n",count+1,path_error.error_type, path_error.loc).unwrap();

                if let Some(src)=src {
                    write!(s,"{}\n",error_line_src(src, path_error.loc)).unwrap();
                }
            }
        }

        s
    }

}

// #[derive(Debug,Clone)]
// struct Hist {
//     owned_ind : usize,
//     start_ind : usize,
// }

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Expectation<'a,E:Ord> {
    // src : Option<&'a str>,
    // path : Option<&'a Path>,
    // loc : Loc,
    record : RecordContainer<'a>,
    error_type : WalkerErrorType<E>,
    val_ind : usize,
}

#[derive(Default,Debug,Clone)]
pub struct Expecter<'a,E:Ord> {
    expects : Vec<Expectation<'a,E>>,
    hists : Vec<usize>,
    // srcs : HashMap<PathBuf,Option<Arc<String>>>,
    srcs : HashMap<&'a Path,&'a str>,
}


impl<'a,E:Ord+std::fmt::Debug+Clone> std::fmt::Display for Expecter<'a,E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.write_fmt(f)
    }
}

impl<'a,E:Ord+std::fmt::Debug+Clone> std::error::Error for Expecter<'a,E> {
    fn description(&self) -> &str {
        "Walker Expecter error"
    }
}

//std::fmt::Display+
impl<'a,E:Ord+std::fmt::Debug+Clone> Expecter<'a,E> {
    pub fn new() -> Self {
        Self { 
            expects : Vec::new(),
            hists : vec![0],
            srcs : HashMap::new(),
        }
    }

    pub fn has_errors(&self) -> bool {
        self.expects.len()>0
    }

    pub fn push(&mut self) {
        self.hists.push(self.expects.len());
    }

    pub fn pop_keep(&mut self) {
        self.hists.pop();
        // println!("e keep");
    }
    
    pub fn pop_discard(&mut self) {
        let ind = self.hists.pop().unwrap();
        // let n = self.expects.len(); //.. n
        self.expects.drain(ind ..);
    }
    

    pub fn expect(&mut self, 
        // path : Option<&'a Path>, loc : Loc, 
        record : RecordContainer<'a>,
        val_ind:usize,
        error_type : WalkerErrorType<E>) 
    {
        // let path = record.path();
        // let loc=record.value(val_ind)
        //     // .or_else(||record.value(0))
        //     .unwrap().start_loc;
        
        self.expects.push(Expectation {record,val_ind, error_type});

        if let (Some(path),Some(src))=(record.path(),record.src()) {
            self.srcs.insert(path, src);
        }
    }

    pub fn eprint(&self) {
        eprint!("{self}");
    }

    pub fn get_errors(&self) -> WalkerErrors<E> {
        
        let mut expects = Vec::from(self.expects[*self.hists.last().unwrap() ..].as_ref());
        
        expects.sort_by(|a,b|{
            let x=(&a.record.path(),a.record.value(a.val_ind).unwrap().start_loc,&a.error_type);
            let y=(&b.record.path(),b.record.value(b.val_ind).unwrap().start_loc,&b.error_type);
            x.cmp(&y)
        });

        expects.dedup();

        //
        // let mut v=Vec::<WalkerErrorNode<E>>::new();

        //problem if using multiple confs that don't have a path as it treats them all as from the same src
        //    need to somehow differentiate between the different confs?

        let mut paths=Vec::<Option<PathBuf>>::new();
        let mut errors = Vec::<Vec<WalkerError<E>>>::new();
        let mut path_inds = HashMap::<Option<&Path>,usize>::new();


        for expect in expects.iter() {
            let p = expect.record.path();

            let path_ind=if let Some(&path_ind)=path_inds.get(&p) {
                path_ind
            } else {
                let path_ind=paths.len();
                path_inds.insert(p, path_ind);
                paths.push(p.map(|x|x.to_path_buf()));
                errors.push(Default::default());
                path_ind
            };

            errors[path_ind].push(WalkerError { 
                error_type: expect.error_type.clone(), 
                loc: expect.record.value(expect.val_ind).unwrap().start_loc, 
            });
        }

        let mut srcs: Vec<Option<&'a str>>= Vec::new();

        // for path in paths.iter() {
        //     srcs.push(path.as_ref().and_then(|path|self.srcs.get(path.as_path()).map(|&x|x)));
        // }

        WalkerErrors { paths, errors, //srcs:Default::default()
        }
    }

    pub fn write_fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // let mut expects = Vec::from(self.expects[*self.hists.last().unwrap() ..].as_ref());

        // expects.sort();
        // expects.dedup();

        // // self.expects.sort_by_key(|x|(x.path.clone(),x.loc));
        // // self.expects.dedup_by_key(|x|(x.path.clone(),x.loc));

        // // let s = String::new();


        // let mut last_path = None;
        // let mut count = 1;

        // let empty_str = String::new();

        // let mut i=0;

        // while i< expects.len() {
            
        //     if last_path != Some(expects[i].path.clone()) {
        //         last_path = Some(expects[i].path.clone());
        //         count = 0; //1

        //         if expects.len() == 1 {
                  
        //             write!(f,"Error in config file: {:?}\n",expects[i].path)?;
        //         } else {
        //             write!(f,"Errors in config file: {:?}\n",expects[i].path)?;
        //         }
        //     }
            
        //     let loc = expects[i].loc;

        //     let mut j=i;
        //     let mut loc_expects=Vec::new();

        //     while j<expects.len() && expects[j].loc==loc && Some(expects[j].path.clone())==last_path {
        //         loc_expects.push(expects[j].error_type.clone());
        //         j+=1;                
        //         count+=1;
        //     }

            

        //     let strs : Vec<String> = loc_expects.iter().map(|x|format!("{:?}",x)).collect();
        //     let joined_strs = strs.join(", ");
        //     // eprintln!(":{}",joined_strs);
        //     write!(f,"  {}) {} at {}\n",count,joined_strs,loc)?;
        //     //


        //     // if let Some(src)=expects[i].src {
        //     //     write!(f,"{}\n",error_line_src(src, loc))?;
        //     // }

        //     i=j;
        // }
   
        Ok(())
    }
}

