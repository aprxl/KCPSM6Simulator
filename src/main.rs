use interpreter::{interpreter::*, parser::*, reader::*, tokenizer::*};

pub mod interpreter;

fn main() {
    let mut r = Reader::new();
    let mut t = Tokenizer::new();
    let mut p = Parser::new();
    let mut sim = SimulationContext::new();

    let test_script = r#"
        namereg s1, t
        main:
            AND t, s2
        ta:
            XOR s2, 01
    "#
    .to_string();

    t.tokenize(r.read_buffer_and_split(test_script).get_contents().clone());

    p.parse(t.get_tokens().clone());

    sim.initialize_instructions(p.get_instructions().clone())
        .run();
}
