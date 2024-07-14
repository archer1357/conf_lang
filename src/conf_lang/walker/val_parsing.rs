use std::{any::{TypeId, Any}, str::FromStr};


#[derive (Debug)]
pub struct ParsedVal(pub Box<dyn std::any::Any>);

impl ParsedVal {
    pub fn get<T:Copy+'static>(&self) -> Option<T> {
        if let Some(x)=self.0.downcast_ref::<T>() {
            Some(*x)
        } else {
            None
        }
    }
}


// #[derive (Clone)] //Debug
pub trait Parseable : std::fmt::Debug {//+ParseableClone
    fn parse(&self, s : &String) -> Result<ParsedVal,Box<dyn std::error::Error>>;
    fn my_box_clone(&self) -> Box<dyn Parseable>;
}


#[derive (Clone,Debug)]
pub struct ParserType<T>(std::marker::PhantomData<T>);

impl<T> ParserType<T> {
    pub fn new() -> Self {
        Self(std::marker::PhantomData::<T>)
    }
}

impl<T> Parseable for ParserType<T> 
where
    T : FromStr + 'static + std::fmt::Debug+Clone,
    <T as FromStr>::Err : std::error::Error
{
    fn parse(&self, s : &String) -> Result<ParsedVal,Box<dyn std::error::Error>> {
        Ok(ParsedVal(Box::new(s.parse::<T>()?)))
    }
    
    fn my_box_clone(&self) -> Box<dyn Parseable> {
        Box::new(self.clone())
    }
}


pub trait Funcable { //:FuncableClone std::fmt::Debug 
    fn func(&self, s : &String) -> Option<ParsedVal>;
    fn my_box_clone(&self) -> Box<dyn Funcable>;
}

#[derive (Clone)]//Debug
pub struct FuncerType<T:Clone>(pub fn(&str)->Option<T>) 
// where
//     F:Fn(&str)->Option<T>
    ;

impl<T> Funcable for FuncerType<T> 
where
    // F:Fn(&str)->Option<T>,
    T : 'static+Clone,
{
    fn func(&self, s : &String) -> Option<ParsedVal> {
        self.0(s).and_then(|x|Some(ParsedVal(Box::new(x))))
    }

    fn my_box_clone(&self) -> Box<dyn Funcable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Parseable> {
    fn clone(&self) -> Self {
        self.my_box_clone()
    }
}

impl Clone for Box<dyn Funcable> {
    fn clone(&self) -> Self {
        self.my_box_clone()
    }
}



////////////////

// pub trait ParseableClone {
//     fn clone_foo(&self) -> Box<dyn Parseable>;
// }

// impl<T> ParseableClone for T
// where
//     T: Parseable + Clone + 'static,
// {
//     fn clone_foo(&self) -> Box<dyn Parseable> {
//         Box::new(self.clone())
//     }
// }
// pub trait FuncableClone {
//     fn clone_foo(&self) -> Box<dyn Funcable>;
// }

// impl<T: Funcable + Clone + 'static> FuncableClone for T {
//     fn clone_foo(&self) -> Box<dyn Funcable> {
//         Box::new(self.clone())
//     }
// }
