use crate::{SimulationContext, SimulationUpdate};
use std::io::{Error, ErrorKind};

pub fn register_register(
    ctx: &SimulationContext,
    lhs: u8,
    rhs: u8,
) -> Result<SimulationUpdate, Error> {
    let mut update = SimulationUpdate::new(ctx);

    update.registers[lhs as usize] = ctx.get_register(rhs as usize).unwrap();

    Ok(update)
}

pub fn register_constant(
    ctx: &SimulationContext,
    lhs: u8,
    rhs: u32,
) -> Result<SimulationUpdate, Error> {
    let mut update = SimulationUpdate::new(ctx);

    if rhs > 255 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("LOAD: The constant specified was too large ({})!", rhs),
        ));
    }

    update.registers[lhs as usize] = rhs as u8;

    Ok(update)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_between_registers() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 0b00001111;
        registers[1] = 0b00001100;

        end_registers[0] = 0b00001100;
        end_registers[1] = 0b00001100;

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
    fn load_between_register_and_constant() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 0b00001111;

        end_registers[0] = 0b00001100;

        let context = SimulationContext::new_with_params(registers, 0, false, false);

        assert_eq!(
            register_constant(&context, 0, 0b00001100).unwrap(),
            SimulationUpdate {
                registers: end_registers,
                carry: false,
                zero: false,
            }
        );
    }

    #[test]
    #[should_panic]
    fn load_constant_overflow() {
        let registers = [0u8; 16];

        let context = SimulationContext::new_with_params(registers, 0, false, false);

        assert_eq!(
            register_constant(&context, 0, 12345).unwrap(),
            SimulationUpdate {
                registers,
                carry: false,
                zero: false,
            }
        );
    }
}
