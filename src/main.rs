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
        JUMP main

        addi:
            LOAD s1, 08
        add_loop:
            COMPARE s1, 00
            JUMP Z, add_end
            ADD s0, 01
            SUB s1, 01
            JUMP add_loop
        add_end:
            RETURN

        main:
            LOAD s0, 00
            CALL addi
            ADD s0, 01
        "#
    .to_string();

    t.tokenize(r.read_buffer_and_split(test_script).get_contents().clone());

    p.parse(t.get_tokens().clone());

    sim.initialize_instructions(p.get_instructions().clone())
        .run()?;

    println!(
        "zero: {}, carry: {}, pc: {}",
        sim.get_zero_flag(),
        sim.get_carry_flag(),
        sim.get_program_counter()
    );

    println!("{0} {0:b}", sim.get_register(0).unwrap());

    Ok(())
}
