
/*

    match curly braces ie count opened/closed

have tripple double quotes?



todo: add {} strings, where newline after { is ignored, indents+4 on the next lines are ignored and indents+4 + newline ignored on }
eg

some
    thing {
        hello
        world
            there
    }
=> "hello\nworld\n    there"

*/

/*

bugs
* when parsing {"\{"} it complains about missing closing brace

*/

pub mod error;

use std::f32::consts::E;

use super::lexer::*;

use std::ops::RangeBounds;


#[derive(Default, Debug)]
pub struct Record {
    // pub parent : Option<&'a Record<'a>>,
    pub values : Vec<Value>,
    // pub children : Vec<Record>,
    pub parent : Option<usize>,
    pub children : Vec<usize>,
}

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd)]
pub enum ValueType {
    None,
    Quote,
    DoubleQuote,
    CurlyBrace,
    BackQuote,
}

#[derive(Debug,Clone)]
pub struct Value {
    pub start_loc : Loc,
    pub end_loc : Loc,
    pub extracted : String,
    pub value_type : ValueType,
}

fn token_to_value(token : Token, value_type : ValueType) -> Value {
    Value {start_loc:token.start_loc, end_loc:token.end_loc, extracted:token.extracted, value_type}
}

fn parse_cmnt(lexer : &mut Lexer) -> bool {
    //[#][^\n]*

    if let Some('#') = lexer.getc(0) {
        lexer.skip(1);
        
        while let Some(c) = lexer.getc(0) {
            if c=='\n' {
                break;
            }

            lexer.skip(1);
        }

        return true;        
    }

    false
}





fn parse_bval(lexer : &mut Lexer) -> Result<bool,(Loc,&'static str)> {
    // [{] (besc|bbody)* [}]
    // [^}]

    
    if let Some('{') = lexer.getc(0) {
        let mut brace_count = 0;

        lexer.push();
        
        let start_loc = lexer.loc();
        lexer.skip(1);
        lexer.consume(0, None); //makes the token start location here, just after the open brace?


        //[{][\s\t]* eol => skip
        //eol [\s\t]*[}] => skip


        let mut i = 0;

        //
        loop {
            if lexer.has(i,[" ", "\t"]).is_some() {
                i+=1;
                continue;
            }

            if let Some(x) = lexer.has(i,["\n", "\r\n"]) {
                lexer.skip(i+x.chars().count());
            }
            
            break;
        }

        //
        let mut take_ind = 0;
        let mut taking = false;
        i=0;

        while let Some(c) = lexer.getc(i) {
            match c {
                '{' => {
                    brace_count+=1;
                    taking=true;
                    i+=1;
                    take_ind=i;
                }
                '}' if brace_count > 0 => {
                    brace_count-=1;
                    taking=true;
                    i+=1;
                    take_ind=i;
                }
                '}' if brace_count==0 => { //closing brace
                    break;
                }
                '\n' => {
                    take_ind=i;
                    taking=false;
                    i+=1;
                }
                '\r' if lexer.getc(i+1)==Some('\n') => {
                    take_ind=i;
                    taking=false;
                    i+=2;
                }
                ' '|'\t' => {
                    i+=1;

                    if taking {
                        take_ind=i;
                    }
                }
                _=> {
                    taking=true;
                    i+=1;
                    take_ind=i;
                }
            }
        }

        //

        if let Some('}') = lexer.getc(i) {
            if brace_count == 0 {
                lexer.consume(take_ind,None);
                lexer.skip(i-take_ind);

                let end_loc=lexer.loc();
                lexer.skip(1);
                lexer.pop_keep();


                let mut token = lexer.token().unwrap();

                lexer.set_token(token);
        
                return Ok(true);
            } else {
                // lexer.add_error(start_loc,"braces must match");
                lexer.pop_discard();
                return Err((lexer.loc(),"braces must match"));

            }
        } else {
            // lexer.add_error(start_loc,"expecting closing brace");
                lexer.pop_discard();
                return Err((lexer.loc(),"expecting closing brace"));
        }

        //
        // lexer.pop_discard();
    } else {
        return Ok(false);
    }


    // false
}



