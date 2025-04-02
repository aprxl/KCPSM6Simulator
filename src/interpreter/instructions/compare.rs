use crate::{SimulationContext, SimulationUpdate};
use std::io::{Error, ErrorKind};

pub fn register_register(
    ctx: &SimulationContext,
    lhs: u8,
    rhs: u8,
) -> Result<SimulationUpdate, Error> {
    let mut update = SimulationUpdate::new(ctx);
    let first_register = ctx.get_register(lhs as usize).unwrap();
    let second_register = ctx.get_register(rhs as usize).unwrap();

    let result = first_register.wrapping_sub(second_register);

    update.carry = (first_register as i32 - second_register as i32) < 0;
    update.zero = result == 0u8;

    Ok(update)
}

pub fn register_constant(
    ctx: &SimulationContext,
    lhs: u8,
    rhs: u32,
) -> Result<SimulationUpdate, Error> {
    let mut update = SimulationUpdate::new(ctx);
    let first_register = ctx.get_register(lhs as usize).unwrap();

    let result = first_register.wrapping_sub(rhs as u8);

    update.carry = (first_register as i32 - rhs as i32) < 0;
    update.zero = result == 0u8;

    Ok(update)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_between_registers() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 0b00000101;
        registers[1] = 0b00000001;

        end_registers[0] = 0b00000101;
        end_registers[1] = 0b00000001;

        let context = SimulationContext::new_with_params(registers, 0, false, false);

        assert_eq!(
            register_register(&context, 0, 1).unwrap(),
            SimulationUpdate {
                registers: end_registers,
                carry: false,
                zero: false,
            }
        );
    }

    #[test]
    fn compare_between_register_and_constant() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 0b00000101;

        end_registers[0] = 0b00000101;

        let context = SimulationContext::new_with_params(registers, 0, false, false);

        assert_eq!(
            register_constant(&context, 0, 5).unwrap(),
            SimulationUpdate {
                registers: end_registers,
                carry: false,
                zero: true,
            }
        );
    }

    #[test]
    fn compare_carry() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 0;

        end_registers[0] = 0;

        let context = SimulationContext::new_with_params(registers, 0, false, false);

        assert_eq!(
            register_constant(&context, 0, 1).unwrap(),
            SimulationUpdate {
                registers: end_registers,
                carry: true,
                zero: false,
            }
        );
    }
}
