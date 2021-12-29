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

    print!("power consumption: {}", calculate_power_consumption(&input))
}

/// takes a list of binary strings as input and returns the product of the gamma and epsilon values
///
/// **gamma**: most common bit in corresponding char of all strings
/// **epsilon**: opposite of gamma
fn calculate_power_consumption(input: &[String]) -> u32 {
    let number_of_bits = input[0].chars().count();
    let mut number_of_ones = vec![0; number_of_bits];
    let mut gamma = vec!['0'; number_of_bits];
    let mut epsilon = vec!['0'; number_of_bits];

    for line in input {
        for (i, binary) in line.chars().enumerate() {
            if binary == '1' {
                number_of_ones[i] += 1;
            }
        }
    }

    for (i, n) in number_of_ones.iter().enumerate() {
        if n > &(input.len() / 2) {
            gamma[i] = '1';
        } else {
            epsilon[i] = '1';
        }
    }

    let gamma_decimal = u32::from_str_radix(&gamma.iter().collect::<String>(), 2).unwrap();
    let epsilon_decimal = u32::from_str_radix(&epsilon.iter().collect::<String>(), 2).unwrap();

    print!("gamma: {}\nepsilon: {}\n", gamma_decimal, epsilon_decimal);

    gamma_decimal * epsilon_decimal
}

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
