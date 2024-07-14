/*
TODO
* make walker a part of the parser,
- let children of a node be specified as a single string

* have return types specified for type overloaded funcs, with dynamic an option
- let user specify type for a var, so can know what type
= if any thing to that var ...
- can't use type inference since don't know what the methods or functions are going to return

*/

mod lexer;

mod parser;
mod walker;
mod conf;

use std::path::Path;

use lexer::*;
use parser::*;

pub use parser::Record;
pub use parser::Value;

pub use lexer::Loc;
pub use walker::*;
// pub use error::*;
pub use parser::error::ParseError;
pub use conf::*;

// use self::parser::error::ParseError;
pub use walker::error::WalkerErrors;


pub fn parse(src : &str, keep_src : bool, path : Option<&Path>) -> 
// Result<Vec<Record>,String> 
    Result<Conf,
    ParseError
    // Box<dyn std::error::Error>
    > 

{
  
    let cs=src.chars();
    let mut lexer = Lexer::new(cs.clone());

    match parse_main(&mut lexer) {
        Ok(records) => {
            Ok(Conf::new(records,if keep_src{Some(src)}else{None},path))
        }
        Err((loc,msg)) => {
            Err(ParseError {msg:msg.to_string(),loc})
        }
    }
}


pub fn parse_file<P: AsRef<std::path::Path>>(p : P, keep_src : bool) -> Result<Conf,Box<dyn std::error::Error>> {
    let src = std::fs::read_to_string(p.as_ref())?;
    let result = parse(src.as_str(),keep_src,Some(p.as_ref()))?;
    // error_line_src(src, e.loc)
    Ok(result)
}

pub fn parse_file_simple<P: AsRef<std::path::Path>>(p : P) -> Option<Conf> {
    match std::fs::read_to_string(p.as_ref()) {
        Ok(src)=>{
            match parse(src.as_str(),true,Some(p.as_ref())) {
                Ok(conf)=>{
                    Some(conf)
                }
                Err(e)=>{
                    let line_src = error_line_src(&src,e.loc);
                    // let p :std::path::PathBuf = p.as_ref();
                    println!("{:?}\n{}\n{}",p.as_ref(),e,line_src);
                    None
                }
            }
        }
        Err(e)=>{
            println!("{}",e);
            None
        }
    }
}

// pub fn parse_file_and_content<P: AsRef<std::path::Path>>(p : P) -> Result<(ConfLang,String),Box<dyn std::error::Error>> {
//     let contents = std::fs::read_to_string(p.as_ref())?;
//     let result = parse(contents.as_str(),Some(p.as_ref()))?;
//     Ok((result,contents))
// }

// pub fn parse_file<P: AsRef<std::path::Path>>(file_path: P) -> 
// //Result<Vec<Record>,String>  

// Result<Vec<Record>,Box<dyn std::error::Error>> 
// //std::result::Result<(), Box<dyn std::error::Error>> 
// {
//     // let file_path = file_path.as_ref();

    
//     // let file_path_str = file_path.clone().to_str().unwrap();
//     let src = std::fs::read_to_string(file_path)?;

//     let result = parse(&src);

//     match result {
//         Ok(records) => {
//             Ok(records)
//         },
//         Err(e) => {
//             Err(Box::new (e))
//         }
//     }
    

//     // let file = std::fs::File::open(&file_path)?;
//     // let mut reader = std::io::BufReader::new(file);

// }


pub fn print_tree(records : &Vec<Record>) {
    // println!("printing tree {:?}",records);

    // let mut stk : Vec<(usize,&Record)> = records.iter().rev().map(|x|(0,x)).collect();
    let mut stk = if records.len()>0 { vec![0 as usize]}else{ vec![]};

    while let Some(cur)=stk.pop() {
        // let (depth,record) = stk.pop().unwrap();

        let record=&records[cur];
        println!("r {:?}",record);

        let mut depth = 0;
        {
            let mut p = record.parent;
            while let Some(mut pp)=p {
                depth+=1;
                p=records[pp].parent;
            }

        }

        let val_strs : Vec<String> = record.values.iter().map(|x|x.extracted.clone()).collect();
        let combined_val_strs = val_strs.join(", ");
        println!("{}{:}","   ".repeat(depth),combined_val_strs);

        for c in record.children.iter().rev() {
            // stk.push((depth+1,c));
            stk.push(*c);
            // println!("push {}",*c);
        }
    }
}

pub fn calc_loc_from_sub_row_col(src : &str, val_start_loc : Loc, val_end_loc : Loc, sub_row : usize, sub_col : usize) -> Loc {


    let src=src.get(val_start_loc.pos .. val_end_loc.pos).unwrap();

    let mut sub_row2=0;
    let mut pos2=val_start_loc.pos;
    let mut line_start_pos2 = val_start_loc.line_start_pos;

    for c in src.chars() {
        if c == '\n' {
            if sub_row2 == sub_row {
                pos2 = line_start_pos2 + sub_col;
                break;
            }

            sub_row2+=1;
            line_start_pos2=pos2+1;
        }

        pos2+=1;
    }

    Loc {
        pos : pos2,
        row : val_start_loc.row+sub_row,
        col : val_start_loc.col+sub_col,
        line_start_pos : line_start_pos2,
    }
}

pub fn error_line_src(src : &str, loc : Loc) -> String //(String,String) 
{
    if src.is_empty() {
        return String::new();
    }
    
    let a = loc.line_start_pos as usize;
    let mut b = a;
    let mut spaces = String::new();

    for c in src.get(a..).unwrap().chars() {

        if c=='\r' || c=='\n' {
            break;                
        } 
        
        if b < loc.pos as usize {
            if c=='\t' {
                spaces.push('\t');
            } else {
                spaces.push(' ');
            }
        } else {
            // break;
        }

        b+=1;
    }
    
    // format!("\"{}\"\n{} ^",src.get(a..b).unwrap(),spaces)
    format!("{}\n{}^",src.get(a..b).unwrap(),spaces)
    // (src.get(a..b).unwrap().to_string(),spaces)
}
