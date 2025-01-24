use interpreter::{reader::*, tokenizer::*};

pub mod interpreter;

fn main() {
    let mut r = Reader::new(
        "test.txt"
    );

    let mut t = Tokenizer::new();

    t.tokenize(r.read_and_parse().get_contents().clone());

    for token in t.get_tokens() {
        println!("{:?}", token);
    }
}
