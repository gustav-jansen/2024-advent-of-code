use crate::token::Token;
use crate::reader::{TokenReader,NoneReader,NumReader,WordReader,CharReader,CatchAllReader};
use crate::visitor::{TokenVisitor,MultiplicationVisitor};

pub fn read_next_token(token_readers: &Vec<Box<dyn TokenReader>>, text: &str) -> Option<(Box<dyn Token>, usize)> {
    for reader in token_readers {
        if reader.contains_token(text) {
            return reader.read_token(text);
        }
    }
    println!("Returned without finding tokens");
    None
}

pub fn tokenize(token_readers: &Vec<Box<dyn TokenReader>>, text: &str)
        -> Vec<Box<dyn Token>> {
    let mut i: usize = 0;
    let mut result:Vec<Box<dyn Token>> = Vec::new();

    while i < text.len() {
        println!("{} of {}", i, text.len());
        if let Some((token, j)) = read_next_token(token_readers, &text[i..]) {
            result.push(token);
            i += j;
        } else {
            break;
        }

    }

    result
}

pub fn process_tokens(visitor: &mut dyn TokenVisitor, tokens: &Vec<Box<dyn Token>> ) -> usize {
    for token in tokens {
        token.accept(visitor);
    }

    visitor.get_result()
}

pub fn process_text(input: &str) -> usize {
        let mut readers: Vec<Box<dyn TokenReader>> = Vec::new();
        readers.push(Box::new(WordReader{word: "mul".to_string()}));
        readers.push(Box::new(WordReader{word: "don't()".to_string()}));
        readers.push(Box::new(WordReader{word: "do()".to_string()}));
        readers.push(Box::new(CharReader{c: ')'}));
        readers.push(Box::new(CharReader{c: ','}));
        readers.push(Box::new(CharReader{c: '('}));
        readers.push(Box::new(NumReader));
        readers.push(Box::new(NoneReader));
        readers.push(Box::new(CatchAllReader));

        let tokens = tokenize(&readers, &input);

        let mut visitor = MultiplicationVisitor::new();

        process_tokens(&mut visitor, &tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{NoneToken,NumToken,WordToken,CharToken};

    #[test]
    fn test_none_reader() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let mut readers: Vec<Box<dyn TokenReader>> = Vec::new();
        readers.push(Box::new(NoneReader));

        let (token, i) = read_next_token(&readers, input).unwrap();
        assert_eq!(i, 1);
        assert!(token.as_any().is::<NoneToken>());
    }

    #[test]
    fn test_mul_reader() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let mut readers: Vec<Box<dyn TokenReader>> = Vec::new();
        readers.push(Box::new(WordReader{word: "mul".to_string()}));

        let (token, i) = read_next_token(&readers, &input[1..]).unwrap();
        assert_eq!(i, 3);
        assert!(token.as_any().is::<WordToken>());
        if let Some(concrete) = token.as_any().downcast_ref::<WordToken>() {
            assert_eq!(concrete.word, "mul");
        } else {
            assert!(false);
        }

    }

    #[test]
    fn test_start_par_reader() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let mut readers: Vec<Box<dyn TokenReader>> = Vec::new();
        readers.push(Box::new(CharReader{c: '('}));

        let (token, i) = read_next_token(&readers, &input[4..]).unwrap();
        assert_eq!(i, 1);
        assert!(token.as_any().is::<CharToken>());
        if let Some(concrete) = token.as_any().downcast_ref::<CharToken>() {
            assert_eq!(concrete.c, '(');
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_num_reader() {
        let input = "xmul(2465,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let mut readers: Vec<Box<dyn TokenReader>> = Vec::new();
        readers.push(Box::new(NumReader));

        let (token, i) = read_next_token(&readers, &input[5..]).unwrap();
        assert_eq!(i, 4);
        assert!(token.as_any().is::<NumToken>());
        if let Some(concrete) = token.as_any().downcast_ref::<NumToken>() {
            assert_eq!(concrete.val, 2465);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_comma_reader() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let mut readers: Vec<Box<dyn TokenReader>> = Vec::new();
        readers.push(Box::new(CharReader{c: ','}));

        let (token, i) = read_next_token(&readers, &input[6..]).unwrap();
        assert_eq!(i, 1);
        assert!(token.as_any().is::<CharToken>());
        if let Some(concrete) = token.as_any().downcast_ref::<CharToken>() {
            assert_eq!(concrete.c, ',');
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_end_par_reader() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let mut readers: Vec<Box<dyn TokenReader>> = Vec::new();
        readers.push(Box::new(CharReader{c: ')'}));

        let (token, i) = read_next_token(&readers, &input[8..]).unwrap();
        assert_eq!(i, 1);
        assert!(token.as_any().is::<CharToken>());
        if let Some(concrete) = token.as_any().downcast_ref::<CharToken>() {
            assert_eq!(concrete.c, ')');
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_do_reader() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let mut readers: Vec<Box<dyn TokenReader>> = Vec::new();
        readers.push(Box::new(WordReader{word: "do()".to_string()}));

        for (i,c) in input.chars().enumerate() {println!("{i}, {c}"); }
        let (token, i) = read_next_token(&readers, &input[59..]).unwrap();
        assert_eq!(i, 4);
        assert!(token.as_any().is::<WordToken>());
        if let Some(concrete) = token.as_any().downcast_ref::<WordToken>() {
            assert_eq!(concrete.word, "do()");
        } else {
            assert!(false);
        }

    }

    #[test]
    fn test_dont_reader() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let mut readers: Vec<Box<dyn TokenReader>> = Vec::new();
        readers.push(Box::new(WordReader{word: "don't()".to_string()}));

        for (i,c) in input.chars().enumerate() {println!("{i}, {c}"); }
        let (token, i) = read_next_token(&readers, &input[20..]).unwrap();
        assert_eq!(i, 7);
        assert!(token.as_any().is::<WordToken>());
        if let Some(concrete) = token.as_any().downcast_ref::<WordToken>() {
            assert_eq!(concrete.word, "don't()");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_tokenize() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let mut readers: Vec<Box<dyn TokenReader>> = Vec::new();
        readers.push(Box::new(WordReader{word: "don't()".to_string()}));
        readers.push(Box::new(WordReader{word: "do()".to_string()}));
        readers.push(Box::new(CharReader{c: ')'}));
        readers.push(Box::new(CharReader{c: ','}));
        readers.push(Box::new(CharReader{c: '('}));
        readers.push(Box::new(NumReader));
        readers.push(Box::new(NoneReader));
        readers.push(Box::new(WordReader{word: "mul".to_string()}));

        let tokens = tokenize(&readers, &input);

        for token in &tokens {
            println!("{}", token);
        }

        assert_eq!(46, tokens.len());
    }

    #[test]
    fn test_process_tokens() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let mut readers: Vec<Box<dyn TokenReader>> = Vec::new();
        readers.push(Box::new(WordReader{word: "don't()".to_string()}));
        readers.push(Box::new(WordReader{word: "do()".to_string()}));
        readers.push(Box::new(CharReader{c: ')'}));
        readers.push(Box::new(CharReader{c: ','}));
        readers.push(Box::new(CharReader{c: '('}));
        readers.push(Box::new(NumReader));
        readers.push(Box::new(NoneReader));
        readers.push(Box::new(WordReader{word: "mul".to_string()}));

        let tokens = tokenize(&readers, &input);

        for token in &tokens {
            println!("{}", token);
        }

        assert_eq!(46, tokens.len());

        let mut visitor = MultiplicationVisitor::new();

        let result = process_tokens(&mut visitor, &tokens);
        assert_eq!(48, result);
    }

    #[test]
    fn test_process_text() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let result = process_text(input);
        assert_eq!(48, result);
    }
}
