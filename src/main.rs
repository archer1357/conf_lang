
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]


use std::path::Path;

use conf_lang::{error_line_src, parse_walker_param, Walker};


mod conf_lang;

fn main() {
  
    println!("Hello, world!\n");

    let path = Path::new("data/test.conf");
    let src = std::fs::read_to_string(path).unwrap();

    // let path2 = Path::new("data/test2.conf");
    // let src2 = std::fs::read_to_string(path2).unwrap();
    let mut walker=Walker::new();

    walker.branch("root")
        .tags(["hello"])
            .entry()
                .str()
                .str()
                .children("hello_children")
                .label("hello")
    .branch("hello_children")
        .tags(["world"])
            .entry()
                .named_param("int")
                .repeat()
                .label("world")
    ;

    let conf=conf_lang::parse(src.as_str(), true,Some(path));

    match conf {
        Ok(conf)=> {
            let res=walker.apply_ext::<()>("root", conf.root(), |n,v|match n{
                "int" => parse_walker_param::<i32>(v),
                _ => None,
            }, |record,on_enter|{

                let vals=(0..record.val_num()).map(|x|format!("{:?}",record.get_val(x).unwrap().extracted.as_str())).collect::<Vec<_>>().join(", ");
                let indent="   ".repeat(record.depth());
                let branch=record.branch();
                let node=record.label();
                let order=record.order();

                println!("{}: {indent}{order}: {vals:} @({branch:?}:{node:?})",
                    if on_enter {"enter"}else{"exit "}
                );

                Ok(None) //can return Some(node) to insert nodes from other confs into the walk
            });

            if let Err(e)=res {
                println!("");
                e.eprint_with_src(|_|Some(src.as_str()));
            }

        }
        Err(e) => {
            let line_src = error_line_src(&src,e.loc);
            println!("{:?}\n{}\n{}",path.as_os_str(),e,line_src);
        }
    }

}
