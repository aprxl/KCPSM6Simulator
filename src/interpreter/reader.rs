use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Debug)]
pub struct Reader {
    file: String,
    contents: Vec<Vec<String>>
}

impl Reader {
    pub fn new(file: &str) -> Reader {
        Reader {
            file: file.into(),
            contents: Vec::new()
        }
    }

    pub fn read_and_parse(&mut self) {
        // Open file and handle possible error.
        let file = File::open(self.file.clone())
            .expect("Unable to open the file.");

        // Create a new file reader.
        let reader = BufReader::new(file);

        // Read all lines in file, convert them into strings (if reader fails, return an empty string), collect them into
        // a String vector.
        let lines: Vec<String> = reader
            .lines()
            .map(|line| line.unwrap_or("".into()))
            .collect();

        // Iterate line by line.
        for line in lines {
            // Split each line by whitespace, convert them into strings and collect them into another string Vector.
            let words: Vec<String> = line
                .split_whitespace()
                .map(|word| word.into())
                .collect();

            self.contents.push(words);
        }
    }

    pub const fn get_contents(&self) -> &Vec<Vec<String>> {
        &self.contents
    }
}