fn parse_bqval(lexer : &mut Lexer) -> Result<bool,(Loc,&'static str)> {
    // [{] (besc|bbody)* [}]
    // [^}]

    // println!("hmm {:?}",lexer.has(0, ["```","`",]));
    if let Some(q)=lexer.has(0, ["```","`"]) {
        // println!("has {q}");

        lexer.push();
        
        let quote_start_loc = lexer.loc();
        lexer.skip(q.chars().count());
        // lexer.consume(0, None); //makes the token start location here, just after the open brace?


        //[{][\s\t]* eol => skip
        //eol [\s\t]*[}] => skip


        let mut i = 0;

        while let Some(x)=lexer.has(i,[" ", "\t"]) {
            i+=1;
        }

        if let Some(x)=lexer.has(i,["\r\n", "\n"]) {
            lexer.skip(i+x.chars().count());
        }

        

        //
        let mut take_ind = 0;
        let mut taking = false;
        i=0;

        while let Some(c) = lexer.getc(i) {
            match c {
                '\n' => {
                    take_ind=i;
                    taking=false;
                    i+=1;
                }
                '\r' if lexer.getc(i+1)==Some('\n') => {
                    take_ind=i;
                    taking=false;
                    i+=2;
                }
                ' '|'\t' => {
                    i+=1;

                    if taking {
                        take_ind=i;
                    }
                }
                _=> {
                    if lexer.has(i, [q]).is_some() {
                        break;
                    }

                    taking=true;
                    i+=1;
                    take_ind=i;
                }
            }
        }

        //

        if lexer.has(i, [q]).is_some() {

            lexer.consume(take_ind,None);
            lexer.skip(i-take_ind);

            let end_loc=lexer.loc();
            lexer.skip(q.chars().count());
            lexer.pop_keep();


            let mut token = lexer.token().unwrap();

            lexer.set_token(token);
    
            return Ok(true);

        } else {
            if q.chars().count()==1 {
                // lexer.add_error(start_loc,"expecting closing back quote");
                lexer.pop_discard();
                return Err((lexer.loc(),"expecting closing back quote"));
                // println!("xxx");
            } else {
                // lexer.add_error(start_loc,"expecting closing tripple back quotes");
                lexer.pop_discard();
                return Err((lexer.loc(),"expecting closing back quotes"));
                // println!("yyy");
            }
        }

        //
        // lexer.pop_discard();
    } else {
        return Ok(false);
    }


    // false
}

fn parse_dval_char(lexer : &mut Lexer) -> bool {
    // [^']

    if let Some(c) = lexer.getc(0) {
        if c!='"' {
            lexer.consume(1, None);
            return true;
        }
    }

    false
}


fn parse_desc(lexer : &mut Lexer) -> bool {
    // [\\](\r\n|\n|[strn]|eof|.)

    if let Some('\\') = lexer.getc(0) {
        if let Some(c) = lexer.getc(1) {
            match c {
                '\n' => {
                    lexer.consume(2, Some("")); //should replace with space?
                },
                '\r' => {
                    if let Some('\n') = lexer.getc(2) {
                        lexer.consume(3, Some(""));  //should replace with space?
                    } else {
                        return false;
                    }
                },
                's' => {
                    lexer.consume(2, Some(" "));
                },
                't' => {
                    lexer.consume(2, Some("\t"));
                },
                'r' => {
                    lexer.consume(2, Some("\r"));
                },
                'n' => {
                    lexer.consume(2, Some("\n"));
                },
                _ => {
                    lexer.consume(2, Some(c.to_string().as_str()));
                }
            }
        } else {
            lexer.consume(1, None);
        }

        return true;
    }

    false
}


fn parse_dval(lexer : &mut Lexer) -> Result<bool,(Loc,&'static str)> {
    // ["] (desc|dbody)* ["]

    
    if let Some('"') = lexer.getc(0) {
        lexer.push();

        let start_loc = lexer.loc();
        lexer.skip(1);

        lexer.consume(0, None);
        
        while parse_desc(lexer) || parse_dval_char(lexer) {
        }

        if let Some('"') = lexer.getc(0) {
            lexer.skip(1);
            lexer.pop_keep();
            return Ok(true);
            
        } else {
            // lexer.add_error(start_loc,"expecting closing double quote after");
            lexer.pop_discard();
            return Err((lexer.loc(),"expecting closing double quote"));
        }

        // lexer.pop_keep();
        // lexer.pop_discard();
        // for e in lexer.get_errors().iter() {
        //     println!("{e:?}");
        // }
    } else {
        Ok(false)
    }

}

fn parse_qval_char(lexer : &mut Lexer) -> bool {
    // [^']

    if let Some(c) = lexer.getc(0) {
        if c!='\'' {
            lexer.consume(1, None);
            return true;
        }
    }

    false
}

fn parse_qval(lexer : &mut Lexer) -> Result<bool,(Loc,&'static str)> {
    // ['] (qbody)* [']

    
    if let Some('\'') = lexer.getc(0) {
        lexer.push();

        let start_loc = lexer.loc();
        lexer.skip(1);

        lexer.consume(0, None);
        
        while parse_qval_char(lexer) {
        }

        if let Some('\'') = lexer.getc(0) {
            lexer.skip(1);
            lexer.pop_keep();
            return Ok(true);
        } else {
            // lexer.add_error(start_loc,"expecting closing quote after");
            
            lexer.pop_discard();
            return Err((lexer.loc(),"expecting closing quote"));
        }

        // lexer.pop_discard();
    } else{
        return Ok(false);
    }

    // false
}

