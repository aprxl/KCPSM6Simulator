use crate::{ConditionType, Token};

#[derive(Debug, Clone)]
pub struct Label(String, u32);

#[derive(Debug, Clone)]
pub struct Constant(String, u32);

#[derive(Debug, Clone)]
pub struct Alias(String, u8);

#[derive(Debug, Clone)]
pub enum Register {
    None(u8),
    Not(u8),
}

// @TODO: Use the Register enum type instead of u8 for registers. Update
// the entire code base accordingly :smiley:.
#[derive(Debug)]
#[rustfmt::skip]
pub enum Instruction {
    None,
    Add { lhs: u8, rhs: u8 },
    AddConstant { lhs: u8, rhs: u32 },
    AddCarry { lhs: u8, rhs: u8 },
    AddCarryConstant { lhs: u8, rhs: u32 },
    And { lhs: u8, rhs: u8 },
    AndConstant { lhs: u8, rhs: u32 },
    Call { address: u32 },
    CallAt { first: u8, second: u8 },
    CallConditional { condition: ConditionType, address: u32 },
    Compare { lhs: u8, rhs: u8 },
    CompareConstant { lhs: u8, rhs: u32 },
    CompareCarry { lhs: u8, rhs: u8 },
    CompareCarryConstant { lhs: u8, rhs: u32 },
    FetchConstant { lhs: u8, rhs: u32 },
    FetchDeref { lhs: u8, rhs: u8 },
    HardwareBuild { register: u8 },
    InputConstant { lhs: u8, rhs: u32 },
    InputDeref { lhs: u8, rhs: u8 },
    Interrupt { state: bool },
    Jump { address: u32 },
    JumpAt { first: u8, second: u8 },
    JumpConditional { condition: ConditionType, address: u32 },
    Load { lhs: u8, rhs: u8 },
    LoadAndReturn { lhs: u8, rhs: u32 },
    LoadConstant { lhs: u8, rhs: u32 },
    Or { lhs: u8, rhs: u8 },
    OrConstant { lhs: u8, rhs: u32 },
    OutputConstant { lhs: u8, rhs: u32 },
    OutputDoubleConstant { lhs: u32, rhs: u32 },
    OutputDeref { lhs: u8, rhs: u8 },
    Regbank { selection: char },
    Return,
    ReturnCondition { condition: ConditionType },
    ReturnInterrupt { state: bool },
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
            Token::Number(_, _) => 'n',
            Token::Address(_) | Token::Label(_) => 'a',
            Token::Condition(_) => 'c',
            Token::Comma => 'C',
            Token::Parentheses => 'p',
            Token::Tilda => continue,
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
    let match_instruction = |instr: &str, lhs: u8, rhs: u8| match instr {
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
    };

