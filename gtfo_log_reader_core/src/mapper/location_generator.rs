use crate::core::{location::Location, token::Token};


pub trait LocationGenerator {
    fn accept_token(&mut self, token: &Token) -> Option<Location>;
}
