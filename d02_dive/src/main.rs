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
    struct Coordinates {
        x: i32,
        y: i32,
    }

    let mut coordinates = Coordinates { x: 0, y: 0 };

    for command in input {
        let instruction: Vec<&str> = command.split(' ').collect();
        let n = instruction[1].parse::<i32>().unwrap();

        match instruction[0] {
            "forward" => coordinates.x += n,
            "down" => coordinates.y += n,
            "up" => coordinates.y -= n,
            _ => panic!("Error, invalid input."),
        };
    }

    coordinates.x * coordinates.y
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
