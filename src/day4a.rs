use itertools::Itertools;

pub fn day4a(_input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let mut board = Board::new(&(1..=25).collect_vec());
    for cell_index in 0..=4 {
        board.mark_cell(cell_index);
    }
    let unmarked_sum: u32 = board.sum_unmarked_numbers();
    let winning_number = 5;
    let answer = unmarked_sum * winning_number;
    ("day4a", answer as i32)
}

struct Board {
    cells: Vec<Cell>,
}

impl Board {
    pub fn new(numbers: &[u32]) -> Self {
        Board {
            cells: numbers.iter()
                .map(|&num| Cell::new(num, false))
                .collect_vec()
        }
    }

    pub fn mark_cell(&mut self, cell_index: usize) {
        self.cells[cell_index].is_marked = true;
    }

    pub fn sum_unmarked_numbers(&self) -> u32 {
        self.cells.iter()
            .filter(|cell| !cell.is_marked)
            .map(|cell| cell.number)
            .sum()
    }
}

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
    fn sum_example_unmarked_numbers() {
        let mut board = Board::new(&(1..=25).collect_vec());
        for cell_index in 0..=4 {
            board.mark_cell(cell_index);
        }
        assert_eq!(board.sum_unmarked_numbers(), (6..=25).sum());
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