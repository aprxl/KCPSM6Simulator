use crate::{ConditionType, SimulationContext, SimulationUpdate};
use std::io::{Error, ErrorKind};

use super::call;

pub fn default(
    ctx: &SimulationContext,
    condition: Option<ConditionType>,
) -> Result<SimulationUpdate, Error> {
    let mut update = SimulationUpdate::new(ctx);
    let call_stack = ctx.get_call_stack();

    if call_stack.len() <= 0 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "RETURN: Unable to return as call stack is empty!",
        ));
    }

    if let Some(condition) = condition {
        let should_return = match condition {
            ConditionType::IfNonZero => !ctx.get_zero_flag(),
            ConditionType::IfZero => ctx.get_zero_flag(),
            ConditionType::IfNonCarry => !ctx.get_carry_flag(),
            ConditionType::IfCarry => ctx.get_carry_flag(),
        };

        if should_return {
            update.ret_addr = true;
        }
    } else {
        update.ret_addr = true;
    }

    Ok(update)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_unconditional() {
        let mut context = SimulationContext::new_with_params([0u8; 16], false, false);

        context.add_to_call_stack_unrestricted(1);

        assert_eq!(
            default(&context, None).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: false,
                zero: false,
                ret_addr: true,
                pc: 1,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn return_non_zero_valid() {
        let mut context = SimulationContext::new_with_params([0u8; 16], false, false);

        context.add_to_call_stack_unrestricted(1);

        assert_eq!(
            default(&context, Some(ConditionType::IfNonZero)).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: false,
                zero: false,
                ret_addr: true,
                pc: 1,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn return_non_zero_invalid() {
        let mut context = SimulationContext::new_with_params([0u8; 16], true, false);

        context.add_to_call_stack_unrestricted(1);

        assert_eq!(
            default(&context, Some(ConditionType::IfNonZero)).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: false,
                zero: true,
                ret_addr: false,
                pc: 1,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn return_zero_valid() {
        let mut context = SimulationContext::new_with_params([0u8; 16], true, false);

        context.add_to_call_stack_unrestricted(1);

        assert_eq!(
            default(&context, Some(ConditionType::IfZero)).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: false,
                zero: true,
                ret_addr: true,
                pc: 1,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn return_zero_invalid() {
        let mut context = SimulationContext::new_with_params([0u8; 16], false, false);

        context.add_to_call_stack_unrestricted(1);

        assert_eq!(
            default(&context, Some(ConditionType::IfZero)).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: false,
                zero: false,
                ret_addr: false,
                pc: 1,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn return_non_carry_valid() {
        let mut context = SimulationContext::new_with_params([0u8; 16], false, false);

        context.add_to_call_stack_unrestricted(1);

        assert_eq!(
            default(&context, Some(ConditionType::IfNonCarry)).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: false,
                zero: false,
                ret_addr: true,
                pc: 1,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn return_non_carry_invalid() {
        let mut context = SimulationContext::new_with_params([0u8; 16], false, true);

        context.add_to_call_stack_unrestricted(1);

        assert_eq!(
            default(&context, Some(ConditionType::IfNonCarry)).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: true,
                zero: false,
                ret_addr: false,
                pc: 1,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn return_carry_valid() {
        let mut context = SimulationContext::new_with_params([0u8; 16], false, true);

        context.add_to_call_stack_unrestricted(1);

        assert_eq!(
            default(&context, Some(ConditionType::IfCarry)).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: true,
                zero: false,
                ret_addr: true,
                pc: 1,
                ..SimulationUpdate::default()
            }
        );
    }

    #[test]
    fn return_carry_invalid() {
        let mut context = SimulationContext::new_with_params([0u8; 16], false, false);

        context.add_to_call_stack_unrestricted(1);

        assert_eq!(
            default(&context, Some(ConditionType::IfCarry)).unwrap(),
            SimulationUpdate {
                registers: [0u8; 16],
                carry: false,
                zero: false,
                ret_addr: false,
                pc: 1,
                ..SimulationUpdate::default()
            }
        );
    }
}
