use std::any::Any;
use std::fmt;

use super::traits::Token;
use crate::visitor::TokenVisitor;

pub struct CatchAllToken;
impl Token for CatchAllToken {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn accept(&self, visitor: &mut dyn TokenVisitor) {
        visitor.visit_catchall_token(self);
    }
}
impl fmt::Display for CatchAllToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<CatchAllToken>")
    }
}

pub struct ErrorToken;
impl Token for ErrorToken {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn accept(&self, visitor: &mut dyn TokenVisitor) {
        visitor.visit_error_token(self);
    }
}
impl fmt::Display for ErrorToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<ErrorToken>")
    }
}

pub struct NoneToken;
impl Token for NoneToken {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn accept(&self, visitor: &mut dyn TokenVisitor) {
        visitor.visit_none_token(self);
    }
}
impl fmt::Display for NoneToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<NoneToken>")
    }
}

pub struct WordToken {
    pub word: String,
}

impl Token for WordToken {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn accept(&self, visitor: &mut dyn TokenVisitor) {
        visitor.visit_word_token(self);
    }
}
impl fmt::Display for WordToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<WordToken>(\"{}\")", self.word)
    }
}

pub struct CharToken {
    pub c: char,
}

impl Token for CharToken {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn accept(&self, visitor: &mut dyn TokenVisitor) {
        visitor.visit_char_token(self);
    }
}
impl fmt::Display for CharToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<CharToken>('{}')", self.c)
    }
}

pub struct NumToken {
    pub val: usize,
}

impl Token for NumToken {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn accept(&self, visitor: &mut dyn TokenVisitor) {
        visitor.visit_num_token(self);
    }
}

impl fmt::Display for NumToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<NumToken>({})", self.val)
    }
}
