pub fn day5a(_input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let mut vent_cells = VentCells::new();
    let answer = vent_cells.get_dangerous_vent_cell_count();
    ("day5a", answer as i32)
}

struct VentCells {
    cell_counts: CellValueCounts,
}

impl VentCells {
    pub fn new() -> Self {
        VentCells {
            cell_counts: CellValueCounts::new(),
        }
    }

    pub fn get_dangerous_vent_cell_count(&mut self) -> u32 {
        vec![1, 1, 1, 2].into_iter().for_each(|cell_value|
            self.cell_counts.add_cell_value(cell_value));
        self.cell_counts.get_dangerous_cell_value_count()
    }
}

struct CellValueCounts {
    dangerous_cell_value_count: u32,
}

impl CellValueCounts {
    pub fn new() -> Self {
        CellValueCounts {
            dangerous_cell_value_count: 0,
        }
    }

    pub fn add_cell_value(&mut self, value: u16) {
        if value == 2 {
            // If value > 2, it was already counted when value == 2.
            self.dangerous_cell_value_count += 1;
        }
    }

    pub fn get_dangerous_cell_value_count(&self) -> u32 {
        self.dangerous_cell_value_count
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::*;
    use crate::util::to_string_iter;

    #[test]
    fn cell_value_counts_of_1_are_not_dangerous() {
        let mut cell_counts = CellValueCounts::new();
        vec![1, 1].into_iter().for_each(|cell_value|
            cell_counts.add_cell_value(cell_value));
        assert_eq!(cell_counts.get_dangerous_cell_value_count(), 0);
    }

    #[test]
    fn dangerous_cells_are_counted_only_once() {
        let mut cell_counts = CellValueCounts::new();
        vec![1, 1, 2, 2, 3, 3].into_iter().for_each(|cell_value|
            cell_counts.add_cell_value(cell_value));
        assert_eq!(cell_counts.get_dangerous_cell_value_count(), 2);
    }

    #[test]
    fn simple_cell_value_counts() {
        let mut cell_counts = CellValueCounts::new();
        vec![1, 1, 1, 2].into_iter().for_each(|cell_value|
            cell_counts.add_cell_value(cell_value));
        assert_eq!(cell_counts.get_dangerous_cell_value_count(), 1);
    }

    #[test]
    fn simple_input_5a() {
        // . 1
        // 1 2
        let input = to_string_iter(vec![
            "0,1 -> 1,1", // row 1, rightward
            "1,1 -> 1,0", // column 1, upward
        ]);
        assert_eq!(day5a(input), ("day5a", 1));
    }

    #[test]
    #[ignore]
    fn example_input_5a() {
        let input = to_string_iter(vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]);
        assert_eq!(day5a(input), ("day5a", 5));
    }
}