fn parse_sval_charn(lexer : &mut Lexer) -> bool {
    // [^\s\t\r\n]

    if let Some(c) = lexer.getc(0) {
        match c {
            ' '|'\t'|'\r'|'\n' => {},
            _ => {
                lexer.consume(1, None);
                return true;
            }
        }
    }

    false
}

fn parse_sval_char0(lexer : &mut Lexer) -> bool {
    // [^\s\t\r\n'\"#]

    if let Some(c) = lexer.getc(0) {
        match c {
            ' '|'\t'|'\r'|'\n'|'{'|'\''|'"'|'#' => {
                // lexer.add_error(lexer.loc(), "invalid char for val");
            },
            _ => {
                lexer.consume(1, None);
                return  true;
            }
        }
    }

    false
}


fn parse_sesc(lexer : &mut Lexer) -> bool {
    // [\\](\r\n|[\s\t\n]|eof)

    //todo?: at start allow escapes for ' " {

    if let Some('\\') = lexer.getc(0) {
        if let Some(c) = lexer.getc(1) {
            match c {
                '\n' => { //eol
                    lexer.consume(2, Some("")); //should replace with space?
                },
                '\r' => { //eol
                    if let Some('\n') = lexer.getc(2) {
                        lexer.consume(3, Some(""));  //should replace with space?
                    } else {
                        return false;
                    }
                },
                ' ' | '\t' => { //spc
                    lexer.consume(2, Some(c.to_string().as_str()));
                },
                _ => {
                    lexer.consume(2, None);
                }
            }
        } else { //eof
            lexer.consume(1, None);
        }

        return true;
    }

    false
}

fn parse_sval (lexer : &mut Lexer) -> Result<bool,(Loc,&'static str)> {
    // (escaped | shead) (escaped | sbody)*

    if parse_sesc(lexer) || parse_sval_char0(lexer) {
        while parse_sesc(lexer) || parse_sval_charn(lexer) {
        }
  
        Ok(true)
    } else {
        Ok(false)
    }

//    false
}

fn parse_val (lexer : &mut Lexer) -> Result<Option<ValueType>,(Loc,&'static str)> {
    //qval | dval | bval |sval

    if parse_qval(lexer)? {
        return Ok(Some(ValueType::Quote));
    } 
    
    if parse_dval(lexer)? {
        return Ok(Some(ValueType::DoubleQuote))
    } 
    // else if lexer.has_errors() {
    //     return None;
    // }

    if parse_bval(lexer)? {
        return Ok(Some(ValueType::CurlyBrace));
    } 
    // else if lexer.has_errors() {
    //     return None;
    // }

    if parse_bqval(lexer)? {
        return Ok(Some(ValueType::BackQuote));
    } 
    // else if lexer.has_errors() {
    //     return None;
    // }
    
    if parse_sval(lexer)? {
        return Ok(Some(ValueType::None));
    }
 
    //lexer.add_error(lexer.loc(), "val");
    return Ok(None);
}

fn parse_empty_lines(lexer : &mut Lexer) {
    //([\s\t]*([\n]|[\r][\n]))*
    //

    while !lexer.is_end() {
        let mut i =0;

        while lexer.has(i, [" ","\t"]).is_some() {
            i+=1;
        }
        
        if lexer.has(i, ["#"]).is_some() {
            //instead of using this, should really modify the grammar in parse_main?
            while !lexer.is_end() {
                if let Some(x)=lexer.has(i, ["\r\n","\n"]) {
                    i+=x.chars().count();
                    break;
                } else {
                    i+=1;
                }
            }

            lexer.skip(i);
        } else if let Some(x)=lexer.has(i, ["\r\n","\n"]) {
            lexer.skip(i+x.chars().count());
        } else {
            break;
        }
    }
}

