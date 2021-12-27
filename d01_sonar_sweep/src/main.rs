use std::fs::File;
use std::io::{BufRead, BufReader};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_count_increases() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(7, count_increases(&input));
    }

    #[test]
    fn should_count_increases_by_window() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(5, count_increases_by_window(&input));
    }
}

fn main() {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut input: Vec<u32> = Vec::with_capacity(2000);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let line = line.unwrap().parse::<u32>().unwrap();

        input.push(line)
    }

    println!(
        "number of increases in sonar readings: {}",
        count_increases(&input)
    );
    println!(
        "number of increases in sonar reading windows: {}",
        count_increases_by_window(&input)
    );
}

/// count the number of increases in the sonar readings
fn count_increases(input: &[u32]) -> u32 {
    let mut count: u32 = 0;
    let mut prev = &input[0];

    for depth in input {
        if depth > prev {
            count += 1;
        }

        prev = depth;
    }

    count
}

/// count the number of increases in the sonar readings by sliding window of 3 reading
fn count_increases_by_window(input: &[u32]) -> u32 {
    let mut count: u32 = 0;
    // prev is now the total of a window, starting with the first 3 elements
    let mut prev = input[0] + input[1] + input[2];

    for w in input.windows(3) {
        let depth = w[0] + w[1] + w[2];

        if depth > prev {
            count += 1;
        }

        prev = depth;
    }

    count
}
