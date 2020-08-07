/// Tokenizer Option
use std::prelude::v1::*;
#[derive(Debug, Clone)]
pub enum TokenizerOption {
    /// Cut the input text, return all possible words
    All,
    /// Cut the input text
    Default {
        /// `hmm`: enable HMM or not
        hmm: bool,
    },

    /// Cut the input text in search mode
    ForSearch {
        /// `hmm`: enable HMM or not
        hmm: bool,
    },
    /// Cut the input text into UTF-8 characters
    Unicode,
}
