use std::fs::File;
use std::io::{BufRead, BufReader};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_count_increases() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(7, count_increases(input));
    }
}

fn main() {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut input: Vec<u32> = vec![];

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let line = line.unwrap().parse::<u32>().unwrap();

        input.push(line)
    }

    println!("{}", count_increases(input));
}

// count the number of increases in the sonar readings
fn count_increases(input: Vec<u32>) -> u32 {
    let mut count: u32 = 0;
    let mut prev = input[0];

    for depth in input {
        if depth > prev {
            count += 1;
        }

        prev = depth;
    }

    count
}
