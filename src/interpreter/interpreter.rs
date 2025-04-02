use crate::{instructions::*, Instruction};

use std::io::{Error, ErrorKind};

use super::helpers::ShiftMode;

const PROGRAM_MEMORY_SIZE: usize = 1024usize;

#[derive(Debug, PartialEq)]
pub struct SimulationUpdate {
    pub registers: [u8; 16],
    pub zero: bool,
    pub carry: bool,
    pub pc: usize,
    pub call_addr: Option<usize>,
    pub ret_addr: bool,
}

impl SimulationUpdate {
    pub fn new(ctx: &SimulationContext) -> SimulationUpdate {
        SimulationUpdate {
            registers: ctx.get_registers(),
            zero: ctx.get_zero_flag(),
            carry: ctx.get_carry_flag(),
            pc: ctx.get_program_counter() + 1,
            call_addr: None,
            ret_addr: false,
        }
    }

    pub fn new_with_pc(ctx: &SimulationContext, pc: usize) -> SimulationUpdate {
        SimulationUpdate {
            registers: ctx.get_registers(),
            zero: ctx.get_zero_flag(),
            carry: ctx.get_carry_flag(),
            call_addr: None,
            ret_addr: false,
            pc,
        }
    }
}

impl Default for SimulationUpdate {
    fn default() -> Self {
        Self {
            registers: [0u8; 16],
            zero: false,
            carry: false,
            pc: 0usize,
            call_addr: None,
            ret_addr: false,
        }
    }
}

pub struct SimulationContext {
    //instructions_: Vec<(usize, Instruction)>,
    instructions: Vec<Option<Instruction>>,
    registers: [u8; 16],
    pc: usize,
    zero: bool,
    carry: bool,
    call_stack: Vec<usize>,
}

impl SimulationContext {
    pub fn new() -> SimulationContext {
        SimulationContext {
            //instructions_: Vec::new(),
            instructions: vec![None; PROGRAM_MEMORY_SIZE],
            registers: [0u8; 16],
            pc: 0,
            zero: false,
            carry: false,
            call_stack: vec![],
        }
    }

    pub fn new_with_params(registers: [u8; 16], zero: bool, carry: bool) -> SimulationContext {
        SimulationContext {
            //instructions_: Vec::new(),
            instructions: vec![None; PROGRAM_MEMORY_SIZE],
            pc: 0,
            registers,
            zero,
            carry,
            call_stack: vec![],
        }
    }

    pub fn new_with_instructions(instructions: Vec<(usize, Instruction)>) -> SimulationContext {
        let mut instr_list: Vec<Option<Instruction>> = vec![None; PROGRAM_MEMORY_SIZE];

        for (addr, i) in instructions {
            instr_list[addr] = Some(i);
        }

        SimulationContext {
            //instructions_: instructions,
            instructions: instr_list,
            registers: [0u8; 16],
            pc: 0,
            zero: false,
            carry: false,
            call_stack: vec![],
        }
    }

    pub fn initialize_instructions(
        &mut self,
        instructions: Vec<(usize, Instruction)>,
    ) -> &mut SimulationContext {
        let mut instr_list: Vec<Option<Instruction>> = vec![None; PROGRAM_MEMORY_SIZE];

        for (addr, i) in instructions {
            instr_list[addr] = Some(i);
        }

        self.instructions = instr_list;
        self
    }

    pub fn reset(&mut self) -> &mut SimulationContext {
        self.registers = [0u8; 16];
        self.zero = false;
        self.carry = false;
        self.pc = 0;
        self.call_stack = vec![];
        self
    }

    pub fn run(&mut self) -> Result<(), Error> {
        // Ensure that all processor flags and registers are reset.
        self.reset();

        loop {
            let i = &self.instructions[self.pc];

            if i.is_none() {
                eprintln!("The program ended because it reached an invalid address.");
                break;
            }

            let update = self.execute_instruction(i.clone().unwrap())?;

            println!("{}: {:?}", self.pc, i.clone().unwrap());

            self.registers = update.registers;
            self.zero = update.zero;
            self.carry = update.carry;
            self.pc = update.pc;

            // We just returned.
            if update.ret_addr {
                if let Some(ret_addr) = self.call_stack.pop() {
                    self.pc = ret_addr;
                }
            }

            // We just called to another routine.
            if let Some(addr) = update.call_addr {
                self.call_stack.push(addr);
            }

            println!("{}", self.call_stack.len());
        }

        Ok(())
    }

    pub fn get_zero_flag(&self) -> bool {
        self.zero
    }

    pub fn set_zero_flag(&mut self, zero: bool) {
        self.zero = zero;
    }

    pub fn get_carry_flag(&self) -> bool {
        self.carry
    }

    pub fn set_carry_flag(&mut self, carry: bool) {
        self.carry = carry;
    }

    pub fn get_program_counter(&self) -> usize {
        self.pc
    }

    pub fn set_program_counter(&mut self, pc: usize) {
        self.pc = pc;
    }

