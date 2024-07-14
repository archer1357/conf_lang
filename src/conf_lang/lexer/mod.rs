
mod loc;
mod error;
pub mod input;

pub use loc::Loc;
pub use error::Error;

use error::*;
pub use input::*;

use std::str::Chars;

#[derive(Debug,Clone)]
pub struct Token {
    pub start_loc : Loc,
    pub end_loc : Loc,
    pub extracted : String,
}

#[derive(Debug,Clone)]
struct Hist<'a> {
    input : Input<'a>,
    token : Option<Token>,
}

pub struct Lexer<'a> {
    stk : Vec<Hist<'a>>,
    error_manager : ErrorManager,
}

impl<'a> Lexer<'a> {
    pub fn new(chrs :Chars<'a>) -> Self {
        Self {
            stk : vec![Hist {
                input : Input::new(chrs),
                token : None,
            }],
            error_manager : ErrorManager::new(),
        } 
    }

    pub fn loc(&self) -> Loc {
        let cur = self.stk.last().unwrap();
        cur.input.loc()
    }

    pub fn stack_size(&self) -> usize {
        self.stk.len()-1
    }

    pub fn push(&mut self) {
        let cur = self.stk.last().unwrap();

        let mut x = cur.clone();
        self.stk.push(x);

        self.error_manager.push();
    }

    pub fn pop_discard(&mut self) {
        if self.stk.len() <= 1 {
            panic!("Lexer stack size 0");
        }

        self.error_manager.on_pop_discard();
        self.stk.pop();
    }

    pub fn pop_keep(&mut self) {
        if self.stk.len() <= 1 {
            panic!("Lexer stack size 0");
        }

        self.error_manager.on_pop_keep();        
        self.stk.remove(self.stk.len()-2);
    }


    pub fn has<const N:usize>(&mut self, i:usize,xs: [&'static str;N]) -> Option<&'static str> {
        for &x in xs.iter() {
            if Some(x)==self.get(i, x.chars().count()) {
                return Some(x);
            }
        }

        None
    }

    pub fn get(&mut self, i : usize, n : usize) -> Option<&str> {
        let cur = self.stk.last_mut().unwrap();
        cur.input.get(i,n)
    }

    pub fn getc(&mut self, i : usize) -> Option<char> {         
        let cur = self.stk.last_mut().unwrap(); 

        if let Some(s) = cur.input.get(i,1) {
            s.chars().last()
        } else {
            None
        }
    }

    pub fn is_end(&mut self) -> bool {
        self.getc(0).is_none()
    }

    pub fn skip(&mut self, n : usize) {
        let cur = self.stk.last_mut().unwrap();
        cur.input.next(n);
        self.error_manager.on_next(self.loc());
    }

    pub fn consume(&mut self, n : usize, replace : Option<&str>) {
        let cur = self.stk.last_mut().unwrap();

        if let None = cur.token {
            cur.token = Some(Token {
                start_loc : cur.input.loc(),
                end_loc : cur.input.loc(),
                extracted : String::new(),
            });
        }

        if let Some(s) = cur.input.get(0,n) { //on n==0, s=""
            let s = if let Some(replace) = replace { replace } else {s};
            let token =  cur.token.as_mut().unwrap();
            token.extracted.extend(s.chars());
            cur.input.next(n);
            token.end_loc = cur.input.loc();
        }
        
        self.error_manager.on_next(self.loc());
    }

    pub fn token(&mut self) -> Option<Token> {
        let cur = self.stk.last_mut().unwrap();
        std::mem::take(&mut cur.token)
        
        // let t = self.cur.token ;
        // self.cur.token = None;
        // println!("{:?}",self.cur.token);
        // t
    }

    pub fn set_token(&mut self, token : Token) {
        let cur = self.stk.last_mut().unwrap();
        cur.token = Some(token);
    }

    // pub fn add_error(&mut self, msg_loc : Loc,msg : &str) {
    //     self.error_manager.add_error(self.loc(), msg_loc, msg);
    // }

    // pub fn get_errors(&self) -> &[Error] {
    //     self.error_manager.get_errors()
    // }

    // pub fn has_errors(&self) -> bool {
    //     self.error_manager.get_errors().len()>0
    // }
}