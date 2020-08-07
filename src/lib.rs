#![no_std]
// use std::prelude::v1::*;
#[macro_use]
extern crate sgx_tstd as std;

pub use crate::{
    options::TokenizerOption, stream::CangjieTokenStream, tokenizer::CangJieTokenizer,
};

pub mod options;
pub mod stream;
pub mod tokenizer;

pub const CANG_JIE: &str = "CANG_JIE";