fn parse_spcs(lexer : &mut Lexer) -> bool {
    //([ \t]|\\(\n|\r\n))+

    let mut found = false;

    while let Some(c) = lexer.getc(0) {
        if c==' ' || c=='\t' {
            lexer.skip(1);
            found = true;
        } else if c=='\\' {
            if lexer.getc(1)==Some('\n') {
                lexer.skip(2);
                found = true;
            } else if lexer.get(1,2)==Some("\r\n") {
                lexer.skip(3);
                found = true;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    found

    // if found {
    //     true
    // } else {
    //     //lexer.add_error(lexer.loc(), "space");
    //     false
    // }
}

//, indent_out : &mut usize
fn parse_indent(lexer : &mut Lexer, has_a_record : bool, last_indent : usize) -> Result<Option<usize>,(Loc,&'static str)> {
    //[    ]*

    while let Some("    ") = lexer.get(0,4) {
        lexer.consume(4, Some(" "));
    }
    
    // if lexer.has(0, [" ","\t"]).is_some() {

    // }
    
    // while let Some(c) = lexer.getc(0) {
    //     if c==' ' || c=='\t' {
    // }

    if let Some(c) = lexer.getc(0) {
        // while 
        if c==' ' || c=='\t' {
            
            // lexer.add_error(lexer.loc(), "indents must be spaces, multiples of four");

            return Err((lexer.loc(),"indents must be four spaces"));
        }
    }

    //generate 0 length token str for when no indent
    lexer.consume(0, None);

    //
    let indent = lexer.token().unwrap().extracted.chars().count();

    if (!has_a_record && indent > 0) || indent > last_indent+1 {
        // lexer.add_error(lexer.loc(), "too many indents");
        return Err((lexer.loc(),"too many indents"));
    }

    //
    // *indent_out = indent;
    // true
    Ok(Some(indent))
}

fn parse_eol(lexer : &mut Lexer) -> bool {
    // [\n]|[\r][\n]

    if let Some('\n') = lexer.getc(0) {
        lexer.skip(1);
        return true;
    }

    if let Some("\r\n") = lexer.get(0,2) {
        lexer.skip(2);
        return true;
    }
    
    // lexer.add_error(lexer.loc(), "expecting eol");
    false
}

fn parse_vals(lexer : &mut Lexer) -> Result<Option<Vec<Value>>,(Loc,&'static str)> {
   // val (spc val)*

    if let Some(value_type)=parse_val(lexer)? {
        let mut values = Vec::new();
        values.push(token_to_value(lexer.token().unwrap(),value_type));
       
        loop {
            lexer.push();

            if parse_spcs(lexer) {
                if let Some(value_type)=parse_val(lexer)? {
                    lexer.pop_keep();                
                    values.push(token_to_value(lexer.token().unwrap(),value_type));
                    continue;
                }
            }

            lexer.pop_discard();
            break;
        }

        Ok(Some(values))
    } else {
        Ok(None)
    }
}


pub fn parse_main(lexer : &mut Lexer) -> Result<Vec<Record>,(Loc,&'static str)> { 
    //( ( spcs | spcs? cmnt  | record (spcs|spcs cmnt)? ) (eol|eof) )*
    //( ( spcs | spcs? cmnt  | indent vals (spcs|spcs cmnt)? ) (eol|eof) )*


    let mut last_indent = 0;
    let mut has_a_record = false; //used to make sure atleast one record declared, for indents
    let mut records=vec![Record{values:Vec::new(),parent:None,children:Vec::new()}];
    let mut cur_parent = Some(0);

    loop {
        // println!("main");
        parse_empty_lines(lexer);

        //record
        lexer.push();

        if let Some(indent) = parse_indent(lexer, has_a_record, last_indent)? {
            if let Some(values)=parse_vals(lexer)? {
                if indent > last_indent {
                    cur_parent = Some(records.len()-1);
                } else if indent < last_indent {
                    //pop top of stk, last_indent-indent times
                    for i in indent .. last_indent {
                        cur_parent = records[cur_parent.unwrap()].parent;
                    }
                }

                //
                if let Some(parent) = cur_parent {
                    let cur_record = records.len();
                    records[parent].children.push(cur_record);
                }

                records.push(Record { values, parent:cur_parent, children:Vec::new(), });


                //
                has_a_record = true;
                last_indent = indent;

                //
                if parse_spcs(lexer) {
                    parse_cmnt(lexer);
                }
                
                if parse_eol(lexer) {
                    lexer.pop_keep();
                    continue;
                } else if lexer.is_end() {
                    lexer.pop_keep();
                    break;
                } else {
                    // lexer.add_error(lexer.loc(), "expecting eol or eof");
                    // lexer.add_error(lexer.loc(), "unexpected symbol");
                    break;
                }

            }
        }

        lexer.pop_discard();
        
        //white spaces, comment, newlines
        lexer.push();
        parse_spcs(lexer);
        parse_cmnt(lexer);
        
        if parse_eol(lexer) {
            lexer.pop_keep();
            continue;
        } else if lexer.is_end() {
            lexer.pop_keep();
            break;
        } else {
            lexer.pop_discard();
            break;
        }
    }

    if lexer.is_end() {
        // Some(stk.into_iter().next().unwrap())
        Ok(records)
    } else {
        Err((lexer.loc(),"unknown error"))
    }
}


