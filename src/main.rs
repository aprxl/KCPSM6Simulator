use interpreter::{parser::*, reader::*, tokenizer::*};

pub mod interpreter;

fn main() {
    let mut r = Reader::new();
    let mut t = Tokenizer::new();
    let mut p = Parser::new();

    let test_script = r#"
        main:
            AND s1, ~s2
    "#
    .to_string();

    t.tokenize(r.read_buffer_and_split(test_script).get_contents().clone());

    p.parse(t.get_tokens().clone());

    for (addr, instr) in p.get_instructions() {
        println!("0x{:x} ({}) {:?}", addr, addr, instr);
    }

    for label in p.get_labels() {
        println!("{:?}", label);
    }

    for con in p.get_constants() {
        println!("{:?}", con);
    }

    for al in p.get_aliases() {
        println!("{:?}", al);
    }
}
