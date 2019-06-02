use std::collections::HashMap;

#[derive(Clone,Debug,PartialEq)]
pub enum Object {
    Int(i64),
    String(String),
    Bool(bool),
    Struct(String,HashMap<String,Object>),
    NoneObj,
}