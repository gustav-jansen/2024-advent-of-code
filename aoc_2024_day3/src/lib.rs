use std::any::Any;
use std::fs;
use std::error::Error;
use regex::Regex;
use once_cell::sync::Lazy;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < 2 {
            panic!("Need filename as first argument.");
        }

        let file_path = args[1].clone();

        Config { file_path }
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Need filename as first argument.");
        }

        Ok(Config::new(args))
    }
}

pub trait TokenReader {
    fn can_add(&mut self, current: &str, c: char) -> bool;
    fn clone(&self) -> Box<dyn TokenReader>;
}

pub struct NoneReader {
}

pub struct MulReader {
}

impl TokenReader for NoneReader {
    fn can_add(&mut self, _current: &str, c: char) -> bool {
        if c == 'd' || c == 'm' { return false; }
        true
    }

    fn clone(&self) -> Box<dyn TokenReader> {
        Box::new(NoneReader{})
     }
}

impl TokenReader for MulReader {
    fn can_add(&mut self, current: &str, c: char) -> bool {

        let test_string = current.to_string()+&c.to_string();
        if test_string == "mul".to_string()[..test_string.len()] {
            true
        } else { false }
    }

    fn clone(&self) -> Box<dyn TokenReader> {
        Box::new(MulReader{})
     }
}



#[derive(PartialEq, Clone, Copy)]
pub enum TokenType {
    None, Mul, Do, Dont, DoOrDont,
}

pub struct Token {
    type_of_token: TokenType,
    current_string: String,
    component: Box<dyn TokenReader>,
}

impl Token {
    pub fn new() -> Token {
        let type_of_token = TokenType::None;
        let current_string = "".to_string();
        Token {
            type_of_token,
            current_string,
            component: Box::new(NoneReader{}),
        }
    }

    fn clone(&self) -> Token {
        let type_of_token = self.type_of_token;
        let current_string = self.current_string.clone();

        Token {
            type_of_token,
            current_string,
            component: self.component.clone(),
        }
    }

    fn is_empty(&self) -> bool {
        return self.type_of_token == TokenType::None;
    }

    fn start(&mut self, c: char) {
        self.type_of_token = self.get_token_type_from_first_char(c);
        if self.is_empty() { return; }
    }

    fn add(&mut self, c: char) {
        if self.component.can_add(&self.current_string, c) {
            self.current_string += &c.to_string();
        }
    }

    fn is_complete(&mut self) -> bool {
        false
    }

    fn get_token_type_from_first_char(&self, c: char) -> TokenType {
        let mut result = TokenType::None;
        if c == 'd' { result = TokenType::DoOrDont; }
        else if c == 'm' { result = TokenType::Mul; }

        result
    }

    fn clear(&mut self) {
        self.type_of_token = TokenType::None;
        self.current_string = "".to_string();
    }
}
pub fn tokenize_text(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_token = Token::new();

    for c in input.chars() {
        if current_token.is_empty() {
            current_token.start(c);
        } else {
            current_token.add(c);
        }
        if current_token.is_complete() {
            tokens.push(current_token.clone());
            current_token.clear();
        }
    }

    tokens
}

pub fn parse_text(input: &str) -> Vec<&str> {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    re.find_iter(input).map(|element| element.as_str()).collect()
}

 pub fn eval(input :&str) -> usize {
     static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap());

     let Some(caps) = RE.captures(input) else { return 0};

     let left = caps[1].parse::<usize>().unwrap();
     let right = caps[2].parse::<usize>().unwrap();

     left*right
 }

pub fn parse_and_eval(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            let left: usize = caps[1].parse().unwrap();
            let right: usize = caps[2].parse().unwrap();
            left*right
        }).sum()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let the_sum = parse_and_eval(&content);
    println!("The sum of all multiplications: {the_sum}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_text() -> Result<(), Box<dyn std::error::Error>> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let actual: Vec<&str> = [ "mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)" ].to_vec();

        let tokens = parse_text(input);

        if actual == tokens {
            Ok(())
        } else {
            println!("Tokens: {:?}", tokens);
            Err(format!("Found {:?}, expected {:?}", tokens, actual).into())
        }
    }

    #[test]
    fn test_eval() -> Result<(), Box<dyn std::error::Error>> {
        let tokens: Vec<&str> = [ "mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)" ].to_vec();
        let actual: Vec<usize> = vec![8, 25, 88, 40];

        let result: Vec<usize> = tokens.iter()
            .map(|token| eval(token))
            .collect();

        if actual == result {
            Ok(())
        } else {
            Err(format!("Found {:?}, expected {:?}", result, actual).into())
        }
    }

    #[test]
    fn test_parse_and_eval() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let actual: usize = 161;

        let result = parse_and_eval(input);
        assert_eq!(result, actual);
    }

    #[test]
    fn test_parse_and_eval_multipline() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))\n\
                     xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let actual: usize = 322;

        let result = parse_and_eval(input);
        assert_eq!(result, actual);
    }

}
