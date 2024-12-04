use crate::token::Token;

pub trait TokenReader {
    fn contains_token(&self, text: &str) -> bool;
    fn read_token(&self, text: &str) -> Option<(Box<dyn Token>, usize)>;
}
