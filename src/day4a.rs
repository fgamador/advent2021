use std::collections::HashMap;
use std::vec;
use itertools::Itertools;

pub fn day4a(mut input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let numbers = parse_numbers_csv(&input.next().unwrap());
    let mut boards = read_boards(input);
    if let Some((winning_board_index, winning_number)) = play_to_first_winning_board(&mut boards, &numbers) {
        let winning_score = boards[winning_board_index].sum_unmarked_numbers();
        let answer = winning_score * winning_number;
        ("day4a", answer as i32)
    } else {
        panic!("No winning board");
    }
}

fn parse_numbers_csv(numbers_csv: &str) -> Vec<u32> {
    numbers_csv
        .split(",")
        .map(|digits| digits.parse().unwrap())
        .collect_vec()
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

fn play_to_first_winning_board(boards: &mut Vec<Board>, numbers: &[u32]) -> Option<(usize, u32)> {
    let cell_indexes = build_cell_indexes(boards);
    for &number in numbers {
        let winning_board_index_and_number = play_number_on_all_boards(number, boards, &cell_indexes);
        if winning_board_index_and_number != None {
            return winning_board_index_and_number;
        }
    }
    None
}

fn build_cell_indexes(boards: &[Board]) -> HashMap<u32, Vec<(usize, usize)>> {
    let mut cell_indexes = HashMap::new();
    for (board_index, board) in boards.iter().enumerate() {
        for (cell_index, cell) in board.cells.iter().enumerate() {
            cell_indexes
                .entry(cell.number)
                .or_insert(vec![])
                .push((board_index, cell_index));
        }
    }
    cell_indexes
}

fn play_number_on_all_boards(number: u32, boards: &mut Vec<Board>, cell_indexes: &HashMap<u32, Vec<(usize, usize)>>) -> Option<(usize, u32)> {
    if let Some(indexes) = cell_indexes.get(&number) {
        for &(board_index, cell_index) in indexes {
            if boards[board_index].play_cell(cell_index) {
                return Some((board_index, number));
            }
        }
    }
    None
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

    pub fn play_cell(&mut self, cell_index: usize) -> bool {
        self.mark_cell(cell_index);
        self.is_cell_in_fully_marked_row(cell_index) || self.is_cell_in_fully_marked_column(cell_index)
    }

    pub fn mark_cell(&mut self, cell_index: usize) {
        self.cells[cell_index].is_marked = true;
    }

    pub fn is_cell_in_fully_marked_row(&self, cell_index: usize) -> bool {
        let row_first_index = (cell_index / 5) * 5;
        (row_first_index..row_first_index + 5).all(|index| self.cells[index].is_marked)
    }

    pub fn is_cell_in_fully_marked_column(&self, cell_index: usize) -> bool {
        let col_first_index = cell_index % 5;
        (0..=4).map(|i| col_first_index + i * 5).all(|index| self.cells[index].is_marked)
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
        assert_eq!(parse_numbers_csv("7,4,9,5,11"), vec![7, 4, 9, 5, 11]);
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
    fn build_example_cell_indexes() {
        let boards = vec![
            Board::new(&(1..=25).collect_vec()),
            Board::new(&(11..=35).collect_vec()),
        ];
        let cell_indexes = build_cell_indexes(&boards);
        assert_eq!(cell_indexes.len(), 35);
        assert_eq!(*cell_indexes.get(&1).unwrap(), vec![(0, 0)]);
        assert_eq!(*cell_indexes.get(&11).unwrap(), vec![(0, 10), (1, 0)]);
        assert_eq!(*cell_indexes.get(&35).unwrap(), vec![(1, 24)]);
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
    fn board_knows_when_cell_is_in_fully_marked_column() {
        let mut board = Board::new(&(1..=25).collect_vec());
        vec![1, 11, 16, 21].into_iter().for_each(|cell_index| board.mark_cell(cell_index));
        assert!(!board.is_cell_in_fully_marked_column(11));
        board.mark_cell(6);
        assert!(board.is_cell_in_fully_marked_column(11));
    }

    #[test]
    fn play_missing_number() {
        let mut boards = vec![Board::new(&(1..=25).collect_vec())];
        let numbers = vec![26];
        assert_eq!(play_to_first_winning_board(&mut boards, &numbers), None);
    }

    #[test]
    fn play_example_board_with_row_win() {
        let mut boards = vec![Board::new(&(11..=35).collect_vec())];
        let numbers = vec![1, 16, 2, 18, 20, 19, 17, 22];
        if let Some((winning_board_index, winning_number)) = play_to_first_winning_board(&mut boards, &numbers) {
            assert_eq!(winning_number, 17);
            assert_eq!(boards[winning_board_index].sum_unmarked_numbers(),
                       (11..=35).sum::<u32>() - (16..=20).sum::<u32>());
        } else {
            panic!("No winning board");
        }
    }

    #[test]
    fn play_example_board_with_column_win() {
        let mut boards = vec![Board::new(&(11..=35).collect_vec())];
        let numbers = vec![1, 12, 2, 17, 32, 27, 22, 34];
        if let Some((winning_board_index, winning_number)) = play_to_first_winning_board(&mut boards, &numbers) {
            assert_eq!(winning_number, 22);
            assert_eq!(boards[winning_board_index].sum_unmarked_numbers(),
                       (11..=35).sum::<u32>() - vec![12, 17, 22, 27, 32].iter().sum::<u32>());
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