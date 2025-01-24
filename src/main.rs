use interpreter::reader::{Reader};

pub mod interpreter;

fn main() {
    let mut r = Reader::new(
        "test.txt"
    );

    r.read_and_parse();

    for (i, l) in r.get_contents().iter().enumerate() {
        print!("{}: ", i);
        for word in l {
            print!("{} ", word);
        }
        println!("")
    }
}
