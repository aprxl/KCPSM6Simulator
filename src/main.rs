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

    t.tokenize(
        r.read_file_and_split("tests/test.s".to_string())
            .get_contents()
            .clone(),
    );

    p.parse(t.get_tokens().clone());

    sim.initialize_instructions(p.get_instructions().clone())
        .run()?;

    println!("{}", sim.get_register(0).unwrap());

    Ok(())
}
