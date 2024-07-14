use std::str::Chars;

use super::loc::*;

#[derive(Debug,Clone)]
pub struct Input<'a> {
    chrs : Chars<'a>,
    buf : String,
    loc : Loc,
}

impl<'a> Input<'a> {
    pub fn new(chrs :Chars<'a>) -> Input {
        Self {
            chrs : chrs.clone(), 
            buf : String::new(), //Vec::new(), 
            loc : Loc::default(),
        }
    }

    pub fn get(&mut self, i : usize, n : usize) -> Option<&str> {
        let m = i+n;

        while self.buf.len() < m {
            if let Some(c) = self.chrs.next() {
                self.buf.push(c);
            } else {
                break;
            }
        }

        if self.buf.len() >= m { //on n is 0, returns Some("") ............why not None? Because consume() needs to?
            Some(&(self.buf[i..m]))
        } else {
            None
        }
    }

    fn calc_loc(&mut self, n : usize) {
        let mut loc = self.loc;

        if let Some(v) = self.get(0,n) {
            for c in v.chars() {
                loc.pos+=1;

                if c=='\n' {
                    loc.row+=1;
                    loc.col=0;
                    loc.line_start_pos = loc.pos;
                } else if c!='\r' {
                    loc.col+=1;
                }
            }
        
            self.loc = loc;
        }
    }

    pub fn next(&mut self, n : usize) {
        self.calc_loc(n);
        self.buf.drain(0 .. self.buf.len().min(n));
    }

    pub fn loc(&self) -> Loc {
        self.loc
    }
}
