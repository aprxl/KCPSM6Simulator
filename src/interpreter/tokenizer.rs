use crate::Reader;

#[derive(Debug, Clone, Copy)]
pub enum NumberType {
    Decimal,
    Hexadecimal,
    Binary,
}

#[derive(Debug, Clone, Copy)]
pub enum ConditionType {
    IfZero,
    IfNonZero,
    IfCarry,
    IfNonCarry,
}

#[derive(Debug, Clone)]
pub enum Token {
    Word(String),
    Instruction(String),
    Label(String),
    Register(u8),
    DerefRegister(u8),
    Number(u32, NumberType),
    Address(u32),
    Condition(ConditionType),
    ConstantDiretive,
    AddressDiretive,
    NameregDiretive,
    Comma,
    EndOfLine,
}

pub struct Tokenizer {
    tokens: Vec<Token>,
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
    // TODO: There are other "instructions" but I will be adding them later.
    // E.g. RETURNI ENABLE/DISABLE, ADDRESS, CONSTANT etc.
    let instructions: Vec<&str> = vec![
        "add", "addcy", "address", "and", "call", "compare", "constant", "fetch", "input", "jump",
        "load", "namereg", "or", "output", "return", "rl", "rr", "sl0", "sl1", "sla", "slx", "sr0",
        "sr1", "sra", "srx", "store", "sub", "subcy", "test", "xor",
    ];

    instructions.contains(&word.as_str())
}

fn is_str_label(word: &String) -> bool {
    word.ends_with(":")
}

fn is_str_hex_number(word: &String) -> bool {
    if word.len() == 2 {
        return word.chars().all(|c| c.is_digit(16));
    }

    false
}

fn is_str_hex_address(word: &String) -> bool {
    // TODO: This can cause issues with identifiers that are three letters long and
    // characters range from 'a' to 'f'. E.g.: 'abc', 'def' etc.
    if word.len() == 3 {
        return word.chars().all(|c| c.is_digit(16));
    }

    false
}

fn is_str_binary_number(word: &String) -> bool {
    // Now checking for binary literals
    if word.ends_with("'b") {
        return word.len() == 10
            && word
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

fn is_str_deref_register(word: &String) -> bool {
    if word.len() != 4 {
        return false;
    }

    if !word.starts_with("(") {
        return false;
    }

    if !word.ends_with(")") {
        return false;
    }

    true
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer { tokens: Vec::new() }
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
                } else if word.to_lowercase() == "c" {
                    self.tokens.push(Token::Condition(ConditionType::IfCarry));
                } else if word.to_lowercase() == "nc" {
                    self.tokens
                        .push(Token::Condition(ConditionType::IfNonCarry));
                } else if word.to_lowercase() == "z" {
                    self.tokens.push(Token::Condition(ConditionType::IfZero));
                } else if word.to_lowercase() == "nz" {
                    self.tokens.push(Token::Condition(ConditionType::IfNonZero));
                } else if word.to_lowercase() == "constant" {
                    self.tokens.push(Token::ConstantDiretive)
                } else if word.to_lowercase() == "address" {
                    self.tokens.push(Token::AddressDiretive);
                } else if word.to_lowercase() == "namereg" {
                    self.tokens.push(Token::NameregDiretive);
                } else if is_str_instruction(&word) {
                    self.tokens.push(Token::Instruction(word.clone()));
                } else if is_str_label(&word) {
                    self.tokens
                        .push(Token::Label(word[0..word.len() - 1].to_string()));
                } else if is_str_hex_number(&word) {
                    let number = u32::from_str_radix(word.as_str(), 16);

                    match number {
                        Ok(number) => self
                            .tokens
                            .push(Token::Number(number, NumberType::Hexadecimal)),
                        Err(_) => {
                            panic!("Unable to parse {} number, at line {}!", word, line_number)
                        }
                    }
                } else if is_str_hex_address(&word) {
                    let number = u32::from_str_radix(word.as_str(), 16);

                    match number {
                        Ok(number) => self.tokens.push(Token::Address(number)),
                        Err(_) => {
                            panic!("Unable to parse {} number, at line {}!", word, line_number)
                        }
                    }
                } else if is_str_binary_number(&word) {
                    // Remove the last two characters of literal
                    // E.g. "00010001'b" becomes "00010001"
                    let literal: &str = &word[..word.len() - 2];
                    let number = u32::from_str_radix(literal, 2);

                    match number {
                        Ok(number) => {
                            if number > 255 {
                                println!(
                                    "WARNING: Possible overflow on {} ({}), at line {}!",
                                    number, literal, line_number
                                );
                            }

                            self.tokens.push(Token::Number(number, NumberType::Binary))
                        }
                        Err(_) => {
                            panic!("Unable to parse {} number, at line {}!", word, line_number)
                        }
                    }
                } else if is_str_decimal_number(&word) {
                    // Remove the last two characters of literal
                    // E.g. "1234'd" becomes "1234"
                    let literal: &str = &word[..word.len() - 2];
                    let number = u32::from_str_radix(literal, 10);

                    match number {
                        Ok(number) => {
                            if number > 255 {
                                println!(
                                    "WARNING: Possible overflow on {}, at line {}!",
                                    number, line_number
                                );
                            }

                            self.tokens.push(Token::Number(number, NumberType::Decimal))
                        }
                        Err(_) => {
                            panic!("Unable to parse {} number, at line {}!", word, line_number)
                        }
                    }
                } else if is_str_register(&word) {
                    // Remove the first letter 's' from the register to access the number.
                    // E.g. 's3' reffers to the 4th (starting from 0) register.
                    let number = u8::from_str_radix(&word[1..], 16);

                    match number {
                        Ok(number) => self.tokens.push(Token::Register(number)),
                        Err(_) => panic!(
                            "Unable to parse {} register, at line {}!",
                            word, line_number
                        ),
                    }
                } else if is_str_deref_register(&word) {
                    // Remove the leading and trailing parentheses, and first letter 's' from the register to access the number.
                    // E.g. '(s3)' reffers to the 4th (starting from 0) register.
                    let number = u8::from_str_radix(&word[2..3], 16);

                    match number {
                        Ok(number) => self.tokens.push(Token::DerefRegister(number)),
                        Err(_) => panic!(
                            "Unable to parse {} register, at line {}!",
                            word, line_number
                        ),
                    }
                } else {
                    self.tokens.push(Token::Word(word.clone()));
                }
            }

            self.tokens.push(Token::EndOfLine);
        }

        self
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}

