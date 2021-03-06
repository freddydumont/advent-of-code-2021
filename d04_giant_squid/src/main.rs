use utils::read_input;

/// Parse the first line of input into a vec of numbers
fn get_numbers(input: &str) -> Vec<u8> {
    input
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
}

#[derive(Clone, Copy, Debug)]
struct Number {
    digits: u8,
    was_drawn: bool,
}

type Board = [[Number; 5]; 5];
/// Parse the input to return a vec of Boards
fn get_boards(input: &[String]) -> Vec<Board> {
    // each board is 6 lines, including the line break
    let board_count = (input.len() - 1) / 6;

    let line = [Number {
        // default to a 3 digit number to check if initialized with input
        digits: 255,
        was_drawn: false,
    }; 5];
    let mut boards: Vec<Board> = vec![[line; 5]; board_count];

    let mut input_index = 2;
    let mut board_index = 0;
    let mut line_index = 0;

    while input_index < input.len() {
        // when board is fully initialized with input values, start next board
        if boards[board_index][4][4].digits != 255 {
            board_index += 1;
            line_index = 0;
        }

        // skip linebreaks in input
        // note also filter_map which makes sure double spaces are ignored
        if !input[input_index].is_empty() {
            let parsed_line = input[input_index]
                .split(' ')
                .filter_map(|x| x.parse::<u8>().ok())
                .collect::<Vec<u8>>();

            let line = &mut boards[board_index][line_index];

            for (i, num) in line.iter_mut().enumerate() {
                num.digits = parsed_line[i];
            }

            line_index += 1;
        }

        input_index += 1;
    }

    boards
}

fn main() {
    let input = read_input::<String>("src/input.txt", 601);
    let numbers = get_numbers(&input[0]);
    let mut boards = get_boards(&input);

    println!("bingo = {:?}", bingo(&numbers, &mut boards));
}

fn mark_and_check_win(board: &mut Board, number: u8, i: usize) -> bool {
    let mut horizontal = 0;
    let mut vertical = [0; 5];

    for line in board {
        if horizontal == 5 {
            break;
        }

        horizontal = 0;

        for (i, num) in line.iter_mut().enumerate() {
            if num.digits == number {
                num.was_drawn = true;
            }

            if num.was_drawn {
                horizontal += 1;
                vertical[i] += 1;
            }
        }
    }

    // only check for validity when at least 5 numbers are drawn
    if i < 4 {
        return false;
    }

    horizontal == 5 || vertical.contains(&5)
}

fn calculate_score(board: &Board, final_number: u8) -> u32 {
    let mut sum = 0;

    for line in board {
        for num in line {
            if !num.was_drawn {
                sum += num.digits as u32;
            }
        }
    }

    sum * final_number as u32
}

fn bingo(numbers: &[u8], boards: &mut [Board]) -> u32 {
    let mut score = 0;
    // for each number
    'outer: for (i, number) in numbers.iter().enumerate() {
        for board in boards.iter_mut() {
            if mark_and_check_win(board, *number, i) {
                score = calculate_score(board, *number);
                break 'outer;
            }
        }
    }

    score
}

fn let_squid_win(numbers: &[u8], boards: &mut [Board]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::convert_vec_strings;

    fn input() -> Vec<String> {
        convert_vec_strings(vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ])
    }

    fn column_win() -> Vec<String> {
        convert_vec_strings(vec![
            "22,8,21,6,1,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ])
    }

    #[test]
    fn should_play_bingo() {
        let input = input();
        let numbers = get_numbers(&input[0]);
        let mut boards = get_boards(&input);

        assert_eq!(4512, bingo(&numbers, &mut boards));
    }

    #[test]
    fn should_check_columns() {
        let input = column_win();
        let numbers = get_numbers(&input[0]);
        let mut boards = get_boards(&input);

        assert_eq!(242, bingo(&numbers, &mut boards));
    }

    fn should_let_squid_win() {
        let input = input();
        let numbers = get_numbers(&input[0]);
        let mut boards = get_boards(&input);

        assert_eq!(1924, let_squid_win(&numbers, &mut boards));
    }
}
