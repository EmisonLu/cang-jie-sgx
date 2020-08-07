#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate std as std;

pub use crate::{
    options::TokenizerOption, stream::CangjieTokenStream, tokenizer::CangJieTokenizer,
};

pub mod options;
pub mod stream;
pub mod tokenizer;

pub const CANG_JIE: &str = "CANG_JIE";
