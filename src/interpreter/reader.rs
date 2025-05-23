use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn split_inclusive(input: &str, delimiter: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut start = 0;

    for (i, c) in input.char_indices() {
        if delimiter.contains(c) {
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

fn squish_between_delimiters(input: String) -> String {
    // In theory, there are only parentheses in Picoblaze assembly.
    // Add other delimiters just in case.
    let opening_delimiter = vec!['(', '[', '{'];
    let closing_delimiter = vec![')', ']', '}'];

    let mut result = String::new();

    let mut is_inside = false;
    for c in input.chars() {
        if opening_delimiter.contains(&c) {
            is_inside = true;
        } else if closing_delimiter.contains(&c) {
            is_inside = false;
        }

        if !is_inside {
            result.push(c);
        } else if !c.is_whitespace() {
            result.push(c);
        }
    }

    result
}

#[derive(Debug)]
pub struct Reader {
    contents: Vec<Vec<String>>,
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            contents: Vec::new(),
        }
    }

    pub fn read_buffer_and_split(&mut self, buffer: String) -> &mut Reader {
        let lines: Vec<String> = buffer.lines().map(|line| line.to_string()).collect();

        self.read_lines(&lines);

        self
    }

    pub fn read_file_and_split(&mut self, file: String) -> &mut Reader {
        let file = File::open(file.clone()).expect("Unable to open the file.");

        let reader = BufReader::new(file);

        // Read all lines in file, convert them into strings (if reader fails, return an empty string), collect them into
        // a String vector.
        let lines: Vec<String> = reader
            .lines()
            .map(|line| line.unwrap_or("".into()))
            .collect();

        self.read_lines(&lines);

        self
    }

    fn read_lines(&mut self, lines: &Vec<String>) {
        for line in lines {
            if line.is_empty() || line.chars().all(|c| c.is_whitespace()) {
                continue;
            }

            // Remove all comments from the code and squish tokens.
            let line = squish_between_delimiters(remove_after_delimiter(line.clone(), ';'));

            let mut tokens: Vec<String> = Vec::new();

            // Split each line by whitespace, convert them into strings and collect them into another string Vector.
            let words: Vec<String> = line
                .split_whitespace()
                .map(|word| word.to_lowercase().into())
                .collect();

            // Split each word into tokens now using a comma as delimiter, and keep the comma, using the 'split_inclusive' method.
            for word in words {
                tokens.extend(split_inclusive(word.clone().as_str(), ",()~"))
            }

            self.contents.push(tokens);
        }
    }

    pub fn get_contents(&self) -> &Vec<Vec<String>> {
        &self.contents
    }
}
