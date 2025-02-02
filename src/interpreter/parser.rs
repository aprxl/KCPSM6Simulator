use std::thread::current;

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
    CompareCarry { lhs: u8, rhs: u8 },
    CompareCarryConstant { lhs: u8, rhs: u32 },
    FetchConstant { lhs: u8, rhs: u32 },
    HardwareBuild { register: u8 },
    InputConstant { lhs: u8, rhs: u32 },
    Load { lhs: u8, rhs: u8 },
    LoadAndReturn { lhs: u8, rhs: u32 },
    LoadConstant { lhs: u8, rhs: u32 },
    Or { lhs: u8, rhs: u8 },
    OrConstant { lhs: u8, rhs: u32 },
    OutputConstant { lhs: u8, rhs: u32 },
    Return,
    ReturnCondition { condition: ConditionType },
    RotateLeft { register: u8 },
    RotateRight { register: u8 },
    ShiftLeftZero { register: u8 },
    ShiftLeftOne { register: u8 },
    ShiftLeftCarry { register: u8 },
    ShiftLeftArth { register: u8 },
    ShiftRightZero { register: u8 },
    ShiftRightOne { register: u8 },
    ShiftRightCarry { register: u8 },
    ShiftRightArth { register: u8 },
    StoreConstant { lhs: u8, rhs: u32 },
    Star { lhs: u8, rhs: u8 },
    StarConstant { lhs: u8, rhs: u32 },
    Subtract { lhs: u8, rhs: u8 },
    SubtractConstant { lhs: u8, rhs: u32 },
    SubtractCarry { lhs: u8, rhs: u8 },
    SubtractCarryConstant { lhs: u8, rhs: u32 },
    Test { lhs: u8, rhs: u8 },
    TestConstant { lhs: u8, rhs: u32 },
    TestCarry { lhs: u8, rhs: u8 },
    TestCarryConstant { lhs: u8, rhs: u32 },
    Xor { lhs: u8, rhs: u8 },
    XorConstant { lhs: u8, rhs: u32 },
}

pub struct Parser {
    instructions: Vec<(usize, Instruction)>,
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
            Token::Address(_) | Token::Label(_) => 'a',
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
                "comparecy" => Instruction::CompareCarry { lhs, rhs },
                "load" => Instruction::Load { lhs, rhs },
                "or" => Instruction::Or { lhs, rhs },
                "star" => Instruction::Star { lhs, rhs },
                "sub" => Instruction::Subtract { lhs, rhs },
                "subcy" => Instruction::SubtractCarry { lhs, rhs },
                "test" => Instruction::Test { lhs, rhs },
                "testcy" => Instruction::TestCarry { lhs, rhs },
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
                "comparecy" => Instruction::CompareCarryConstant { lhs, rhs },
                "fetch" => Instruction::FetchConstant { lhs, rhs },
                "input" => Instruction::InputConstant { lhs, rhs },
                "load" => Instruction::LoadConstant { lhs, rhs },
                "load&return" => Instruction::LoadAndReturn { lhs, rhs },
                "or" => Instruction::OrConstant { lhs, rhs },
                "output" => Instruction::OutputConstant { lhs, rhs },
                "store" => Instruction::StoreConstant { lhs, rhs },
                "star" => Instruction::StarConstant { lhs, rhs },
                "sub" => Instruction::SubtractConstant { lhs, rhs },
                "subcy" => Instruction::SubtractCarryConstant { lhs, rhs },
                "test" => Instruction::TestConstant { lhs, rhs },
                "testcy" => Instruction::TestCarryConstant { lhs, rhs },
                "xor" => Instruction::XorConstant { lhs, rhs },
                _ => panic!("Unable to parse line!"),
            }
        }
        _ => panic!("Unable to parse line!"),
    }
}

