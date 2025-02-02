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
    FetchDeref { lhs: u8, rhs: u8 },
    HardwareBuild { register: u8 },
    InputConstant { lhs: u8, rhs: u32 },
    InputDeref { lhs: u8, rhs: u8 },
    Load { lhs: u8, rhs: u8 },
    LoadAndReturn { lhs: u8, rhs: u32 },
    LoadConstant { lhs: u8, rhs: u32 },
    Or { lhs: u8, rhs: u8 },
    OrConstant { lhs: u8, rhs: u32 },
    OutputConstant { lhs: u8, rhs: u32 },
    OutputDoubleConstant { lhs: u32, rhs: u32 },
    OutputDeref { lhs: u8, rhs: u8 },
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
    StoreDeref { lhs: u8, rhs: u8 },
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
    addresses: Vec<usize>,
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

fn instr_reg_deref(token_list: &Vec<Token>) -> Instruction {
    match token_list.as_slice() {
        [Token::Instruction(instr), Token::Register(lhs), _, Token::DerefRegister(rhs)] => {
            let lhs = *lhs;
            let rhs = *rhs;

            match instr.as_str() {
                "input" => Instruction::InputDeref { lhs, rhs },
                "output" => Instruction::OutputDeref { lhs, rhs },
                "fetch" => Instruction::FetchDeref { lhs, rhs },
                "store" => Instruction::StoreDeref { lhs, rhs },
                _ => panic!("Unable to parse line!"),
            }
        }
        _ => panic!("Unable to parse line!"),
    }
}

fn instr_num_num(token_list: &Vec<Token>) -> Instruction {
    match token_list.as_slice() {
        [Token::Instruction(instr), Token::Number(lhs, _), _, Token::Number(rhs, _)] => {
            let lhs = *lhs;
            let rhs = *rhs;

            match instr.as_str() {
                "outputk" => Instruction::OutputDoubleConstant { lhs, rhs },
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
            addresses: Vec::new(),
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

        // Run through the tokens once to find assembler directive.
        for line in tokens_per_line.clone() {
            if self.addresses.contains(&instruction_address) {
                panic!(
                    "Attempted to add instruction at address that's already occupied ({}).",
                    instruction_address
                );
            }

            let (should_increment, new_address) = self.parse_directives(&line, instruction_address);

            if should_increment {
                instruction_address = new_address + 1;
            } else {
                instruction_address = new_address;
            }
        }

        instruction_address = 0;

        // Then parse the tokens for instructions.
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
            self.ignore_directives_and_update_tokens(token_list, instruction_address);

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
            "irCd" => (updated_addr, instr_reg_deref(&token_list)),
            "inCn" => (updated_addr, instr_num_num(&token_list)),
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
            [Token::ConstantDirective, Token::Word(constant_name), _, Token::Number(value, _)] => {
                self.constants.push(Constant(constant_name.clone(), *value));
            }
            _ => unreachable!(),
        }
    }

    fn add_alias(&mut self, tokens: &Vec<Token>) {
        // TODO: It turns out namereg directives are not creating aliases, but instead RENAMING a
        // register. For example, if you do `namereg s1, first`, then `s1` is not longer "in the
        // scope". Right now we're hoping that the user won't try to access a register by its
        // original name after the namereg.
        match tokens.as_slice() {
            [Token::NameregDirective, Token::Register(register), _, Token::Word(alias_name)] => {
                self.aliases.push(Alias(alias_name.clone(), *register));
            }
            _ => unreachable!(),
        }
    }

    fn update_address(&mut self, tokens: &Vec<Token>) -> usize {
        match tokens.as_slice() {
            [Token::AddressDirective, Token::Address(addr)] => *addr as usize,
            _ => unreachable!(),
        }
    }

    fn parse_directives(
        &mut self,
        token_list: &Vec<Token>,
        instruction_address: usize,
    ) -> (bool, usize) {
        let mut updated_addr = instruction_address;
        let mut is_valid_instruction = true;

        for token in token_list {
            match token {
                Token::Label(_) => {
                    self.add_label(token, instruction_address);

                    // Checking if length is greater than one to check if this is an inline label.
                    is_valid_instruction = token_list.len() > 1;
                }
                Token::ConstantDirective => {
                    self.add_constant(token_list);

                    is_valid_instruction = false;
                    break;
                }
                Token::NameregDirective => {
                    self.add_alias(token_list);

                    is_valid_instruction = false;
                }
                Token::AddressDirective => {
                    updated_addr = self.update_address(token_list);

                    is_valid_instruction = false;
                }
                _ => {
                    continue;
                }
            }
        }

        if is_valid_instruction {
            println!("Instr: {}", instruction_address);

            for t in token_list {
                println!("{:?}", t);
            }

            println!();
            self.addresses.push(instruction_address);
        }

        (is_valid_instruction, updated_addr)
    }

    fn try_to_convert_word_into_token(&self, word: &String) -> Token {
        if let Some(Label(_, addr)) = self.find_label(word) {
            return Token::Address(addr);
        }

        if let Some(Constant(_, value)) = self.find_constant(word) {
            return Token::Number(value, crate::NumberType::Decimal);
        }

        Token::Word(word.clone())
    }

    fn ignore_directives_and_update_tokens(
        &mut self,
        token_list: &Vec<Token>,
        instruction_address: usize,
    ) -> (usize, Vec<Token>) {
        let mut updated_tokens: Vec<Token> = Vec::new();
        let mut updated_addr = instruction_address;

        // TODO: You can use constants in the address and constants directive, e.g.
        // ```
        // CONSTANT addr, 1234'd
        // CONSTANT addr2, addr
        // ADDRESS addr2
        // ```
        // The code currently doesn't support this.
        for token in token_list {
            match token {
                Token::Label(_) => continue,
                Token::ConstantDirective | Token::NameregDirective => {
                    break;
                }
                Token::AddressDirective => {
                    updated_addr = self.update_address(token_list);
                    break;
                }
                _ => {
                    let mut final_token = token.clone();

                    if let Token::Word(word) = final_token {
                        final_token = self.try_to_convert_word_into_token(&word);
                    }

                    updated_tokens.push(final_token.clone());
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

    pub fn find_label(&self, label: &String) -> Option<Label> {
        self.labels
            .iter()
            .find(|l| {
                let Label(name, _) = l;
                name == label
            })
            .cloned()
    }

    pub fn get_constants(&self) -> &Vec<Constant> {
        &self.constants
    }

    pub fn find_constant(&self, constant: &String) -> Option<Constant> {
        self.constants
            .iter()
            .find(|c| {
                let Constant(name, _) = c;
                name == constant
            })
            .cloned()
    }
}
