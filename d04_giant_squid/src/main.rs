use utils::read_input;

/// Parse the first line of input into a vec of numbers
fn get_numbers(input: &str) -> Vec<u8> {
    input
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
}

type Board = Vec<Vec<u8>>;
/// Parse the input to return a vec of Boards
fn get_boards(input: &[String]) -> Vec<Board> {
    // each board is 6 lines, including the line break
    let board_count = (input.len() - 1) / 6;

    let mut boards: Vec<Board> = vec![vec![Vec::with_capacity(5); 5]; board_count];

    let mut input_index = 2;
    let mut board_index = 0;
    let mut line_index = 0;

    while input_index < input.len() {
        // when board is full, start the next one and reset line_index
        if !(boards[board_index].last().unwrap().is_empty()) {
            board_index += 1;
            line_index = 0;
        }

        // skip linebreaks in input
        // note also filter_map which makes sure double spaces are ignored
        if !input[input_index].is_empty() {
            boards[board_index][line_index] = input[input_index]
                .split(' ')
                .filter_map(|x| x.parse::<u8>().ok())
                .collect::<Vec<u8>>();

            line_index += 1;
        }

        input_index += 1;
    }

    boards
}

fn main() {
    let input = read_input::<String>("src/input.txt", 601);
    let numbers = get_numbers(&input[0]);
    let boards = get_boards(&input);

    bingo(&numbers, &boards);
}

fn bingo(numbers: &[u8], boards: &[Board]) -> u32 {
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

    #[test]
    fn should_play_bingo() {
        let input = input();
        let numbers = get_numbers(&input[0]);
        let boards = get_boards(&input);

        assert_eq!(4512, bingo(&numbers, &boards));
    }
}
