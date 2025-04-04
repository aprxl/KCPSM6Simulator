use crate::{SimulationContext, SimulationUpdate};
use std::io::Error;

pub fn register_register(
    ctx: &SimulationContext,
    lhs: u8,
    rhs: u8,
) -> Result<SimulationUpdate, Error> {
    let mut update = SimulationUpdate::new(ctx);
    let first_register = ctx.get_register(lhs as usize).unwrap();
    let second_register = ctx.get_register(rhs as usize).unwrap();

    let carry_inc = if ctx.get_carry_flag() { 1 } else { 0 };

    let result = first_register.wrapping_add(second_register.wrapping_add(carry_inc));

    update.carry = first_register as u32 + second_register as u32 + carry_inc as u32 > 255;
    update.zero = result == 0u8;
    update.registers[lhs as usize] = result;

    Ok(update)
}

pub fn register_constant(
    ctx: &SimulationContext,
    lhs: u8,
    rhs: u32,
) -> Result<SimulationUpdate, Error> {
    let mut update = SimulationUpdate::new(ctx);
    let first_register = ctx.get_register(lhs as usize).unwrap();

    let carry_inc = if ctx.get_carry_flag() { 1 } else { 0 };

    let result = first_register.wrapping_add((rhs as u8).wrapping_add(carry_inc));

    update.carry = first_register as u32 + rhs + carry_inc as u32 > 255;
    update.zero = result == 0u8;
    update.registers[lhs as usize] = result;

    Ok(update)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_between_registers() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 0b00000001;
        registers[1] = 0b00000001;

        end_registers[0] = 0b00000010;
        end_registers[1] = 0b00000001;

        let context = SimulationContext::new_with_params(registers, false, false);

        assert_eq!(
            register_register(&context, 0, 1).unwrap(),
            SimulationUpdate {
                registers: end_registers,
                carry: false,
                zero: false,
                pc: 1,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn add_between_registers_with_carry() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 0b00000001;
        registers[1] = 0b00000001;

        end_registers[0] = 0b00000011;
        end_registers[1] = 0b00000001;

        let context = SimulationContext::new_with_params(registers, false, true);

        assert_eq!(
            register_register(&context, 0, 1).unwrap(),
            SimulationUpdate {
                registers: end_registers,
                carry: false,
                zero: false,
                pc: 1,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn add_between_register_and_constant() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 0b00000001;

        end_registers[0] = 0b00000010;

        let context = SimulationContext::new_with_params(registers, false, false);

        assert_eq!(
            register_constant(&context, 0, 1).unwrap(),
            SimulationUpdate {
                registers: end_registers,
                carry: false,
                zero: false,
                pc: 1,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn add_between_register_and_constant_with_carry() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 0b00000001;

        end_registers[0] = 0b00000011;

        let context = SimulationContext::new_with_params(registers, false, true);

        assert_eq!(
            register_constant(&context, 0, 1).unwrap(),
            SimulationUpdate {
                registers: end_registers,
                carry: false,
                zero: false,
                pc: 1,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn add_carry() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 255;

        end_registers[0] = 0;

        let context = SimulationContext::new_with_params(registers, false, false);

        assert_eq!(
            register_constant(&context, 0, 1).unwrap(),
            SimulationUpdate {
                registers: end_registers,
                carry: true,
                zero: true,
                pc: 1,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn add_carry_with_carry() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 255;

        end_registers[0] = 1;

        let context = SimulationContext::new_with_params(registers, false, true);

        assert_eq!(
            register_constant(&context, 0, 1).unwrap(),
            SimulationUpdate {
                registers: end_registers,
                carry: true,
                zero: false,
                pc: 1,
                ..SimulationUpdate::default()
            }
        );
    }
}
