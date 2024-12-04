use crate::token::{CatchAllToken,ErrorToken,NoneToken,NumToken,WordToken,CharToken};

pub trait TokenVisitor {
    fn get_result(&self) -> usize;
    fn visit_error_token(&mut self, token: &ErrorToken);
    fn visit_none_token(&mut self, token: &NoneToken);
    fn visit_num_token(&mut self, token: &NumToken);
    fn visit_word_token(&mut self, token: &WordToken);
    fn visit_char_token(&mut self, token: &CharToken);
    fn visit_catchall_token(&mut self, token: &CatchAllToken);
}
