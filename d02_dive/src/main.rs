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

    println!("position x * y: {}", calculate_position(&input));
}

/// returns the product of the final position's coordinates
fn calculate_position(input: &[String]) -> i32 {
    let mut position: (i32, i32) = (0, 0);

    for command in input {
        let instruction: Vec<&str> = command.split(' ').collect();

        match instruction[0] {
            "forward" => position.0 += instruction[1].parse::<i32>().unwrap(),
            "down" => position.1 += instruction[1].parse::<i32>().unwrap(),
            "up" => position.1 -= instruction[1].parse::<i32>().unwrap(),
            _ => panic!("Error, invalid input."),
        };
    }

    position.0 * position.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_position_by_depth() {
        let input = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];

        assert_eq!(150, calculate_position(&input));
    }
}
