use super::traits::TokenReader;

use crate::token::{Token,NumToken,NoneToken,WordToken,CharToken};

pub struct NoneReader;

impl NoneReader {
    const INVALID_ASCII: [char; 5] = ['m', 'd', '(', ')', ','];

    fn is_valid_char(&self, c: char) -> bool {
        !(NoneReader::INVALID_ASCII.contains(&c) || c.is_ascii_digit())
    }
}

impl TokenReader for NoneReader {
    fn contains_token(&self, text:&str) -> bool {
        if let Some(first_char) = text.chars().next() {
            self.is_valid_char(first_char)
        } else {
            true
        }
    }

    fn read_token(&self, text: &str) -> Option<(Box<dyn Token>, usize)> {
        let mut token_string = "".to_string();
        for c in text.chars() {
            if self.is_valid_char(c) {
                token_string += &c.clone().to_string();
            } else {
                break;
            }
        }
        Some((Box::new(NoneToken), token_string.len()))
    }
}

pub struct WordReader {
    pub word: String,
}

impl TokenReader for WordReader {
    fn contains_token(&self, text:&str) -> bool {
        if text.len() >= self.word.len() {
            &text[..self.word.len()] == self.word
        } else {
            false
        }
    }

    fn read_token(&self, text: &str) -> Option<(Box<dyn Token>, usize)> {
        if &text[..self.word.len()] == self.word {
            Some((Box::new(WordToken{word: self.word.clone()}), self.word.len()))
        } else {
            None
        }
    }
}

pub struct CharReader {
    pub c: char,
}

impl TokenReader for CharReader {
    fn contains_token(&self, text:&str) -> bool {
        if text.len() > 0 {
            &text[..1] == self.c.to_string()
        } else {
            false
        }
    }

    fn read_token(&self, text: &str) -> Option<(Box<dyn Token>, usize)> {
        if &text[..1] == self.c.to_string() {
            Some((Box::new(CharToken{c: self.c}), 1))
        } else {
            None
        }
    }
}

pub struct NumReader;

impl TokenReader for NumReader {
    fn contains_token(&self, text:&str) -> bool {
        if let Some(c) = text.chars().next() {
            c.is_ascii_digit()
        } else {
            false
        }
    }

    fn read_token(&self, text: &str) -> Option<(Box<dyn Token>, usize)> {
        let mut s = "".to_string();

        for c in text.chars() {
            if c.is_ascii_digit() { s += &c.to_string(); }
            else { break}
        }

        let res: usize = s.parse().unwrap();
        if res == 0 {
            None
        } else {
            Some((Box::new(NumToken{val: res}), s.len()))
        }
    }
}
