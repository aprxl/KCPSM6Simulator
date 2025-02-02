use interpreter::{parser::*, reader::*, tokenizer::*};

pub mod interpreter;

fn main() {
    let mut r = Reader::new("test.txt");

    let mut t = Tokenizer::new();

    let mut p = Parser::new();

    t.tokenize(r.read_and_split().get_contents().clone());
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
}
