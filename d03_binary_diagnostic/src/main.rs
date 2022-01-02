use utils::read_input;

fn main() {
    let input = read_input("src/input.txt", 1000);

    println!("power consumption: {}", calculate_power_consumption(&input));
    println!(
        "life support rating: {}",
        calculate_life_support_rating(&input)
    );
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

enum BitCriteria {
    Zero,
    One,
}

fn reduce_to_decimal(input: &[String], bit: usize, bit_criteria: BitCriteria) -> u32 {
    // base case
    let rows = input.len();
    if rows == 1 {
        return u32::from_str_radix(&input[0], 2).unwrap();
    }

    let mut i = 0;
    let mut ones = 0;
    let mut zeroes = 0;
    while i < rows {
        let chars = input[i].chars().collect::<Vec<char>>();
        if chars[bit] == '1' {
            ones += 1;
        } else {
            zeroes += 1;
        }

        i += 1;
    }

    let to_keep = match bit_criteria {
        BitCriteria::One => {
            if ones >= zeroes {
                '1'
            } else {
                '0'
            }
        }
        BitCriteria::Zero => {
            if zeroes <= ones {
                '0'
            } else {
                '1'
            }
        }
    };

    let mut new_input: Vec<String> = Vec::with_capacity(rows);
    let mut j = 0;
    while j < rows {
        let chars = input[j].chars().collect::<Vec<char>>();
        if chars[bit] == to_keep {
            new_input.push(input[j].to_string());
        }

        j += 1;
    }

    reduce_to_decimal(&new_input, bit + 1, bit_criteria)
}

fn calculate_life_support_rating(input: &[String]) -> u32 {
    reduce_to_decimal(input, 0, BitCriteria::One) * reduce_to_decimal(input, 0, BitCriteria::Zero)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::convert_vec_strings;

    fn input() -> Vec<String> {
        convert_vec_strings(vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ])
    }

    #[test]
    fn should_calculate_power_consumption() {
        assert_eq!(198, calculate_power_consumption(&input()));
    }

    #[test]
    fn should_calculate_life_support_rating() {
        assert_eq!(230, calculate_life_support_rating(&input()));
    }
}
