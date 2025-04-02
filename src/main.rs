use interpreter::{interpreter::*, parser::*, reader::*, tokenizer::*};

#[path = "interpreter/mod.rs"]
pub mod interpreter;

#[path = "interpreter/instructions/mod.rs"]
pub mod instructions;

fn main() -> std::io::Result<()> {
    let mut r = Reader::new();
    let mut t = Tokenizer::new();
    let mut p = Parser::new();
    let mut sim = SimulationContext::new();

    let test_script = r#"
        NAMEREG sE, arg1
        NAMEREG sF, arg2
        NAMEREG sD, temp

        JUMP main

        mult:
            LOAD temp, arg1
        mult_loop:
            SUB arg2, 01
            JUMP Z, mult_end
            ADD arg1, temp
            JUMP mult_loop
        mult_end:
            RETURN

        main:
            LOAD arg1, 08
            LOAD arg2, 06
            CALL mult
        "#
    .to_string();

    t.tokenize(
        r.read_file_and_split("tests/test.s".to_string())
            .get_contents()
            .clone(),
    );

    p.parse(t.get_tokens().clone());

    sim.initialize_instructions(p.get_instructions().clone())
        .run()?;

    println!(
        "zero: {}, carry: {}, pc: {}",
        sim.get_zero_flag(),
        sim.get_carry_flag(),
        sim.get_program_counter()
    );

    println!("arg1: {0} {0:b}", sim.get_register(14).unwrap());

    Ok(())
}
