use interpreter::{reader::*, tokenizer::*, parser::*};

pub mod interpreter;

fn main() {
    let mut r = Reader::new(
        "test.txt"
    );

    let mut t = Tokenizer::new();
    
    let mut p = Parser::new();

    t.tokenize(r.read_and_split().get_contents().clone());
    p.parse(t.get_tokens().clone());

    for instr in p.get_instructions() {
        println!("{:?}", instr);
    }
}
