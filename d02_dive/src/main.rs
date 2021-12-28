use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut input: Vec<String> = Vec::with_capacity(1000);

    for line in reader.lines() {
        let line = line.unwrap();

        input.push(line)
    }

    print!("{:#?}", input)
}
