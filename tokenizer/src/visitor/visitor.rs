use super::traits::TokenVisitor;

use crate::token::{CatchAllToken,ErrorToken,NoneToken,NumToken,WordToken,CharToken};

pub struct Multiplication {
    enabled: bool,
    left: usize,
    right: usize,
}

impl Multiplication {
    pub fn new() -> Self {
        Self { enabled: true, left: 0, right: 0 }
    }

    pub fn update_left(&mut self, left: usize) {
        self.left = left;
    }

    pub fn update_right(&mut self, right: usize) {
        self.right = right;
    }

    pub fn get_result(&mut self) -> usize {
        let result = self.left*self.right;
        self.left = 0; self.right = 0;
        if self.enabled { result }
        else { 0 }
    }

    pub fn enable (&mut self) {
        println!("Enabled");
        self.enabled = true;
    }

    pub fn disable (&mut self) {
        println!("Disabled");
        self.enabled = false;
    }
}

pub struct MultiplicationVisitor {
    depth: usize,
    result: usize,
    op: Multiplication,
}

impl MultiplicationVisitor {
    pub fn new() -> Self {
        Self { depth: 0, result: 0, op: Multiplication::new() }
    }
}

impl TokenVisitor for MultiplicationVisitor {
    fn get_result(&self) -> usize {
        self.result
    }

    fn visit_catchall_token(&mut self, token: &CatchAllToken) {
//        println!("Visiting: {}", token);
    }

    fn visit_error_token(&mut self, token: &ErrorToken) {
        println!("Visiting: {}", token);
    }

    fn visit_none_token(&mut self, token: &NoneToken) {
//        println!("Visiting: {}", token);
        self.depth = 0;
    }

    fn visit_num_token(&mut self, token: &NumToken) {
//        println!("Visiting: {}", token);

        if self.depth == 2 {
            self.depth += 1;
            print!("{}", token.val);
            self.op.update_left(token.val);
        } else if self.depth == 4 {
            self.depth += 1;
            self.op.update_right(token.val);
            print!("{}", token.val);
        } else {
            self.depth = 0;
        }
    }

    fn visit_word_token(&mut self, token: &WordToken) {
//        println!("Visiting: {}", token);
        if self.depth > 0 {
//            println!("Stack is not empty, cleaning up!");
            self.depth = 0;
        }

        match token.word.as_str() {
            "do()" => self.op.enable(),
            "don't()" => self.op.disable(),
            "mul" => {
                self.depth += 1;
                print!("mul"); },
            _ => println!("Unknown token value: {}", token.word),

        }
    }

    fn visit_char_token(&mut self, token: &CharToken) {
 //       println!("Visiting: {}", token);
        
        match token.c{
            '(' =>  if self.depth == 1 {
                        print!("(");
                        self.depth +=1;
                    } else { self.depth = 0; },
            ',' =>  if self.depth == 3 {
                        print!(",");
                        self.depth += 1;
                    } else { self.depth = 0; },
            ')' =>  {
                        if self.depth == 5 {
                            let result = self.op.get_result();
                            print!(") = {result}");
                            self.result += result;
                            println!(" => {}", self.result);
                        }
                        self.depth = 0;
                    },
            _ => { self.depth = 0; println!("Unknown token value: {}", token.c)},
        }
    }
}