    pub fn get_call_stack(&self) -> &Vec<usize> {
        &self.call_stack
    }

    pub fn add_to_call_stack_unrestricted(&mut self, addr: usize) {
        self.call_stack.push(addr);
    }

    pub fn get_registers(&self) -> [u8; 16] {
        self.registers
    }

    pub fn get_register(&self, index: usize) -> Option<u8> {
        if index > 16 {
            return None;
        }

        Some(self.registers[index])
    }

    pub fn set_register(&mut self, index: usize, value: u8) {
        if index > 16 {
            return;
        }

        self.registers[index] = value;
    }

    fn execute_instruction(&self, instruction: Instruction) -> Result<SimulationUpdate, Error> {
        match instruction {
            Instruction::Load { lhs, rhs } => load::register_register(self, lhs, rhs),
            Instruction::LoadConstant { lhs, rhs } => load::register_constant(self, lhs, rhs),
            Instruction::And { lhs, rhs } => and::register_register(self, lhs, rhs),
            Instruction::AndConstant { lhs, rhs } => and::register_constant(self, lhs, rhs),
            Instruction::Call { address } => call::address(self, address, None),
            Instruction::CallConditional { condition, address } => {
                call::address(self, address, Some(condition))
            }
            Instruction::Compare { lhs, rhs } => compare::register_register(self, lhs, rhs),
            Instruction::CompareConstant { lhs, rhs } => compare::register_constant(self, lhs, rhs),
            Instruction::CompareCarry { lhs, rhs } => {
                compare_carry::register_register(self, lhs, rhs)
            }
            Instruction::CompareCarryConstant { lhs, rhs } => {
                compare_carry::register_constant(self, lhs, rhs)
            }
            Instruction::Or { lhs, rhs } => or::register_register(self, lhs, rhs),
            Instruction::OrConstant { lhs, rhs } => or::register_constant(self, lhs, rhs),
            Instruction::Xor { lhs, rhs } => xor::register_register(self, lhs, rhs),
            Instruction::XorConstant { lhs, rhs } => xor::register_constant(self, lhs, rhs),
            Instruction::Add { lhs, rhs } => add::register_register(self, lhs, rhs),
            Instruction::AddConstant { lhs, rhs } => add::register_constant(self, lhs, rhs),
            Instruction::AddCarry { lhs, rhs } => add_carry::register_register(self, lhs, rhs),
            Instruction::AddCarryConstant { lhs, rhs } => {
                add_carry::register_constant(self, lhs, rhs)
            }
            Instruction::Jump { address } => jump::address(self, address, None),
            Instruction::JumpConditional { condition, address } => {
                jump::address(self, address, Some(condition))
            }
            Instruction::Return => ret::default(self, None),
            Instruction::ReturnCondition { condition } => ret::default(self, Some(condition)),
            Instruction::ShiftLeftZero { register } => {
                shift_left::register(self, register, ShiftMode::Number(0))
            }
            Instruction::ShiftLeftOne { register } => {
                shift_left::register(self, register, ShiftMode::Number(1))
            }
            Instruction::ShiftLeftCarry { register } => {
                shift_left::register(self, register, ShiftMode::Carry)
            }
            Instruction::ShiftLeftArth { register } => {
                shift_left::register(self, register, ShiftMode::Repeat)
            }
            Instruction::ShiftRightZero { register } => {
                shift_right::register(self, register, ShiftMode::Number(0))
            }
            Instruction::ShiftRightOne { register } => {
                shift_right::register(self, register, ShiftMode::Number(1))
            }
            Instruction::ShiftRightCarry { register } => {
                shift_right::register(self, register, ShiftMode::Carry)
            }
            Instruction::ShiftRightArth { register } => {
                shift_right::register(self, register, ShiftMode::Repeat)
            }
            Instruction::Subtract { lhs, rhs } => subtract::register_register(self, lhs, rhs),
            Instruction::SubtractConstant { lhs, rhs } => {
                subtract::register_constant(self, lhs, rhs)
            }
            Instruction::SubtractCarry { lhs, rhs } => {
                subtract_carry::register_register(self, lhs, rhs)
            }
            Instruction::SubtractCarryConstant { lhs, rhs } => {
                subtract_carry::register_constant(self, lhs, rhs)
            }
            Instruction::Test { lhs, rhs } => test::register_register(self, lhs, rhs),
            Instruction::TestConstant { lhs, rhs } => test::register_constant(self, lhs, rhs),
            Instruction::TestCarry { lhs, rhs } => test_carry::register_register(self, lhs, rhs),
            Instruction::TestCarryConstant { lhs, rhs } => {
                test_carry::register_constant(self, lhs, rhs)
            }
            Instruction::RotateLeft { register } => rotate_left::register(self, register),
            Instruction::RotateRight { register } => rotate_right::register(self, register),

            _ => Err(Error::new(
                ErrorKind::Unsupported,
                "Unable to run instruction as there's no behavior defined for it.",
            )),
        }
    }
}
