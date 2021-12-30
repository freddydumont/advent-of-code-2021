use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

/// converts Vec<&str> to Vec<String>
pub fn convert_vec_strings(input: Vec<&str>) -> Vec<String> {
    input.iter().map(|x| x.to_string()).collect::<Vec<String>>()
}

pub fn read_input<T: FromStr>(path: &str, capacity: usize) -> Vec<T> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut input: Vec<T> = Vec::with_capacity(capacity);

    for (i, line) in reader.lines().enumerate() {
        let line = line
            .unwrap()
            .parse::<T>()
            .unwrap_or_else(|_| panic!("Error reading line {} of input", i));

        input.push(line);
    }

    input
}
