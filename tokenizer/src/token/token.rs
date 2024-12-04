use std::any::Any;
use std::fmt;
use super::traits::Token;

// ErrorToken
pub struct ErrorToken;
impl Token for ErrorToken {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl fmt::Display for ErrorToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<ErrorToken>")
    }
}

// NoneToken
pub struct NoneToken;
impl Token for NoneToken {
    fn as_any(&self) -> &dyn Any {
        self
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
}

impl fmt::Display for NumToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<NumToken>({})", self.val)
    }
}
