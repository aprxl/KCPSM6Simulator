use crate::{instructions::*, Instruction};

use std::io::{Error, ErrorKind};

use super::helpers::ShiftMode;

#[derive(Debug, PartialEq)]
pub struct SimulationUpdate {
    pub registers: [u8; 16],
    pub zero: bool,
    pub carry: bool,
}

impl SimulationUpdate {
    pub fn new(ctx: &SimulationContext) -> SimulationUpdate {
        SimulationUpdate {
            registers: ctx.get_registers(),
            zero: ctx.get_zero_flag(),
            carry: ctx.get_carry_flag(),
        }
    }
}

pub struct SimulationContext {
    instructions: Vec<(usize, Instruction)>,
    registers: [u8; 16],
    program_counter: u32,
    temp: u8,
    zero: bool,
    carry: bool,
}

impl SimulationContext {
    pub fn new() -> SimulationContext {
        SimulationContext {
            instructions: Vec::new(),
            registers: [0u8; 16],
            program_counter: 0,
            temp: 0,
            zero: false,
            carry: false,
        }
    }

    pub fn new_with_params(
        registers: [u8; 16],
        temp: u8,
        zero: bool,
        carry: bool,
    ) -> SimulationContext {
        SimulationContext {
            instructions: Vec::new(),
            program_counter: 0,
            registers,
            temp,
            zero,
            carry,
        }
    }

    pub fn new_with_instructions(instructions: Vec<(usize, Instruction)>) -> SimulationContext {
        SimulationContext {
            instructions,
            registers: [0u8; 16],
            program_counter: 0,
            temp: 0,
            zero: false,
            carry: false,
        }
    }

    pub fn initialize_instructions(
        &mut self,
        instructions: Vec<(usize, Instruction)>,
    ) -> &mut SimulationContext {
        self.instructions = instructions;
        self
    }

    pub fn reset(&mut self) -> &mut SimulationContext {
        self.registers = [0u8; 16];
        self.zero = false;
        self.carry = false;
        self.temp = 0;
        self.program_counter = 0;
        self
    }

    pub fn run(&mut self) -> Result<(), Error> {
        // Ensure that all processor flags and registers are reset.
        self.reset();

        for (addr, i) in &self.instructions {
            let update = self.execute_instruction(i.clone())?;

            self.registers = update.registers;
            self.zero = update.zero;
            self.carry = update.carry;
        }

        Ok(())
    }

    pub fn get_zero_flag(&self) -> bool {
        self.zero
    }

    pub fn get_carry_flag(&self) -> bool {
        self.carry
    }

    pub fn get_program_counter(&self) -> u32 {
        self.program_counter
    }

    pub fn get_temporary_var(&self) -> u8 {
        self.temp
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

    fn execute_instruction(&self, instruction: Instruction) -> Result<SimulationUpdate, Error> {
        match instruction {
            Instruction::Load { lhs, rhs } => load::register_register(self, lhs, rhs),
            Instruction::LoadConstant { lhs, rhs } => load::register_constant(self, lhs, rhs),
            Instruction::And { lhs, rhs } => and::register_register(self, lhs, rhs),
            Instruction::AndConstant { lhs, rhs } => and::register_constant(self, lhs, rhs),
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
            Instruction::RotateLeft { register } => rotate_left::register(self, register),
            Instruction::RotateRight { register } => rotate_right::register(self, register),

            _ => Err(Error::new(
                ErrorKind::Unsupported,
                "Unable to run instruction as there's no behavior defined for it.",
            )),
        }
    }
}
