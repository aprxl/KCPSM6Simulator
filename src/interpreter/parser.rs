use crate::{ConditionType, Token};

#[derive(Debug, Clone)]
pub struct Label(String, u32);

#[derive(Debug, Clone)]
pub struct Constant(String, u32);

#[derive(Debug, Clone)]
pub struct Alias(String, u8);

#[derive(Debug)]
pub enum Instruction {
    None,
    Add { lhs: u8, rhs: u8 },
    AddConstant { lhs: u8, rhs: u32 },
    AddCarry { lhs: u8, rhs: u8 },
    AddCarryConstant { lhs: u8, rhs: u32 },
    And { lhs: u8, rhs: u8 },
    AndConstant { lhs: u8, rhs: u32 },
    Compare { lhs: u8, rhs: u8 },
    CompareConstant { lhs: u8, rhs: u32 },
    FetchConstant { lhs: u8, rhs: u32 },
    InputConstant { lhs: u8, rhs: u32 },
    Load { lhs: u8, rhs: u8 },
    LoadConstant { lhs: u8, rhs: u32 },
    Or { lhs: u8, rhs: u8 },
    OrConstant { lhs: u8, rhs: u32 },
    OutputConstant { lhs: u8, rhs: u32 },
    Return,
    ReturnCondition { condition: ConditionType },
    StoreConstant { lhs: u8, rhs: u32 },
    Subtract { lhs: u8, rhs: u8 },
    SubtractConstant { lhs: u8, rhs: u32 },
    SubtractCarry { lhs: u8, rhs: u8 },
    SubtractCarryConstant { lhs: u8, rhs: u32 },
    Test { lhs: u8, rhs: u8 },
    TestConstant { lhs: u8, rhs: u32 },
    Xor { lhs: u8, rhs: u8 },
    XorConstant { lhs: u8, rhs: u32 },
}

pub struct Parser {
    instructions: Vec<Instruction>,
    labels: Vec<Label>,
    constants: Vec<Constant>,
    aliases: Vec<Alias>,
}

fn convert_tokens_into_string(token_list: &Vec<Token>) -> String {
    let mut res = String::new();
    for token in token_list {
        let c = match token {
            Token::Word(_) => 'w',
            Token::Instruction(_) => 'i',
            Token::Register(_) => 'r',
            Token::DerefRegister(_) => 'd',
            Token::Number(_, _) => 'n',
            Token::Address(_) => 'a',
            Token::Condition(_) => 'c',
            Token::Comma => 'C',
            Token::EndOfLine => 'e',
            _ => '.',
        };

        res.push(c);
    }

    res
}

fn instr_only(token_list: &Vec<Token>) -> Instruction {
    match token_list.as_slice() {
        [Token::Instruction(instr)] => match instr.as_str() {
            "return" => Instruction::Return,
            _ => panic!("Unable to parse line!"),
        },
        _ => panic!("Unable to parse line!"),
    }
}

fn instr_condition(token_list: &Vec<Token>) -> Instruction {
    match token_list.as_slice() {
        [Token::Instruction(instr), Token::Condition(condition)] => {
            let condition = *condition;
            match instr.as_str() {
                "return" => Instruction::ReturnCondition { condition },
                _ => panic!("Unable to parse line!"),
            }
        }
        _ => panic!("Unable to parse line!"),
    }
}

fn instr_reg_reg(token_list: &Vec<Token>) -> Instruction {
    match token_list.as_slice() {
        [Token::Instruction(instr), Token::Register(lhs), _, Token::Register(rhs)] => {
            let lhs = *lhs;
            let rhs = *rhs;
            match instr.as_str() {
                "add" => Instruction::Add { lhs, rhs },
                "addcy" => Instruction::AddCarry { lhs, rhs },
                "and" => Instruction::And { lhs, rhs },
                "compare" => Instruction::Compare { lhs, rhs },
                "load" => Instruction::Load { lhs, rhs },
                "or" => Instruction::Or { lhs, rhs },
                "sub" => Instruction::Subtract { lhs, rhs },
                "subcy" => Instruction::SubtractCarry { lhs, rhs },
                "test" => Instruction::Test { lhs, rhs },
                "xor" => Instruction::Xor { lhs, rhs },
                _ => panic!("Unable to parse line!"),
            }
        }
        _ => panic!("Unable to parse line!"),
    }
}

