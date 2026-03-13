use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

pub type Dictionary = BTreeMap<String, Value>;

pub enum Value {
    Dictionary(Dictionary),
    Sequence(Vec<Value>),
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
}

pub struct Document(Vec<Dictionary>);
