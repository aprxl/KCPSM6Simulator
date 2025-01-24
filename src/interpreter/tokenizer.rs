use crate::Reader;

#[derive(Debug, Clone)]
pub enum Token {
    Basic(String)
}

pub struct Tokenizer {
    tokens: Vec<Token>
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            tokens: Vec::new()
        }
    }

    pub fn tokenize(&mut self, file_contents: Vec<Vec<String>>) -> &mut Tokenizer {
        for line in file_contents {
            for word in line {
                self.tokens.push(Token::Basic(word));
            }
        }

        self
    }

    pub const fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}