fn instr_reg_num(token_list: &Vec<Token>) -> Instruction {
    match token_list.as_slice() {
        [Token::Instruction(instr), Token::Register(lhs), _, Token::Number(rhs, _)] => {
            let lhs = *lhs;
            let rhs = *rhs;
            match instr.as_str() {
                "add" => Instruction::AddConstant { lhs, rhs },
                "addcy" => Instruction::AddCarryConstant { lhs, rhs },
                "and" => Instruction::AndConstant { lhs, rhs },
                "compare" => Instruction::CompareConstant { lhs, rhs },
                "fetch" => Instruction::FetchConstant { lhs, rhs },
                "input" => Instruction::InputConstant { lhs, rhs },
                "load" => Instruction::LoadConstant { lhs, rhs },
                "or" => Instruction::OrConstant { lhs, rhs },
                "output" => Instruction::OutputConstant { lhs, rhs },
                "store" => Instruction::StoreConstant { lhs, rhs },
                "sub" => Instruction::SubtractConstant { lhs, rhs },
                "subcy" => Instruction::SubtractCarryConstant { lhs, rhs },
                "test" => Instruction::TestConstant { lhs, rhs },
                "xor" => Instruction::XorConstant { lhs, rhs },
                _ => panic!("Unable to parse line!"),
            }
        }
        _ => panic!("Unable to parse line!"),
    }
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            instructions: Vec::new(),
            labels: Vec::new(),
            constants: Vec::new(),
            aliases: Vec::new(),
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> &mut Parser {
        // Split the tokens by line.
        let tokens_per_line: Vec<Vec<Token>> = tokens
            .split(|token| match token {
                Token::EndOfLine => true,
                _ => false,
            })
            .map(|list| list.to_vec())
            .collect();

        for (line_number, line) in tokens_per_line.iter().enumerate() {
            let instr = self.parse_line(line, line_number);

            match instr {
                Instruction::None => continue,
                _ => self.instructions.push(instr),
            }
        }

        self
    }

    fn parse_line(&mut self, token_list: &Vec<Token>, line_number: usize) -> Instruction {
        let token_list = self.parse_diretives_and_update_tokens(token_list, line_number);

        if token_list.is_empty() {
            return Instruction::None;
        }

        let syntax_pattern = convert_tokens_into_string(&token_list);

        // I'm so not proud of this, but we ball.
        // Picoblaze assembly is very simple, so we don't need a super
        // sofisticated parser and this will suffice.
        match syntax_pattern.as_str() {
            "i" => instr_only(&token_list),
            "ic" => instr_condition(&token_list),
            "irCr" => instr_reg_reg(&token_list),
            "irCn" => instr_reg_num(&token_list),
            _ => {
                eprintln!(
                    "Failed to parse line {} (pattern {})",
                    line_number, syntax_pattern
                );

                Instruction::None
            }
        }
    }

    fn add_label(&mut self, token: &Token, line_number: usize) {
        if let Token::Label(label) = token {
            self.labels.push(Label(label.clone(), line_number as u32));
        }
    }

    fn add_constant(&mut self, tokens: &Vec<Token>) {
        for t in tokens {
            println!("{:?}", t);
        }
        match tokens.as_slice() {
            [Token::ConstantDiretive, Token::Word(constant_name), _, Token::Number(value, _)] => {
                self.constants.push(Constant(constant_name.clone(), *value));
            }
            _ => unreachable!(),
        }
    }

    fn parse_diretives_and_update_tokens(
        &mut self,
        token_list: &Vec<Token>,
        line_number: usize,
    ) -> Vec<Token> {
        let mut updated_tokens: Vec<Token> = Vec::new();

        for token in token_list {
            match token {
                Token::Label(_) => self.add_label(token, line_number),
                Token::ConstantDiretive => {
                    self.add_constant(token_list);
                    break;
                }
                _ => updated_tokens.push(token.clone()),
            }
        }

        updated_tokens
    }

    pub fn get_instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    pub fn get_labels(&self) -> &Vec<Label> {
        &self.labels
    }

    pub fn get_constants(&self) -> &Vec<Constant> {
        &self.constants
    }
}
