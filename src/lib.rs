#![no_std]

extern crate alloc;

#[cfg(feature = "lex")]
pub mod lex;

#[cfg(feature = "lex")]
pub mod utils;

#[cfg(feature = "lex")]
pub use utils::Source;

#[cfg(feature = "parse")]
pub mod parse;

#[cfg(test)]
mod tests;
