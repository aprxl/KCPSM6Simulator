use crate::Token;

pub struct Label(String, u32);

#[derive(Debug)]
pub enum Instruction {
    None,
    Add {
        lhs: u8,
        rhs: u8
    },
    AddConstant {
        lhs: u8,
        rhs: u32
    }
}

pub struct Parser {
    instructions: Vec<Instruction>,
    labels: Vec<Label>
}

fn convert_tokens_into_string(token_list: &Vec<Token>) -> String {
    let mut res = String::new();
    for token in token_list {
        let c = match token {
            Token::Word(_) => 'w',
            Token::Instruction(_) => 'i',
            Token::Label(_) => 'l',
            Token::Register(_) => 'r',
            Token::DerefRegister(_) => 'd',
            Token::Number(_, _) => 'n',
            Token::Condition(_) => 'c',
            Token::Comma => 'C',
            Token::EndOfLine => 'e',
        };

        res.push(c);
    }

    res
}

fn instr_reg_reg(token_list: &Vec<Token>) -> Instruction {
    match &token_list[..] {
        [Token::Instruction(instr), Token::Register(lhs), _, Token::Register(rhs)] => {
            match instr.as_str() {
                "add" => Instruction::Add { lhs: *lhs, rhs: *rhs },
                _ => panic!("Unable to parse line!")
            }
        },
        _ => panic!("Unable to parse line!")
    }
}

fn instr_reg_num(token_list: &Vec<Token>) -> Instruction {
    match &token_list[..] {
        [Token::Instruction(instr), Token::Register(lhs), _, Token::Number(rhs, _)] => {
            match instr.as_str() {
                "add" => Instruction::AddConstant { lhs: *lhs, rhs: *rhs },
                _ => panic!("Unable to parse line!")
            }
        },
        _ => panic!("Unable to parse line!")
    }
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            instructions: Vec::new(),
            labels: Vec::new()
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> &mut Parser {
        // Split the tokens by line.
        let tokens_per_line: Vec<Vec<Token>> = tokens.split(|token| {
            match token {
                Token::EndOfLine => true,
                _ => false
            }
        }).map(|list| list.to_vec()).collect();

        for (line_number, line) in tokens_per_line.iter().enumerate() {
            let instr = self.parse_line(line);

            self.instructions.push(instr);
        }

        self
    }

    fn parse_line(&mut self, token_list: &Vec<Token>) -> Instruction {
        let token_list = token_list.clone();

        if let Token::Label(label) = &token_list[0] {
            self.labels.push(Label(label.clone(), 0));
        }

        // I'm so not proud of this, but we ball.
        match convert_tokens_into_string(&token_list).as_str() {
            "irCr" => instr_reg_reg(&token_list),
            "irCn" => instr_reg_num(&token_list),
            _ => Instruction::None
        }
    }

    pub fn get_instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }
}