fn instr_reg(token_list: &Vec<Token>) -> Instruction {
    match token_list.as_slice() {
        [Token::Instruction(instr), Token::Register(register)] => {
            let register = *register;

            match instr.as_str() {
                "sl0" => Instruction::ShiftLeftZero { register },
                "sl1" => Instruction::ShiftLeftOne { register },
                "sla" => Instruction::ShiftLeftArth { register },
                "slx" => Instruction::ShiftLeftCarry { register },
                "sr0" => Instruction::ShiftRightZero { register },
                "sr1" => Instruction::ShiftRightOne { register },
                "sra" => Instruction::ShiftRightArth { register },
                "srx" => Instruction::ShiftRightCarry { register },
                "rl" => Instruction::RotateLeft { register },
                "rr" => Instruction::RotateRight { register },
                "hwbuild" => Instruction::HardwareBuild { register },
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

        let mut instruction_address = 0;

        for line in tokens_per_line {
            let (new_address, instr) = self.parse_line(&line, instruction_address);

            match instr {
                Instruction::None => instruction_address = new_address,
                _ => {
                    self.instructions.push((new_address, instr));
                    instruction_address = new_address + 1;
                }
            }
        }

        self
    }

    fn parse_line(
        &mut self,
        token_list: &Vec<Token>,
        instruction_address: usize,
    ) -> (usize, Instruction) {
        let (updated_addr, token_list) =
            self.parse_diretives_and_update_tokens(token_list, instruction_address);

        if token_list.is_empty() {
            return (updated_addr, Instruction::None);
        }

        let syntax_pattern = convert_tokens_into_string(&token_list);

        // I'm so not proud of this, but we ball.
        // Picoblaze assembly is very simple, so we don't need a super
        // sofisticated parser and this will suffice.
        match syntax_pattern.as_str() {
            "i" => (updated_addr, instr_only(&token_list)),
            "ic" => (updated_addr, instr_condition(&token_list)),
            "ir" => (updated_addr, instr_reg(&token_list)),
            "irCr" => (updated_addr, instr_reg_reg(&token_list)),
            "irCn" => (updated_addr, instr_reg_num(&token_list)),
            _ => {
                eprintln!(
                    "Failed to parse line {} (pattern {})",
                    updated_addr, syntax_pattern
                );

                (updated_addr, Instruction::None)
            }
        }
    }

    fn add_label(&mut self, token: &Token, instruction_address: usize) {
        if let Token::Label(label) = token {
            if self
                .labels
                .iter()
                .find(|l| {
                    let Label(name, _) = l;
                    label == name
                })
                .is_some()
            {
                panic!(
                    "There is already a label called '{}' (line {})!",
                    label, instruction_address
                );
            }

            self.labels
                .push(Label(label.clone(), instruction_address as u32));
        }
    }

    fn add_constant(&mut self, tokens: &Vec<Token>) {
        match tokens.as_slice() {
            [Token::ConstantDiretive, Token::Word(constant_name), _, Token::Number(value, _)] => {
                self.constants.push(Constant(constant_name.clone(), *value));
            }
            _ => unreachable!(),
        }
    }

    fn update_address(&mut self, tokens: &Vec<Token>) -> usize {
        match tokens.as_slice() {
            [Token::AddressDiretive, Token::Address(addr)] => *addr as usize,
            _ => unreachable!(),
        }
    }

    fn parse_diretives_and_update_tokens(
        &mut self,
        token_list: &Vec<Token>,
        instruction_address: usize,
    ) -> (usize, Vec<Token>) {
        let mut updated_tokens: Vec<Token> = Vec::new();
        let mut updated_addr = instruction_address;

        for token in token_list {
            match token {
                Token::Label(_) => self.add_label(token, instruction_address),
                Token::ConstantDiretive => {
                    self.add_constant(token_list);
                    break;
                }
                Token::AddressDiretive => {
                    updated_addr = self.update_address(token_list);
                    break;
                }
                _ => {
                    updated_tokens.push(token.clone());
                }
            }
        }

        (updated_addr, updated_tokens)
    }

    pub fn get_instructions(&self) -> &Vec<(usize, Instruction)> {
        &self.instructions
    }

    pub fn get_labels(&self) -> &Vec<Label> {
        &self.labels
    }

    pub fn get_constants(&self) -> &Vec<Constant> {
        &self.constants
    }
}
