use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::Source;
use crate::lex::{LexemeKind, LexemeStream};
use crate::utils::{Error, ParsingError, Result};

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

pub fn parse<'a>(src: &'a Source, lexemes: &'a mut LexemeStream) -> Result<'a, Document> {
    let doc = Document::default();

    while let Some(lexeme) = lexemes.pop_front() {
        match lexeme.kind {
            LexemeKind::String(s) | LexemeKind::Ident(s) => {}
            _ => {
                return Err(Error::parsing(
                    ParsingError::InvalidTopLevel,
                    lexeme.span,
                    src,
                ));
            }
        }
    }

    Ok(doc)
}
