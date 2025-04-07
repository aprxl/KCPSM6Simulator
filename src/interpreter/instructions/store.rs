use crate::{interpreter::interpreter::MemoryOperation, SimulationContext, SimulationUpdate, SCRATCH_PAD_MEMORY_SIZE};
use std::io::{Error, ErrorKind};

pub fn register_constant(
    ctx: &SimulationContext,
    lhs: u8,
    rhs: u32,
) -> Result<SimulationUpdate, Error> {
    let rhs= rhs as usize;
    let value = ctx.get_register(lhs as usize).unwrap();
    let mut update = SimulationUpdate::new(ctx);

    if rhs > SCRATCH_PAD_MEMORY_SIZE {
        return Err(
            Error::new(ErrorKind::AddrNotAvailable, 
            format!("Unable to store value into address as it is out of bounds! (address was {}, max is {}!", rhs, SCRATCH_PAD_MEMORY_SIZE))
        );
    }

    // TODO: How does pBlazeIDE handle values that are greater than FF?
    update.memory_op = Some(MemoryOperation::Store(rhs, value));

    Ok(update)
}

pub fn register_deref(
    ctx: &SimulationContext,
    lhs: u8,
    rhs: u8,
) -> Result<SimulationUpdate, Error> {
    let addr = ctx.get_register(rhs as usize).unwrap() as usize;
    let value = ctx.get_register(lhs as usize).unwrap();

    let mut update = SimulationUpdate::new(ctx);

    if addr > SCRATCH_PAD_MEMORY_SIZE {
        return Err(
            Error::new(ErrorKind::AddrNotAvailable, 
            format!("Unable to store value into address as it is out of bounds! (address was {}, max is {}!", rhs, SCRATCH_PAD_MEMORY_SIZE))
        );
    }

    // TODO: How does pBlazeIDE handle values that are greater than FF?
    update.memory_op = Some(MemoryOperation::Store(addr, value));

    Ok(update)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_constant() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 0b00001111;

        end_registers[0] = 0b00001111;

        let context = SimulationContext::new_with_params(registers, false, false);

        assert_eq!(
            register_constant(&context, 0, 1).unwrap(),
            SimulationUpdate {
                registers: end_registers,
                carry: false,
                zero: false,
                pc: 1,
                memory_op: Some(MemoryOperation::Store(1usize, 15)),
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn store_deref_register() {
        let mut registers = [0u8; 16];
        let mut end_registers = [0u8; 16];

        registers[0] = 0b00001111;
        registers[1] = 0b00000100;

        end_registers[0] = 0b00001111;
        end_registers[1] = 0b00000100;

        let context = SimulationContext::new_with_params(registers, false, false);

        assert_eq!(
            register_deref(&context, 0, 1).unwrap(),
            SimulationUpdate {
                registers: end_registers,
                carry: false,
                zero: false,
                pc: 1,
                memory_op: Some(MemoryOperation::Store(4usize, 15)),
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    #[should_panic]
    fn store_out_of_bounds() {
        let registers = [0u8; 16];

        let context = SimulationContext::new_with_params(registers, false, false);

        assert_eq!(
            register_constant(&context, 0, 1024).unwrap(),
            SimulationUpdate {
                registers,
                carry: false,
                zero: false,
                pc: 1,
                ..SimulationUpdate::default()
            }
        );
    }
}
