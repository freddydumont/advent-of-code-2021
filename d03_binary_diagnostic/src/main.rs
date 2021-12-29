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

fn calculate_power_consumption(input: &[String]) -> u32 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_power_consumption() {
        let input = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        assert_eq!(198, calculate_power_consumption(&input));
    }
}