    match token_list.as_slice() {
        [Token::Instruction(instr), Token::Register(lhs), _, Token::Register(rhs)] => {
            match_instruction(instr.as_str(), *lhs, *rhs)
        }
        [Token::Instruction(instr), Token::Register(lhs), _, Token::Tilda, Token::Register(rhs)] => {
            match_instruction(instr.as_str(), *lhs, *rhs)
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
        [Token::Instruction(instr), Token::Register(lhs), _, _, Token::Register(rhs), _] => {
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

fn instr_double_deref(token_list: &Vec<Token>) -> Instruction {
    match token_list.as_slice() {
        [Token::Instruction(instr), _, Token::Register(first), _, Token::Register(second), _] => {
            let first = *first;
            let second = *second;

            match instr.as_str() {
                "jump@" => Instruction::JumpAt { first, second },
                "call@" => Instruction::CallAt { first, second },
                _ => panic!("Unable to parse line!"),
            }
        }
        _ => panic!("Unable to parse line!"),
    }
}

fn instr_addr(token_list: &Vec<Token>) -> Instruction {
    match token_list.as_slice() {
        [Token::Instruction(instr), Token::Address(address)] => {
            let address = *address;

            match instr.as_str() {
                "jump" => Instruction::Jump { address },
                "call" => Instruction::Call { address },
                _ => panic!("Unable to parse line!"),
            }
        }
        _ => panic!("Unable to parse line!"),
    }
}

fn instr_condition_addr(token_list: &Vec<Token>) -> Instruction {
    match token_list.as_slice() {
        [Token::Instruction(instr), Token::Condition(condition), _, Token::Address(address)] => {
            let condition = *condition;
            let address = *address;

            match instr.as_str() {
                "jump" => Instruction::JumpConditional { condition, address },
                "call" => Instruction::CallConditional { condition, address },
                _ => panic!("Unable to parse line!"),
            }
        }
        _ => panic!("Unable to parse line!"),
    }
}

fn word_word(token_list: &Vec<Token>) -> Instruction {
    match token_list.as_slice() {
        [Token::Word(w1), Token::Word(w2)] => match w1.to_lowercase().as_str() {
            "regbank" => {
                let w2 = w2.to_lowercase();

                if w2 == "a" {
                    return Instruction::Regbank { selection: 'a' };
                } else if w2 == "b" {
                    return Instruction::Regbank { selection: 'b' };
                } else {
                    panic!("Unable to parse line!")
                }
            }

            "returni" => {
                let w2 = w2.to_lowercase();

                if w2 == "disable" {
                    return Instruction::ReturnInterrupt { state: false };
                } else if w2 == "enable" {
                    return Instruction::ReturnInterrupt { state: true };
                } else {
                    panic!("Unable to parse line!")
                }
            }

            "enable" | "disable" => {
                let w1 = w1.to_lowercase();
                let w2 = w2.to_lowercase();

                if w2 == "interrupt" {
                    if w1 == "enable" {
                        return Instruction::Interrupt { state: true };
                    } else if w1 == "disable" {
                        return Instruction::Interrupt { state: false };
                    } else {
                        panic!("Unable to parse line!");
                    }
                } else {
                    panic!("Unable to parse line!");
                }
            }
            _ => panic!("Unable to parse line!"),
        },
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

        // TODO: We could build a possibly smaller token matrix here already ignoring the
        // directives. This could save some time when parsing for instructions.
        //
        // Run through the tokens once to find assembler directive.
        for line in tokens_per_line.clone() {
            if self.addresses.contains(&instruction_address) {
                panic!(
                    "Attempted to add instruction at address that's already occupied ({}).",
                    instruction_address
                );
            }

            let (should_increment, new_address) = self.parse_directives(&line, instruction_address);

            // Check if we should increment the current address. This makes sure that
            // lines with only directives aren't incrementing the address since they don't
            // technically make part of the code.
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

        // Make sure that our instructions are sorted so we execute them in order.
        self.instructions.sort_by(|a, b| {
            let (addr_a, _) = a;
            let (addr_b, _) = b;

            addr_a.cmp(addr_b)
        });

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
        //
        // TODO: Implement support for call@ (sx, sy) and jump@ (sx, sy)
        match syntax_pattern.as_str() {
            "i" => (updated_addr, instr_only(&token_list)),
            "ic" => (updated_addr, instr_condition(&token_list)),
            "ir" => (updated_addr, instr_reg(&token_list)),
            "irCr" => (updated_addr, instr_reg_reg(&token_list)),
            "irCn" => (updated_addr, instr_reg_num(&token_list)),
            "irCprp" => (updated_addr, instr_reg_deref(&token_list)), // Update
            "inCn" => (updated_addr, instr_num_num(&token_list)),
            "ia" => (updated_addr, instr_addr(&token_list)),
            "icCa" => (updated_addr, instr_condition_addr(&token_list)),
            "iprCrp" => (updated_addr, instr_double_deref(&token_list)), // Update
            "ww" => (updated_addr, word_word(&token_list)),
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

            [Token::ConstantDirective, Token::Word(constant_name), _, Token::Word(word)] => {
                if let Some(Constant(_, value)) = self.find_constant(word) {
                    self.constants.push(Constant(constant_name.clone(), value));
                } else {
                    panic!("Unable to parse constant.");
                }
            }
            _ => unreachable!("Constant"),
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

            [Token::NameregDirective, Token::Word(other_alias), _, Token::Word(alias_name)] => {
                if let Some(Alias(_, register)) = self.find_alias(other_alias) {
                    self.aliases.push(Alias(alias_name.clone(), register));
                } else {
                    panic!("Unable to parse alias.");
                }
            }
            _ => unreachable!("Alias"),
        }
    }

    fn update_address(&mut self, tokens: &Vec<Token>) -> usize {
        match tokens.as_slice() {
            [Token::AddressDirective, Token::Address(addr)] => *addr as usize,
            [Token::AddressDirective, Token::Word(word)] => {
                if let Some(Constant(_, addr)) = self.find_constant(word) {
                    return addr as usize;
                } else {
                    panic!("Unable to parse address.");
                }
            }
            _ => unreachable!("Address"),
        }
    }

    fn parse_directives(
        &mut self,
        token_list: &Vec<Token>,
        instruction_address: usize,
    ) -> (bool, usize) {
        if token_list.is_empty() {
            return (false, instruction_address);
        }

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
                    break;
                }
                Token::AddressDirective => {
                    updated_addr = self.update_address(token_list);

                    is_valid_instruction = false;
                    break;
                }
                _ => {
                    continue;
                }
            }
        }

        if is_valid_instruction {
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

        if let Some(Alias(_, reg)) = self.find_alias(word) {
            return Token::Register(reg);
        }

        // Remove trailing and leading parentheses to make sure DerefRegister's with an alias work.
        let word = word
            .clone()
            .chars()
            .filter(|c| *c != '(' && *c != ')')
            .collect::<String>();

        // TODO: Deref registers don't exist anymore, woohoo!
        //if let Some(Alias(_, reg)) = self.find_alias(&word) {
        //return Token::DerefRegister(reg);
        //}

        // TODO: This is such a painful way of doing this. It works, but it scratches that part of
        // my brain that tells me I'm being stupid.
        /*
        if word.contains(",") {
            let is_register_like = |s: &String| -> bool {
                let mut chars = s.chars();

                return s.len() == 2
                    && chars.next().unwrap_or(' ') == 's'
                    && chars.nth(0).unwrap_or(' ').is_digit(16);
            };

            let words: Vec<&str> = word.split(",").collect();

            if words.len() == 2 {
                let w1 = words[0].to_string();
                let w2 = words[1].to_string();
                println!("{} {}", w1, w2);
                let mut first = 0u8;
                let mut second = 0u8;

                if !is_register_like(&w1) {
                    if let Some(Alias(_, register)) = self.find_alias(&w1) {
                        first = register;
                    } else {
                        panic!("Unable to parse register at DerefDoubleRegister.");
                    }
                } else {
                    first = u8::from_str_radix(&w1[1..2], 16)
                        .expect("Unable to parse register at DerefDoubleRegister.");
                }

                if !is_register_like(&w2) {
                    if let Some(Alias(_, register)) = self.find_alias(&w2) {
                        second = register;
                    } else {
                        panic!("Unable to parse register at DerefDoubleRegister.");
                    }
                } else {
                    second = u8::from_str_radix(&w2[1..2], 16)
                        .expect("Unable to parse register at DerefDoubleRegister.");
                }

                return Token::DoubleDerefRegister(first, second);
            }
        }*/

        Token::Word(word.clone())
    }

    fn ignore_directives_and_update_tokens(
        &mut self,
        token_list: &Vec<Token>,
        instruction_address: usize,
    ) -> (usize, Vec<Token>) {
        let mut updated_tokens: Vec<Token> = Vec::new();
        let mut updated_addr = instruction_address;

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

    pub fn get_aliases(&self) -> &Vec<Alias> {
        &self.aliases
    }

    pub fn find_alias(&self, alias: &String) -> Option<Alias> {
        self.aliases
            .iter()
            .find(|a| {
                let Alias(name, _) = a;
                name == alias
            })
            .cloned()
    }
}
