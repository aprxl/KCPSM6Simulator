use crate::Instruction;

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

    pub fn run(&mut self) -> Result<(), String> {
        // Ensure that all processor flags and registers are reset.
        self.reset();

        for (addr, i) in &self.instructions {
            self.execute_instruction(i.clone())?;
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
        if index < 0 || index > 16 {
            return None;
        }

        Some(self.registers[index])
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), String> {
        match instruction {
            Instruction::And { lhs, rhs } => todo!(),
            _ => unreachable!(),
        }

        Ok(())
    }
}
