use crate::{interpreter::helpers::ShiftMode, SimulationContext, SimulationUpdate};
use std::io::Error;

fn get_shift_value(register: u8, carry: bool, mode: ShiftMode) -> u8 {
    match mode {
        ShiftMode::Carry => {
            if carry {
                1
            } else {
                0
            }
        }
        ShiftMode::Number(n) => n,
        ShiftMode::Repeat => register & 0b00000001,
    }
}

pub fn register(
    ctx: &SimulationContext,
    register: u8,
    mode: ShiftMode,
) -> Result<SimulationUpdate, Error> {
    let mut update = SimulationUpdate::new(ctx);
    let mut register_value = ctx.get_register(register as usize).unwrap();

    // A left shift will always insert a new bit into the right, depending on which
    // shift operation you use.
    let shift_value = get_shift_value(register_value, ctx.get_carry_flag(), mode);

    // Check if the bit we're shifting out of the register is one. If so, set carry flag.
    let carry_value = register_value & (0b10000000);

    // To shift the value, just use the left shift operator then add the new bit to the right.
    register_value = (register_value << 1).wrapping_add(shift_value);

    update.carry = if carry_value == 1 { true } else { false };
    update.zero = register_value == 0u8;
    update.registers[register as usize] = register_value;

    Ok(update)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_left_zero() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 0b00000010;

        end_registers[0] = 0b0000100;

        let context = SimulationContext::new_with_params(registers, false, false);

        assert_eq!(
            register(&context, 0, ShiftMode::Number(0)).unwrap(),
            SimulationUpdate {
                registers: end_registers,
                carry: false,
                zero: false,
                pc: 1
            }
        );
    }

    #[test]
    fn shift_left_one() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 0b00000010;

        end_registers[0] = 0b0000101;

        let context = SimulationContext::new_with_params(registers, false, false);

        assert_eq!(
            register(&context, 0, ShiftMode::Number(1)).unwrap(),
            SimulationUpdate {
                registers: end_registers,
                carry: false,
                zero: false,
                pc: 1
            }
        );
    }

    #[test]
    fn shift_left_carry() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 0b00000010;

        end_registers[0] = 0b0000101;

        let context = SimulationContext::new_with_params(registers, false, true);

        assert_eq!(
            register(&context, 0, ShiftMode::Carry).unwrap(),
            SimulationUpdate {
                registers: end_registers,
                carry: false,
                zero: false,
                pc: 1
            }
        );
    }

    #[test]
    fn shift_left_repeat() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 0b00000011;

        end_registers[0] = 0b00000111;

        let context = SimulationContext::new_with_params(registers, false, false);

        assert_eq!(
            register(&context, 0, ShiftMode::Repeat).unwrap(),
            SimulationUpdate {
                registers: end_registers,
                carry: false,
                zero: false,
                pc: 1
            }
        );
    }
}
