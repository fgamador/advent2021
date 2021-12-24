use std::vec;
use itertools::Itertools;

pub fn day4a(mut input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let numbers = parse_the_numbers(&input.next().unwrap());
    let mut boards = read_boards(input);
    if let Some((winning_board, winning_number)) = play_boards(&mut boards, &numbers) {
        let winning_score = winning_board.sum_unmarked_numbers();
        let answer = winning_score * winning_number;
        ("day4a", answer as i32)
    } else {
        panic!("No winning board");
    }
}

fn parse_the_numbers(_the_numbers: &str) -> Vec<u32> {
    vec![1, 2, 3, 4, 5]
}

fn read_boards(mut input: impl Iterator<Item=String>) -> Vec<Board> {
    let mut boards = vec![];
    while input.next().is_some() {
        boards.push(read_board(&mut input));
    }
    boards
}

fn read_board(input: &mut impl Iterator<Item=String>) -> Board {
    let mut numbers: Vec<u32> = Vec::with_capacity(25);
    for _i in 1..=5 {
        input.next().unwrap()
            .split_whitespace()
            .map(|digits| digits.parse().unwrap())
            .for_each(|num| numbers.push(num));
    }
    Board::new(&numbers)
}

fn play_boards<'a>(boards: &'a mut Vec<Board>, numbers: &[u32]) -> Option<(&'a Board, u32)> {
    for number in numbers {
        for (board_index, cell_index) in find_boards_containing_number(number) {
            boards[board_index].mark_cell(cell_index);
            if boards[board_index].is_cell_in_fully_marked_row(cell_index) || boards[board_index].is_cell_in_fully_marked_column(cell_index) {
                return Some((&boards[board_index], 5));
            }
        }
    }
    None
}

fn find_boards_containing_number(number: &u32) -> Vec<(usize, usize)> {
    vec![(0, (number - 1) as usize)]
}

#[derive(Debug, PartialEq)]
struct Board {
    cells: Vec<Cell>,
}

impl Board {
    pub fn new(numbers: &[u32]) -> Self {
        assert_eq!(numbers.len(), 5 * 5);
        Board {
            cells: numbers.iter()
                .map(|&num| Cell::new(num, false))
                .collect_vec()
        }
    }

    pub fn mark_cell(&mut self, cell_index: usize) {
        self.cells[cell_index].is_marked = true;
    }

    pub fn is_cell_in_fully_marked_row(&self, cell_index: usize) -> bool {
        let row_first_index = (cell_index / 5) * 5;
        (row_first_index..row_first_index + 5).all(|index| self.cells[index].is_marked)
    }

    pub fn is_cell_in_fully_marked_column(&self, _cell_index: usize) -> bool {
        false
    }

    pub fn sum_unmarked_numbers(&self) -> u32 {
        self.cells.iter()
            .filter(|cell| !cell.is_marked)
            .map(|cell| cell.number)
            .sum()
    }
}

#[derive(Debug, PartialEq)]
struct Cell {
    number: u32,
    is_marked: bool,
}

impl Cell {
    pub fn new(number: u32, is_marked: bool) -> Self {
        Cell { number, is_marked }
    }
}

#[cfg(test)]
mod tests {
    use crate::day4a::*;
    use crate::util::to_string_iter;

    #[test]
    fn parse_example_numbers() {
        assert_eq!(parse_the_numbers("1,2,3,4,5"), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn read_example_boards() {
        let input = to_string_iter(vec![
            "",
            " 1  2  3  4  5",
            " 6  7  8  9 10",
            "11 12 13 14 15",
            "16 17 18 19 20",
            "21 22 23 24 25",
            "",
            "25 24 23 22 21",
            "20 19 18 17 16",
            "15 14 13 12 11",
            "10  9  8  7  6",
            " 5  4  3  2  1",
        ]);
        let expected = vec![
            Board::new(&(1..=25).collect_vec()),
            Board::new(&(1..=25).rev().collect_vec()),
        ];
        assert_eq!(read_boards(input), expected);
    }

    #[test]
    fn board_knows_when_cell_is_in_fully_marked_row() {
        let mut board = Board::new(&(1..=25).collect_vec());
        vec![5, 7, 8, 9].into_iter().for_each(|cell_index| board.mark_cell(cell_index));
        assert!(!board.is_cell_in_fully_marked_row(7));
        board.mark_cell(6);
        assert!(board.is_cell_in_fully_marked_row(7));
    }

    #[test]
    #[ignore]
    fn board_knows_when_cell_is_in_fully_marked_column() {
        let mut board = Board::new(&(1..=25).collect_vec());
        vec![1, 11, 16, 21].into_iter().for_each(|cell_index| board.mark_cell(cell_index));
        assert!(!board.is_cell_in_fully_marked_column(7));
        board.mark_cell(6);
        assert!(board.is_cell_in_fully_marked_column(7));
    }

    #[test]
    fn play_example_board() {
        let mut boards = vec![Board::new(&(1..=25).collect_vec())];
        let numbers = vec![1, 2, 3, 4, 5];
        if let Some((winning_board, winning_number)) = play_boards(&mut boards, &numbers) {
            assert_eq!(winning_board.sum_unmarked_numbers(), (6..=25).sum());
            assert_eq!(winning_number, 5);
        } else {
            panic!("No winning board");
        }
    }

    #[test]
    fn score_example_board() {
        let mut board = Board::new(&(11..=35).rev().collect_vec());
        board.mark_cell(0);
        board.mark_cell(11);
        board.mark_cell(12);
        board.mark_cell(24);
        assert_eq!(board.sum_unmarked_numbers(), (11..=35).sum::<u32>() - (35 + 24 + 23 + 11));
    }

    #[test]
    fn simple_input() {
        let input = to_string_iter(vec![
            "1,2,3,4,5",
            "",
            " 1  2  3  4  5",
            " 6  7  8  9 10",
            "11 12 13 14 15",
            "16 17 18 19 20",
            "21 22 23 24 25",
        ]);
        let unmarked_sum: i32 = (6..=25).sum();
        let winning_number = 5;
        assert_eq!(day4a(input), ("day4a", unmarked_sum * winning_number));
    }

    #[test]
    #[ignore]
    fn example_input() {
        let input = to_string_iter(vec![
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
        ]);
        assert_eq!(day4a(input), ("day4a", 4512));
    }
}