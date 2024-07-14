
use std::{collections::{HashMap, HashSet}, default, str::FromStr, any::{Any, TypeId}, path::{Path, PathBuf}, fmt::Debug};

use self::val_parsing::*;

use super::{RecordContainer, Conf};

use error::*;

pub mod error;
mod val_parsing;
// mod val_tuple;
mod container;


// pub use val_tuple::*;
pub use container::*;

pub fn to_box_any<T:Any+'static>(t:T)->Box<dyn Any+'static> {
    Box::new(t)
}
pub fn parse_walker_param<T:FromStr+'static>(s:&str)->Option<Box<dyn Any+'static>> {
    T::from_str(s).ok().map(to_box_any)
}


#[derive (Debug,Clone)] //Debug
enum WalkerVal {
    Str,
    // Parse(Box<dyn Parseable>),
    // Func(Box<dyn Funcable>),
    Named(String),


}


// pub type WalkerCallback<L> = for<'a> fn (&L,RecordContainer); // -> Result<Option<(PathBuf,RecordContainer<'a>)>,(Loc,E)>

#[derive (Clone)]
struct WalkerNode {
    vals : Vec<WalkerVal>,
    has_tag : bool,
    repeat : bool,
    node_label : Option<String>,
    children : Option<String>,
    ignore_children : bool,
    branch_ind:usize,
}

impl<> WalkerNode {
    pub fn new(has_tag : bool,branch_ind:usize) -> Self {
        Self {
            vals : Vec::new(),
            has_tag,
            repeat:false,
            node_label : None,
            children : None,
            ignore_children:false,
            branch_ind,
        }
    }
}

// enum WalkerBranchNode<B,N> {
//     Node(WalkerNode<B,N>),
//     FromBranch(B),
// }
struct WalkerBranch {
    tags : HashMap<String,Vec<usize>>, //[tag][tag_node_ind]=node_index
    non_tags : Vec<usize>, //[no_tag_node_ind]=node_index
    branch_inserts : Vec<String>,
    ignore_other_errors : bool,
    branch_label : String,
    tag_onces : HashMap<String,HashSet<String>>,
    // tag_haves : HashMap<String,HashSet<String>>,
}

impl WalkerBranch {
    pub fn new(branch_label : String) -> Self {
        Self {
            tags : HashMap::new(),
            non_tags : Vec::new(),
            branch_inserts : Vec::new(),
            ignore_other_errors : false,
            branch_label,
            tag_onces : HashMap::new(),
            // tag_haves : HashMap::new(),
        }
    }
}

pub struct Walker {
    branches : Vec<WalkerBranch>,
    branch_map : HashMap<String,usize>,
    nodes : Vec<WalkerNode>,
    for_tag_names : Vec<String>,
    cur_nodes_start : usize,
    // node_defines : HashMap<String,usize>,
}

impl Walker {
    pub fn new() -> Self {
        Self {
            branches : Vec::new(),
            branch_map : HashMap::new(),
            nodes : Vec::new(),
            for_tag_names : Vec::new(),
            cur_nodes_start : 0,
            // node_defines : HashMap::new(),
        }
    }
    
    fn get_branch(&self, branch_label : &str) -> &WalkerBranch {
        let branch_index = self.branch_map.get(branch_label);

        if branch_index.is_none() {
            panic!("ConfLang, branch not found! {:?}", branch_label);
        }

        &self.branches[*branch_index.unwrap()]
    }

    fn cur_branch_mut(&mut self) -> &mut WalkerBranch {
        if self.branches.len()==0 {
            panic!("ConfLang, no branch available!");
        }

        self.branches.last_mut().unwrap()
    }


    pub fn branch(&mut self, branch_label : &str) -> &mut Self {
        if self.branch_map.contains_key(branch_label) {
            panic!("Branch label already used.")
        }

        self.cur_nodes_start = self.nodes.len();
        let branch_index = self.branches.len();
        self.branches.push(WalkerBranch::new(branch_label.to_string()));
        self.branch_map.insert(branch_label.to_string(), branch_index);
        self
    }

