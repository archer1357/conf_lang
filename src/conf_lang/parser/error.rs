use std::error::Error;
use std::fmt;
use std::path::Path;

use crate::conf_lang::{Loc, error_line_src};



#[derive(Debug,Clone)]
pub struct ParseError {
    pub loc : Loc,
    pub msg : String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"At {} : {:?}",self.loc, self.msg)
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        &self.msg
    }
}


impl ParseError {
    pub fn eprint(&self,src:Option<&str>, p:Option<&Path>) {
        eprint!("{}",self.get_msg(src,p));
    }

    
    pub fn get_msg<'s>(&self,src:Option<&str>, p:Option<&Path>) -> String {
        let mut s = String::new();

        use std::fmt::Write;

        write!(s,"Parse error").unwrap();

        if let Some(p)=p {
            write!(s," in {p:?}").unwrap();
        }

        write!(s,":\n").unwrap();


        write!(s,"    {:?} at {}\n",self.msg, self.loc).unwrap();

        if let Some(src)=src {
            write!(s,"{}\n",error_line_src(src, self.loc)).unwrap();
        }

        s
    }

}