use std::{fs::File, io::{BufRead, BufReader}};

fn split_inclusive(input: &str, delimiter: char) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut start = 0;

    for (i, c) in input.char_indices() {
        if c == delimiter {
            // Push the substring before the delimiter
            if start < i {
                tokens.push(input[start..i].to_string());
            }
            // Push the delimiter itself
            tokens.push(input[i..i + 1].to_string());
            // Update the start index to the next character after the delimiter
            start = i + 1;
        }
    }

    // Push the last substring after the last delimiter
    if start < input.len() {
        tokens.push(input[start..].to_string());
    }

    tokens
}

fn remove_after_delimiter(input: String, delimiter: char) -> String {
    if let Some(pos) = input.find(delimiter) {
        input[..pos].to_string() // Return the substring before the delimiter
    } else {
        input // If the delimiter is not found, return the original string
    }
}

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

    pub fn read_and_split(&mut self) -> &mut Reader {
        let file = File::open(self.file.clone())
            .expect("Unable to open the file.");

        let reader = BufReader::new(file);

        // Read all lines in file, convert them into strings (if reader fails, return an empty string), collect them into
        // a String vector.
        let lines: Vec<String> = reader
            .lines()
            .map(|line| line.unwrap_or("".into()))
            .collect();

        for line in lines {
            // Remove all comments from the code.
            let line = remove_after_delimiter(line, ';');

            let mut tokens: Vec<String> = Vec::new();

            // Split each line by whitespace, convert them into strings and collect them into another string Vector.
            let words: Vec<String> = line
                .split_whitespace()
                .map(|word| word.to_lowercase().into())
                .collect();

            // Split each word into tokens now using a comma as delimiter, and keep the comma, using the 'split_inclusive' method.
            for word in words {
                tokens.extend(split_inclusive(word.clone().as_str(), ','))
            }

            self.contents.push(tokens);
        }

        self
    }

    pub fn get_contents(&self) -> &Vec<Vec<String>> {
        &self.contents
    }
}