    pub fn branch_nodes_from(&mut self, branch_label : &str) -> &mut Self {        
        let cur_branch = self.cur_branch_mut();
        cur_branch.branch_inserts.push(branch_label.to_string());
        self.cur_nodes_start = self.nodes.len();
        self
    }

    //todo add quoted,unquoted vals, etc

    pub fn skip_node_errs(&mut self) -> &mut Self {
        let cur_branch = self.cur_branch_mut();
        //skip errors on nontags and unknown tags
        cur_branch.ignore_other_errors = true;
        self
    }

    pub fn skip_child_errs(&mut self) -> &mut Self {
        for node_index in self.cur_nodes_start .. self.nodes.len() {
            self.nodes[node_index].ignore_children=true;
        }

        self
    }

    // pub fn def(&mut self, name : &str) -> &mut Self {
    //     if self.node_defines.contains_key(name) {
    //         panic!("conflang walker, define node key exists");
    //     }

    //     let node_index = self.nodes.len();
    //     self.cur_nodes_start = node_index;
    //     self.node_defines.insert(name.to_string(),node_index);
    //     self.nodes.push(WalkerNode::new(false));

    //     // self.nodes[node_index].node_label=name.to_string();

    //     self
    // }

    // pub fn use_def(&mut self, name : &str) -> &mut Self {
    //     if let Some(&def_node_index) = self.node_defines.get(name) {
    //         if def_node_index==self.cur_nodes_start {
    //             panic!("conflang walker, can't append def to itself");
    //         }

    //         let def_node = self.nodes[def_node_index].clone();
    //         // let def_vals=self.nodes[def_node_index].vals.clone();
    //         // let def_label = self.nodes[def_node_index].node_label.clone();
    //         // let def_repeat =;
    //         // self.


    //         // 
                
    //         for node_index in self.cur_nodes_start .. self.nodes.len() {
    //             self.nodes[node_index].vals.extend(def_node.vals.clone());

    //             if def_node.node_label.is_some() {
    //                 self.nodes[node_index].node_label=def_node.node_label.clone();
    //             }

    //             if !self.nodes[node_index].repeat {
    //                 self.nodes[node_index].repeat=def_node.repeat;
    //             }

    //             if !self.nodes[node_index].ignore_children {
    //                 self.nodes[node_index].ignore_children=def_node.ignore_children;
    //             }

    //             if self.nodes[node_index].children.is_none() {
    //                 self.nodes[node_index].children=def_node.children.clone();
    //             }
    //         }



            

    //     } else {
    //         panic!("conflang walker, define node hasn't been declared");
    //     }

    //     self
    // }

    // pub fn node_tagless(&mut self, ) -> &mut Self {
    //     let node_index = self.nodes.len();
    //     self.nodes.push(WalkerNode::new(false));
    //     let cur_branch = self.cur_branch_mut();
    //     cur_branch.non_tags.push(node_index);
    //     self.cur_nodes_start = node_index;
    //     self
    // }

    // pub fn node_tag(&mut self, tag_name: &str) -> &mut Self {
    //     let node_index = self.nodes.len();
    //     self.cur_nodes_start = node_index;

    //     self.nodes.push(WalkerNode::new(true));
    //     // self.nodes[node_index].node_label=tag_name.to_string();

    //     let cur_branch = self.cur_branch_mut();
    //     let tag_nodes=cur_branch.tags.entry(tag_name.into()).or_insert_with(Default::default);
    //     tag_nodes.push(node_index);

    //     self
    // }

    // pub fn for_tags<const N:usize>(&mut self, tag_names: [&'static str;N]) -> &mut Self {
    //     self.for_tag_names.clear();
    //     self.for_tag_names.extend(tag_names.map(|x|x.to_string()));
    //     self
    // }
    
    // pub fn for_tags<'t,T:AsRef<[&'t str]>>(&mut self, tag_names: T) -> &mut Self {
    //     self.for_tag_names.clear();
    //     self.for_tag_names.extend(tag_names.as_ref().iter().map(|x|x.to_string()));
    //     self
    // }

