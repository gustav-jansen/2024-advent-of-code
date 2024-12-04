use super::traits::TokenReader;

use crate::token::{CatchAllToken, CharToken, ErrorToken, NoneToken, NumToken, Token, WordToken};

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
            false
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
        if token_string.len() > 0 {
            println!("Found NoneToken: {}", token_string);
            Some((Box::new(NoneToken), token_string.len()))
        } else {
            Some((Box::new(ErrorToken), 0))
        }
    }
}

pub struct CatchAllReader;
impl TokenReader for CatchAllReader {
    fn contains_token(&self, text:&str) -> bool {
        text.len() > 0
    }

    fn read_token(&self, text: &str) -> Option<(Box<dyn Token>, usize)> {
        if text.len() > 0 {
            println!("Found CatchAllToken: {}", &text[..1]);
            Some((Box::new(CatchAllToken), 1))
        } else {
            Some((Box::new(ErrorToken), 0))
        }
    }
}

pub struct WordReader {
    pub word: String,
}

impl TokenReader for WordReader {
    fn contains_token(&self, text:&str) -> bool {
        let mut result = false;
        if text.len() >= self.word.len() {
            result = &text[..self.word.len()] == self.word;
        }
        if text.len() >= self.word.len() {
        }
        result
    }

    fn read_token(&self, text: &str) -> Option<(Box<dyn Token>, usize)> {
        if &text[..self.word.len()] == self.word {
            println!("Found WordToken: {}", &text[..self.word.len()]);
            Some((Box::new(WordToken{word: self.word.clone()}), self.word.len()))
        } else {
            Some((Box::new(ErrorToken), 0))
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
            println!("Found CharToken: {}", &text[..1]);
            Some((Box::new(CharToken{c: self.c}), 1))
        } else {
            Some((Box::new(ErrorToken), 0))
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

        if let Ok(res) = s.parse::<usize>() {
            println!("Found NumToken: {}", res);
            Some((Box::new(NumToken{val: res}), s.len()))
        } else {
            Some((Box::new(ErrorToken), 0))
        }
    }
}
