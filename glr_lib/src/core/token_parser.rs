use glr_core::{time::Time, token::Token};

pub trait TokenParser {
    fn parse_token(&mut self, time: Time, token: &Token);
}

pub trait IterTokenParser: TokenParser {
    fn parse_tokens(&mut self, iterator: impl Iterator<Item = (Time, Token)>) {
        for (time, token) in iterator {
            self.parse_token(time, &token);
        }
    }
}

impl<T: TokenParser + ?Sized> IterTokenParser for T {}

impl<F> TokenParser for F
where
    F: FnMut(Time, &Token),
{
    fn parse_token(&mut self, time: Time, token: &Token) {
        self(time, token);
    }
}

impl TokenParser for Vec<Box<dyn TokenParser>> {
    fn parse_token(&mut self, time: Time, token: &Token) {
        for tp in self {
            tp.parse_token(time, token);
        }
    }
}