    pub fn tagless(&mut self, ) -> &mut Self {
        self.cur_nodes_start = self.nodes.len(); //makes modifying a node an error if each hasnt been called?
        self.for_tag_names.clear();
        self
    }
    pub fn tags<'t,T:IntoIterator<Item = &'t str>>(&mut self, tag_names: T) -> &mut Self {
        self.cur_nodes_start = self.nodes.len(); //makes modifying a node an error if each hasnt been called?
        self.for_tag_names.clear();
        
        self.for_tag_names.extend(tag_names.into_iter().map(|x|x.to_string()));

        self
    }
    
    pub fn entry(&mut self) -> &mut Self {
        let branch_ind=self.branches.len()-1;
        let cur_branch = self.branches.last_mut().unwrap();

        if self.for_tag_names.len()==0 {
            let node_index = self.nodes.len();
            self.nodes.push(WalkerNode::new(false,branch_ind));
            
            cur_branch.non_tags.push(node_index);
            self.cur_nodes_start = node_index;
        } else {
            self.cur_nodes_start = self.nodes.len();

            for tag_name in self.for_tag_names.iter() {
                let node_index = self.nodes.len();
                self.nodes.push(WalkerNode::new(true,branch_ind));

                let tag_nodes=cur_branch.tags.entry(tag_name.clone()).or_insert_with(Default::default);
                tag_nodes.push(node_index);
            }
        }



        self
    }

    pub fn repeat(&mut self) -> &mut Self {
        for node_index in self.cur_nodes_start .. self.nodes.len() {
            self.nodes[node_index].repeat = true;
        }
        
        self
    }

    pub fn label(&mut self, node_label : &str) -> &mut Self {
        for node_index in self.cur_nodes_start .. self.nodes.len() {
            self.nodes[node_index].node_label = Some(node_label.to_string());
        }

        self
    }

    // pub fn label_tag(&mut self) -> Self 
    // {
    //     let cur_node = self.cur_node_mut();
    //     cur_node.
    //     cur_node.label = label;//Some(label.into());
    //     self
    // }


    // pub fn branch_tag_have<'t,T:IntoIterator<Item = &'t str>>(&mut self, tags : T) -> &mut Self {
    //     let cur_branch = self.branches.last_mut().unwrap();
    //     let tags=tags.into_iter().collect::<Vec<_>>();

    //     for &tag in tags.iter() {
    //         let tag_have=cur_branch.tag_haves.entry(tag.to_string()).or_default();

    //         for &tag2 in tags.iter() {
    //             tag_have.insert(tag2.to_string());
    //         }
    //     }

    //     self
    // }

    pub fn branch_tag_once<'t,T:IntoIterator<Item = &'t str>>(&mut self, tags : T) -> &mut Self {
        let cur_branch = self.branches.last_mut().unwrap();
        let tags=tags.into_iter().collect::<Vec<_>>();

        for &tag in tags.iter() {
            let tag_once=cur_branch.tag_onces.entry(tag.to_string()).or_default();

            for &tag2 in tags.iter() {
                tag_once.insert(tag2.to_string());
            }
        }
        
        self
    }

    pub fn children(&mut self, branch_label : &str) -> &mut Self {

        // println!("children {}",branch_label);
        for node_index in self.cur_nodes_start .. self.nodes.len() {
            // println!("try adding child {branch_label} to [{node_index}] {:?}",self.nodes[node_index].vals.first());
            if self.nodes[node_index].children.is_some() {
                // println!("children2 {:?} {}",self.nodes[node_index].children,branch_label);
                panic!("ConfLang Walker, children already set.");
            }
            // println!("adding child {branch_label} to [{node_index}] {:?}",self.nodes[node_index].vals.first());

            self.nodes[node_index].children = Some(branch_label.to_string());
        }

        self
    }

    
    pub fn str(&mut self) -> &mut Self {
        for node_index in self.cur_nodes_start .. self.nodes.len() {
            self.nodes[node_index].vals.push(WalkerVal::Str);
        }

        self
    }

