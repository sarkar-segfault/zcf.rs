use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::Source;
use crate::lex::LexemeStream;
use crate::utils::Result;

pub type Dictionary = BTreeMap<String, Value>;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Dictionary(Dictionary),
    Sequence(Vec<Value>),
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
}

pub type Document = Vec<Dictionary>;

pub fn parse<'a>(_src: &'a Source, _lexemes: &'a mut LexemeStream) -> Result<'a, Document> {
    let doc = Document::default();

    while let Some(_lexeme) = _lexemes.pop_front() {}

    Ok(doc)
}
