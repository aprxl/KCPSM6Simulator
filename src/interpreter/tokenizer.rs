use crate::Reader;

#[derive(Debug, Clone, Copy)]
pub enum NumberType {
    Decimal,
    Hexadecimal,
    Binary
}

#[derive(Debug, Clone)]
pub enum Token {
    Instruction(String),
    Label(String),
    Register(u8),
    Number(u32, NumberType),
    Comma,
    EndOfLine
}

pub struct Tokenizer {
    tokens: Vec<Token>
}

// Found Char::is_digit to be a better solution.
// fn is_char_hexadecimal_number(c: char) -> bool {
//     match c.to_ascii_lowercase() {
//         'a'..='f' => true,
//         '0'..='9' => true,
//         _ => false
//     }
// }

// fn is_str_begin_of_comment(word: &String) -> bool {
//     word.contains(";")
// }

fn is_str_instruction(word: &String) -> bool {
    let instructions: Vec<&str> = vec![
        "add",
        "addcy",
        "and",
        "call",
        "compare",
        "fetch",
        "input",
        "jump",
        "load",
        "or",
        "output",
        "return",
        "rl",
        "rr",
        "sl0",
        "sl1",
        "sla",
        "slx",
        "sr0",
        "sr1",
        "sra",
        "srx",
        "store",
        "sub",
        "subcy",
        "test",
        "xor"
    ];

    instructions.contains(&word.as_str())
}

fn is_str_label(word: &String) -> bool {
    word.ends_with(":")
}

fn is_str_hex_number(word: &String) -> bool {
    if word.len() == 2 {
        return word
            .chars()
            .all(|c| c.is_digit(16));
    }

    false
}


fn is_str_binary_number(word: &String) -> bool {
    // Now checking for binary literals
    if word.ends_with("'b") {
        return word
            .chars()
            .take(word.len() - 2) // Make sure that the last two characters are not included.
            .all(|c| c.is_digit(2));
    }

    false
}

fn is_str_decimal_number(word: &String) -> bool {
    // Finally, decimal literals.
    if word.ends_with("'d") {
        return word
            .chars()
            .take(word.len() - 2) // Make sure that the last two characters are not included.
            .all(|c| c.is_digit(10));
    }

    false
}

fn is_str_register(word: &String) -> bool {
    if word.len() != 2 {
        return false;
    }

    if word.chars().next().unwrap() != 's' {
        return false;
    }

    if !word.chars().nth(1).unwrap().is_digit(16) {
        return false;
    }

    true
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            tokens: Vec::new()
        }
    }

    pub fn tokenize(&mut self, file_contents: Vec<Vec<String>>) -> &mut Tokenizer {
        for (line_number, line) in file_contents.iter().enumerate() {
            for (word_number, word) in line.iter().enumerate() {
                // Comments are filtered out by the Reader.
                // if is_str_begin_of_comment(&word) {
                //     break;
                // }

                if word == "," {
                    self.tokens.push(Token::Comma);
                }

                else if is_str_instruction(&word) {
                    self.tokens.push(Token::Instruction(word.clone()));
                }

                else if is_str_label(&word) {
                    self.tokens.push(Token::Label(word[0..word.len()-1].to_string()));
                }

                else if is_str_hex_number(&word) {
                    let number = u32::from_str_radix(word.as_str(), 16);

                    match number {
                        Ok(number) => self.tokens.push(Token::Number(number, NumberType::Hexadecimal)),
                        Err(_) => panic!("Unable to parse {} number, at line {}!", word, line_number),
                    }
                }

                else if is_str_binary_number(&word) {
                    let literal: &str = &word[..word.len() - 2];
                    let number = u32::from_str_radix(literal, 2);

                    match number {
                        Ok(number) => self.tokens.push(Token::Number(number, NumberType::Binary)),
                        Err(_) => panic!("Unable to parse {} number, at line {}!", word, line_number),
                    }
                }

                else if is_str_decimal_number(&word) {
                    let literal: &str = &word[..word.len() - 2];
                    let number = u32::from_str_radix(literal, 10);

                    match number {
                        Ok(number) => self.tokens.push(Token::Number(number, NumberType::Decimal)),
                        Err(_) => panic!("Unable to parse {} number, at line {}!", word, line_number),
                    }
                }

                else if is_str_register(&word) {
                    // Remove the first letter 's' from the register to access the number.
                    // E.g. 's3' reffers to the 4th (starting from 0) register.
                    let number = u8::from_str_radix(&word[1..], 16);

                    match number {
                        Ok(number) => self.tokens.push(Token::Register(number)),
                        Err(_) => panic!("Unable to parse {} register, at line {}!", word, line_number),
                    }
                }

                else {
                    panic!("Unexpected token '{}' found at line {}, word {}!", word, line_number + 1, word_number + 1);
                }
            }

            self.tokens.push(Token::EndOfLine);
        }

        self
    }

    pub const fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}