    pub fn named_param(&mut self, name:&str) -> &mut Self {
        for node_index in self.cur_nodes_start .. self.nodes.len() {
            self.nodes[node_index].vals.push(WalkerVal::Named(name.to_string()));
        }

        self
    }

    // pub fn parse<T>(&mut self) -> &mut Self
    // where
    //     T:FromStr + 'static + std::fmt::Debug+Clone,
    //     <T as FromStr>::Err : std::error::Error,
    // {
    //     // for node_index in self.cur_nodes_start .. self.nodes.len() {
    //     //     self.nodes[node_index].vals.push(WalkerVal::Parse(Box::new(ParserType::<T>::new())));
    //     // }
        
    //     self
    // }
    
    // pub fn func<T>(&mut self, f:fn(&str)->Option<T>) -> &mut Self
    // where
    //     // F:fn(&str)->Option<T>,
    //     T : 'static+Clone,
    // {
    //     // for node_index in self.cur_nodes_start .. self.nodes.len() {
    //     //     self.nodes[node_index].vals.push(WalkerVal::Func(Box::new(FuncerType::<T>(f))));
    //     // }

    //     self
    // }
    // pub fn option(&mut self, arr: &[&str]) -> Self {
    //     let cur_node = self.cur_node_mut();
    //     // cur_node.vals.push(WalkerVal::Parse(Box::new(ParserType::<T>::new())));
    //     self
    // }

    pub fn validate(&mut self) -> &mut Self {
        //todo
        self
    }

    pub fn apply_simple<'a>(&self, branch_label : &str, top_record : RecordContainer<'a>,
        // param_parser : impl Fn(&str,&'a str) -> Option<Box<dyn Any>>,
        param_parser : fn(&str,&'a str) -> Option<Box<dyn Any>>,
        mut callback : impl FnMut(WalkerRecordContainer),
    ) -> Result<(),WalkerErrors<()>>
    // where 
    //     C : FnMut(WalkerRecordContainer),
    {
        self.apply_ext(branch_label, top_record, param_parser,|record,enter|{
            if enter {
                callback(record);
            }

            Ok(None)
        })
    }


    pub fn apply_simple_ext<'a>(&self, branch_label : &str, top_record : RecordContainer<'a>, 
        param_parser : impl Fn(&str,&'a str) -> Option<Box<dyn Any>>,
        mut callback : impl FnMut(WalkerRecordContainer,bool),
    ) -> Result<(),WalkerErrors<()>>
    // where 
    //     C : FnMut(WalkerRecordContainer,bool),
    {
        self.apply_ext(branch_label, top_record, param_parser,|record,enter|{
            if enter {
                callback(record,true);
            }

            Ok(None)
        })
    }

    pub fn apply<'a,E>(&self, branch_label : &str, top_record : RecordContainer<'a>, 
        param_parser : impl Fn(&str,&'a str) -> Option<Box<dyn Any>>,
        mut callback : impl for<'w> FnMut(WalkerRecordContainer<'w,'a>) -> Result<Option<RecordContainer<'a>>,(usize,E)>,
    ) -> Result<(),WalkerErrors<E>>
    where 
        // C : for<'w> FnMut(WalkerRecordContainer<'w,'a>) -> Result<Option<RecordContainer<'a>>,(usize,E)>,
        E : 'a+Ord+Clone+Debug,
    {
        self.apply_ext(branch_label, top_record, param_parser,|record,enter|{
            if enter {
                callback(record)
            } else {
                Ok(None)
            }
        })
    }

    pub fn apply_ext<'a,E>(&self, branch_label : &str, top_record : RecordContainer<'a>, 
        param_parser : impl Fn(&str,&'a str) -> Option<Box<dyn Any>>,
        callback : impl for<'w> FnMut(WalkerRecordContainer<'w,'a>,bool) -> Result<Option<RecordContainer<'a>>,(usize,E)>,
    ) -> Result<(),WalkerErrors<E>>
    where 
        // C : for<'w> FnMut(WalkerRecordContainer<'w,'a>,bool) -> Result<Option<RecordContainer<'a>>,(usize,E)>,
        E : 'a+Ord+Clone+Debug,
    {
        let mut expecter = Expecter::<'a,E>::new();
        self.traverse(branch_label,top_record,&mut expecter,param_parser,callback);

        if expecter.has_errors() {
            Err(expecter.get_errors())
        } else {
            Ok(())
        }
    }

