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
        main:
            LOAD s1,    00001111'b
            COMPARECY s1, 00001111'b
    "#
    .to_string();

    t.tokenize(r.read_buffer_and_split(test_script).get_contents().clone());

    p.parse(t.get_tokens().clone());

    sim.initialize_instructions(p.get_instructions().clone())
        .run()?;

    println!(
        "zero: {}, carry: {}",
        sim.get_zero_flag(),
        sim.get_carry_flag()
    );

    println!("{0} {0:b}", sim.get_register(1).unwrap());

    Ok(())
}
