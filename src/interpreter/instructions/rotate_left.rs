use crate::{SimulationContext, SimulationUpdate};
use std::io::Error;

pub fn register(ctx: &SimulationContext, register: u8) -> Result<SimulationUpdate, Error> {
    let mut update = SimulationUpdate::new(ctx);
    let mut register_value = ctx.get_register(register as usize).unwrap();

    // Get the left-most bit and shift it all the way to the right:
    // E.g. stating with 0b10000001. The left-most bit is 1, so we apply
    // the mask and shift it by seven bits, getting the number 0b1.
    let rotated_bit = (register_value & 0b10000000) >> 7;

    // To rotate left, we shift it left first to create a new empty bit on the right.
    // Then we simply add the rotated bit, finishing the rotation.
    register_value = (register_value << 1).wrapping_add(rotated_bit);

    update.carry = rotated_bit > 0;
    update.zero = register_value == 0u8;
    update.registers[register as usize] = register_value;

    Ok(update)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_left() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 0b11001001;

        end_registers[0] = 0b10010011;

        let context = SimulationContext::new_with_params(registers, false, false);

        assert_eq!(
            register(&context, 0).unwrap(),
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