    //apply simple is for when there are no error types
    //ext is for callback on both enter/exit and not just enter

    //should return error(s), not just eprint them, maybe return expecter?
    //should also add param to return on the first error?
    
    fn traverse<'a,E,C>(&self, root_branch_label : &str, root_record : RecordContainer<'a>, 
        expecter : &mut Expecter<'a,E>,
        param_parser : impl Fn(&str,&'a str) -> Option<Box<dyn Any>>,
        mut callback : C)
    where
        C : for<'w> FnMut(WalkerRecordContainer<'w,'a>,bool) -> Result<Option<RecordContainer<'a>>,(usize,E)>,
        E : Ord+Clone+Debug,
    {
        //root should be zero, therefore nodes entered by user should be 1?
        // .. currently entered node depths starting at 0 ..
        let depth_start=0;

        
        //
        let mut tags_useds = vec![HashSet::<&str>::new()]; //tags_useds[node_depth] = parent_tags_used
        //
        let mut stk=Vec::<(RecordContainer,String, Option<usize>, usize, Option<usize>,)>::new();
        stk.extend(root_record.child_iter().rev().map(|x|(x,root_branch_label.to_string(), None, depth_start, None)));

        let mut walker_records = Vec::<WalkerRecord<'a>>::new();
        let mut order =0;

        while let Some((
            record,top_branch_label, walker_record_parent_index, depth, 
            exit_walker_record_index,
        )) = stk.pop() {
            tags_useds.resize(depth+1, HashSet::new());

            //on exit
            if let Some(walker_record_index)=exit_walker_record_index {
                
                let walker_record_container = WalkerRecordContainer {
                    walker_records:&walker_records,walker_record_index,
                };

                //
                let callback_result = callback(walker_record_container,false);

                match callback_result {
                    Ok(include) => {
                        //add includes
                        if let Some(r)=include {
                            stk.extend(r.child_iter().rev()
                                .map(|c|(c,top_branch_label.clone(), walker_record_parent_index,depth,None,))); //+1
                        }
                    }
                    Err((val_ind,e)) => {
                        //todo use as predicate, try other nodes if error?
                        //  need option for both?
                        // found_err = true;
                        if val_ind<record.value_count() {
                            expecter.expect(record,val_ind,WalkerErrorType::Custom(e));
                        } else {
                            expecter.expect(record,0,WalkerErrorType::CustomInvalidValInd(val_ind,e));

                        }
                    }
                }

                //
                //walker_records.get(walker_record_index).unwrap().record.value(0)
                //

                continue;
            }

            //on enter
            let record_index = record.record_index();

            let v0=&record.value(0).unwrap().extracted;

            expecter.push();

            let mut ok = false;
            let mut found_err = false;
            let mut last_node = None;

            //using array of branches, as multiple branches can be set as children for a node
            let top_branch = self.get_branch(top_branch_label.as_str());
            
            let mut tag_node_indices = Vec::<usize>::new();
            let mut non_tag_node_indices = Vec::<usize>::new();
            
            let mut tags_onces = HashMap::<&str,HashSet<&str>>::new();
            // let mut tag_haves = HashMap::<&str,HashSet<&str>>::new();

            let mut branch_inserts_stk= vec![top_branch_label.clone()];
            let mut branches_visited = HashSet::<String>::new();

            while let Some(b)=branch_inserts_stk.pop() {
                if branches_visited.contains(b.as_str()) {
                    continue;
                }

                let branch = self.get_branch(b.as_str());

                for (tag,set) in branch.tag_onces.iter() {
                    tags_onces.entry(tag.as_str()).or_default().extend(set.iter().map(|x|x.as_str()));
                }

                //
                if let Some(x)=branch.tags.get(v0) {
                    tag_node_indices.extend(x);
                }
                
                //
                non_tag_node_indices.extend(&branch.non_tags);

                //
                // branch_inserts_stk.extend(&branch.branch_inserts);
                // branch_inserts_stk.extend(branch.branch_inserts.clone());
                branch_inserts_stk.extend_from_slice(branch.branch_inserts.as_slice());
                branches_visited.insert(b);
            }

            let are_tags = !tag_node_indices.is_empty();
            let all_node_indices=if are_tags{tag_node_indices}else{non_tag_node_indices};

            //
            let record_actual_vals_num =record.value_count() - if are_tags {1} else {0};
            let mut record_parsed_values = HashMap::<usize,ParsedVal>::new();
            let mut record_parsed_names = HashMap::<usize,String>::new();

            //
            let mut params_num_err = false;

            //
            for &node_index in all_node_indices.iter() {
                let node = &self.nodes[node_index];
                last_node = Some(node);

                //tag with no args
                if are_tags && node.vals.len()==0 && record.value_count()==1 {
                    ok = true;
                    break;
                }

                //
                if are_tags && node.vals.len()==0 && record.value_count()>1 {
                    params_num_err = true;
                    continue;
                } 
                
                //
                // println!("hm {:?} : {record_actual_vals_num:?} {}",node.vals,node.vals.len());
                
                if node.repeat && record_actual_vals_num % node.vals.len() !=0 {
                    params_num_err = true;
                    continue;
                }
                if node.repeat && record_actual_vals_num==0 && node.vals.len() !=0 {
                    params_num_err = true;
                    continue;
                }
                
                //
                if !node.repeat && record_actual_vals_num != node.vals.len() {
                    params_num_err = true;
                    continue;
                }

                //todo, check if node + record have children, also base on ignore child errs

                //
                record_parsed_values.clear();
                record_parsed_names.clear();
                
                //kinda messy, 
                let mut record_val_ind = if are_tags {1} else {0};

                //loop through record vals, parsing etc
                while record_val_ind < record.value_count() { //node_val_ind < node.vals.len()
                    let node_val_ind = (record_val_ind-if are_tags {1} else {0}) % node.vals.len();
                    let walker_val = &node.vals[node_val_ind];
                    let record_val = record.value(record_val_ind).unwrap();

                    match walker_val {
                        WalkerVal::Str => {
                            // record_parsed_values.insert(record_val_ind,ParsedVal(Box::new(record_val.extracted.as_str())));
                        }
                        WalkerVal::Named(n) => {
                            if let Some(x)=param_parser(n,record_val.extracted.as_str()) {
                                record_parsed_values.insert(record_val_ind,ParsedVal(x));
                                record_parsed_names.insert(record_val_ind, n.clone());
                                // println!("===yes");
                            } else {
                                expecter.expect(record,record_val_ind, WalkerErrorType::ParamParseError);
                                found_err = true;
                                // println!("===no");
                                break;
                            }
                        }
                        // WalkerVal::Parse(p) => {
                        //     match p.parse(&record_val.extracted) {
                        //         Ok(x) => {
                        //             record_parsed_values.insert(record_val_ind,x);
                        //         }
                        //         Err(e) => { //parse of record val failed
                        //             expecter.expect(record.path(),record_val.start_loc, WalkerError::ParseError);
                        //             found_err = true;
                        //             break;
                        //         }
                        //     }
                        // }
                        // WalkerVal::Func(f) => {
                        //     if let Some(x) = f.func(&record_val.extracted) {
                        //         record_parsed_values.insert(record_val_ind,x);
                        //     } else {
                        //         expecter.expect(record.src(),record.path(),record_val.start_loc,WalkerError::ParseError);
                        //         found_err = true;
                        //         break;
                        //     }
                        // }
                    }

                    record_val_ind+=1;
                }

                //
                if record_val_ind == record.value_count() { //all tag args succeeded
                    ok = true;
                    break;
                }
            
            }

            //
            if ok {
                
                //
                let last_node = last_node.clone().unwrap();
            
                if last_node.has_tag {
                    let tag_name=record.value(0).unwrap().extracted.as_str();

                    if let Some(tag_onces)=tags_onces.get(tag_name) {
                        if tag_onces.contains(tag_name) {
                            if tags_useds.last().unwrap().intersection(tag_onces).count()>0 {
                                expecter.expect(record,0,WalkerErrorType::ExpectedTagOnce);
                                ok=false;
                            }
                        }

                    }
                    
                }
                
            }

            //on nodes vals success
            if ok {
                let last_node = last_node.clone().unwrap();
            
                if last_node.has_tag {
                    let tag_name=record.value(0).unwrap().extracted.as_str();
                    tags_useds.last_mut().unwrap().insert(tag_name);
                }
                
                //
                expecter.pop_discard();

                //
                // if last_node.has_tag {
                //     record_parsed_values.remove(&0);

                //     for (k,v) in record_parsed_names.iter_mut() {

                //     }
                // }
                

                //
                let walker_record = WalkerRecord::<'a> {
                    record,
                    parsed_values : record_parsed_values,
                    parsed_names: record_parsed_names,
                    node_vals_len:last_node.vals.len(),
                    has_tag : last_node.has_tag,
                    has_repeat : last_node.repeat,
                    parent : walker_record_parent_index,
                    children : Vec::new(),
                    node_label : last_node.node_label.clone().unwrap_or_default()
                        // .unwrap_or_else(||
                        //     if last_node.has_tag{record.value(0).unwrap().extracted.clone()}else{String::new()}
                        // )
                    ,
                    // branch_label : top_branch.branch_label.clone(),
                    branch_label : self.branches.get(last_node.branch_ind).unwrap().branch_label.clone(), 
                    depth,
                    order,
                };

                let walker_record_index=walker_records.len();
                walker_records.push(walker_record);

                if let Some(walker_record_parent)=walker_record_parent_index {
                    walker_records[walker_record_parent].children.push(walker_record_index);
                }

                let walker_record_container = WalkerRecordContainer {
                    walker_records:&walker_records,walker_record_index,
                };

                //
                let callback_result = callback(walker_record_container,true);

                match callback_result {
                    // Ok(None) => {}
                    // Ok(Some(r)) => { //(p,r)
                    //     stk.extend(r.child_iter().rev().map(|c|(c,top_branch_label.clone(), walker_record_parent_index,depth,false))); //+1
                    // }
                    Ok(include) => {
                        //add on exit
                        stk.push((record,top_branch_label.clone(), walker_record_parent_index, depth,Some(walker_record_index),)); //

                        //add includes
                        if let Some(r)=include {
                            stk.extend(r.child_iter().rev().map(|c|
                                (c,top_branch_label.clone(), walker_record_parent_index,depth,None,)
                            )); //+1
                        }

                        //add children of node/record
                        if record.child_count()>0 {
                            if last_node.children.is_none() {//should check branch for no children aswell //last_node.child_branch_labels.len() == 0
                                // found_err = true;
                                if !last_node.ignore_children {
                                    expecter.expect(record,0,WalkerErrorType::ChildrenNotExpected);
                                }
                            } else {
                                stk.extend(record.child_iter().rev().map(|x|
                                    (x,last_node.children.clone().unwrap(),Some(walker_record_index),depth+1,None,)
                                ));
                            }
                        }
                    }
                    Err((val_ind,e)) => {
                        //todo use as predicate, try other nodes if error?
                        //  need option for both?
                        // found_err = true;

                        if val_ind<record.value_count() {
                            expecter.expect(record,val_ind,WalkerErrorType::Custom(e));
                        } else {
                            expecter.expect(record,0,WalkerErrorType::CustomInvalidValInd(val_ind,e));

                        }
                    }
                }

                //expect children or don't care
                if record.child_count()==0 && last_node.children.is_some() && !last_node.ignore_children{                    
                    expecter.expect(record,0,WalkerErrorType::ChildrenExpected);                    
                }
            }

            //handle records where no nodes succeed on them
            if !ok && !found_err && !top_branch.ignore_other_errors {
                if params_num_err {
                    expecter.expect(record,0,WalkerErrorType::ParamsIncorrectNum);
                } else {  //when does this happen?
                    //if !top_branch.ignore_other_errors { //ignore non tag errors or unknown tags
                    expecter.expect(record,0,WalkerErrorType::Unknown);
                    //}
                }
            }

            order+=1;
            
            if ok {
                continue;
            } else {
                expecter.pop_keep();
            }

            
        }
    }












    
    pub fn from_conf(conf:&Conf) -> Result<Self,WalkerErrors<()>> {
        let mut def_walker = Walker::new();

        def_walker
            .branch("root")
                .tagless()
                    .entry().str().skip_child_errs().children("branch")
            .branch("branch")
                .tags(["tags"])
                    .entry().repeat().str().skip_child_errs().children("node")
                .tags(["tagless"])
                    .entry().skip_child_errs().children("node")
                .tags(["from","once"]) //,"have"
                    .entry().repeat().str()
                .tags(["ignore_rest"])
                    .entry()
            .branch("node")
                .tags(["entry"])
                    .entry()
                        .children("entry")
                        .skip_child_errs()
                    .entry().str().repeat()
                        .children("entry")
                        .skip_child_errs()
            .branch("entry")
                .branch_tag_once(["repeat"])
                .branch_tag_once(["label"])
                .branch_tag_once(["children","children_optional"])
                .tags(["repeat","children_optional"])
                    .entry()
                .tags(["children","children_optional","label"])
                    .entry().str()
            .validate();

        let mut new_walker = Walker::new();

        let res=def_walker.apply_simple("root",conf.root(),|n,v|{None},|record|{
            // println!("{} {} [{},{},{}] = {}",
            //     if record.has_tag(){'T'}else{'L'},
            //     record.depth(), 
            //     record.branch(),record.label(),record.tag(),
            //     (0..record.value_count())
            //         .filter_map(|i|if i==0 && record.has_tag(){None}else{Some(record.str(i).unwrap())})
            //         // .map(|i|record.str(i).unwrap())
            //         .collect::<Vec<_>>()
            //         .join(",")
            // );

            match record.branch() {
                "root" => {
                    new_walker.branch(record.str(0));
                }
                "branch" => match record.tag() {
                    "tags" =>{
                        let tags = (0..record.val_num()).map(|i|record.str(i));
                        new_walker.tags(tags);
                    }
                    "tagless" =>{
                        new_walker.tagless();
                    }
                    "from" =>{
                        let tags = (0..record.val_num()).map(|i|record.str(i));

                        for tag in tags {
                            new_walker.branch_nodes_from(tag);
                        }
                    }
                    // "have" => {
                    //     let tags = (0..record.value_count()).map(|i|record.str(i).unwrap());
                    //     new_walker.branch_tag_have(tags);
                    // }
                    "once" => {
                        let tags = (0..record.val_num()).map(|i|record.str(i));
                        new_walker.branch_tag_once(tags);
                    }
                    "ignore_rest" => {
                        new_walker.skip_node_errs();
                    }
                    _ => {panic!("");}
                }
                "node" => match record.tag() {
                    "entry"  =>{
                        new_walker.entry();

                        for i in 0 .. record.val_num() {

                            // if i==0 && record.has_tag() { //what was this for? wouldn't the tag always be entry?
                            //     continue;
                            // }

                            let v=record.str(i);

                            if v=="any" {
                                new_walker.str();
                            } else {
                                new_walker.named_param(v);
                            }
                        }
                    }

                    _ => {panic!("");}
                }
                "entry" => match record.tag() {
                    "label" => {
                        new_walker.label(record.str(0));
                    }
                    "repeat" => {
                        new_walker.repeat();
                    }
                    "children" => {
                        // for i in 1 .. record.val_count() {
                        new_walker.children(record.str(0));
                        // }
                    }
                    "children_optional" => {
                        // for i in 1 .. record.val_count() {
                        new_walker.children(record.str(0));
                        // }

                        new_walker.skip_child_errs();
                    }
                    _ => {panic!("");}
                }
                _ => {}
            }
        });

        res.and_then(|_|Ok(new_walker))
    }
}