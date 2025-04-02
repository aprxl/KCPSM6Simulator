use crate::{ConditionType, SimulationContext, SimulationUpdate};
use std::io::{Error, ErrorKind};

pub fn address(
    ctx: &SimulationContext,
    address: u32,
    condition: Option<ConditionType>,
) -> Result<SimulationUpdate, Error> {
    let mut update = SimulationUpdate::new(ctx);
    let call_stack = ctx.get_call_stack();

    if call_stack.len() >= 30 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "CALL: The call stack is full!",
        ));
    }

    if address > 0x3FF {
        return Err(Error::new(
            ErrorKind::AddrNotAvailable,
            format!("JUMP: The user tried to jump to an address outside of the program ROM ({}, max is 0x3FF)!", address)
        ));
    }

    if let Some(condition) = condition {
        let should_jump = match condition {
            ConditionType::IfNonZero => !ctx.get_zero_flag(),
            ConditionType::IfZero => ctx.get_zero_flag(),
            ConditionType::IfNonCarry => !ctx.get_carry_flag(),
            ConditionType::IfCarry => ctx.get_carry_flag(),
        };

        if should_jump {
            update.pc = address as usize;
            update.call_addr = Some(ctx.get_program_counter() + 1);
        }
    } else {
        update.pc = address as usize;
        update.call_addr = Some(ctx.get_program_counter() + 1);
    }

    Ok(update)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call() {
        let context = SimulationContext::new_with_params([0u8; 16], false, false);

        assert_eq!(
            address(&context, 25, None).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: false,
                zero: false,
                pc: 25,
                call_addr: Some(1),
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn call_non_zero_valid() {
        let context = SimulationContext::new_with_params([0u8; 16], false, false);

        assert_eq!(
            address(&context, 25, Some(ConditionType::IfNonZero)).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: false,
                zero: false,
                pc: 25,
                call_addr: Some(1),
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn call_non_zero_invalid() {
        let context = SimulationContext::new_with_params([0u8; 16], true, false);

        assert_eq!(
            address(&context, 25, Some(ConditionType::IfNonZero)).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: false,
                zero: true,
                pc: 1,
                call_addr: None,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn call_zero_valid() {
        let context = SimulationContext::new_with_params([0u8; 16], true, false);

        assert_eq!(
            address(&context, 25, Some(ConditionType::IfZero)).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: false,
                zero: true,
                pc: 25,
                call_addr: Some(1),
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn call_zero_invalid() {
        let context = SimulationContext::new_with_params([0u8; 16], false, false);

        assert_eq!(
            address(&context, 25, Some(ConditionType::IfZero)).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: false,
                zero: false,
                pc: 1,
                call_addr: None,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn call_carry_valid() {
        let context = SimulationContext::new_with_params([0u8; 16], false, true);

        assert_eq!(
            address(&context, 25, Some(ConditionType::IfCarry)).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: true,
                zero: false,
                pc: 25,
                call_addr: Some(1),
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn call_carry_invalid() {
        let context = SimulationContext::new_with_params([0u8; 16], false, false);

        assert_eq!(
            address(&context, 25, Some(ConditionType::IfCarry)).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: false,
                zero: false,
                pc: 1,
                call_addr: None,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn call_non_carry_valid() {
        let context = SimulationContext::new_with_params([0u8; 16], false, false);

        assert_eq!(
            address(&context, 25, Some(ConditionType::IfNonCarry)).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: false,
                zero: false,
                pc: 25,
                call_addr: Some(1),
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn call_non_carry_invalid() {
        let context = SimulationContext::new_with_params([0u8; 16], true, true);

        assert_eq!(
            address(&context, 25, Some(ConditionType::IfNonCarry)).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: true,
                zero: true,
                pc: 1,
                call_addr: None,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    #[should_panic]
    fn call_invalid() {
        let context = SimulationContext::new_with_params([0u8; 16], false, false);

        assert_eq!(
            address(&context, 1025, None).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: false,
                zero: false,
                pc: 1025,
                call_addr: None,
                ..SimulationUpdate::default()
            }
        );
    }
}
