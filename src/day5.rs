use std::ops::RangeInclusive;

pub fn day5a(input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let mut loc_grid = LocationGrid::new();
    input
        .map(|line| parse_input_line(&line))
        .for_each(|vent_line| loc_grid.add_non_diagonal_vent_line(vent_line));
    let answer = loc_grid.num_dangerous_locs();
    ("day5a", answer as i32)
}

pub fn day5b(input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let mut loc_grid = LocationGrid::new();
    input
        .map(|line| parse_input_line(&line))
        .for_each(|vent_line| loc_grid.add_vent_line(vent_line));
    let answer = loc_grid.num_dangerous_locs();
    ("day5b", answer as i32)
}

fn parse_input_line(line: &str) -> LineSegment {
    let mut nums = line
        .split(" -> ")
        .map(|loc_str| loc_str.split(","))
        .flatten()
        .map(|num_str| num_str.parse::<u32>().unwrap());
    LineSegment(Loc::new(nums.next().unwrap(), nums.next().unwrap()),
                Loc::new(nums.next().unwrap(), nums.next().unwrap()))
}

fn increasing_inclusive_range(v1: u32, v2: u32) -> RangeInclusive<u32> {
    if v1 <= v2 {
        v1..=v2
    } else {
        v2..=v1
    }
}

struct LocationGrid {
    rows: Vec<Vec<u16>>,
    num_dangerous_locs: u32,
}

impl LocationGrid {
    pub fn new() -> Self {
        LocationGrid {
            rows: vec![],
            num_dangerous_locs: 0,
        }
    }

    pub fn add_vent_line(&mut self, vent_line: LineSegment) {
        if vent_line.is_diagonal() {
            let locs = vent_line.diagonal_line_segment_locs();
            locs.for_each(|loc| self.add_vent(&loc));
        } else {
            self.add_non_diagonal_vent_line(vent_line);
        }
    }

    pub fn add_non_diagonal_vent_line(&mut self, vent_line: LineSegment) {
        if vent_line.0.y == vent_line.1.y {
            let locs = vent_line.horizontal_line_segment_locs();
            locs.for_each(|loc| self.add_vent(&loc));
        } else if vent_line.0.x == vent_line.1.x {
            let locs = vent_line.vertical_line_segment_locs();
            locs.for_each(|loc| self.add_vent(&loc));
        }
    }

    fn add_vent(&mut self, loc: &Loc) {
        let row_index = loc.y as usize;
        let col_index = loc.x as usize;
        self.expand_grid_if_needed(row_index, col_index);
        self.increment_vent_count(row_index, col_index)
    }

    fn expand_grid_if_needed(&mut self, row_index: usize, col_index: usize) {
        if self.rows.len() < row_index + 1 {
            self.rows.resize(row_index + 1, vec![]);
        }
        let row = &mut self.rows[row_index];
        if row.len() < col_index + 1 {
            row.resize(col_index + 1, 0);
        }
    }

    fn increment_vent_count(&mut self, row_index: usize, col_index: usize) {
        let cell = &mut self.rows[row_index][col_index];
        *cell += 1;
        if *cell == 2 {
            self.num_dangerous_locs += 1;
        }
    }

    pub fn num_dangerous_locs(&self) -> u32 {
        self.num_dangerous_locs
    }
}

#[derive(Debug, PartialEq)]
struct LineSegment(Loc, Loc);

impl LineSegment {
    pub fn is_diagonal(&self) -> bool {
        self.0.x != self.1.x && self.0.y != self.1.y
    }

    pub fn horizontal_line_segment_locs(&self) -> impl Iterator<Item=Loc> {
        let y = self.0.y;
        increasing_inclusive_range(self.0.x, self.1.x)
            .map(move |x| Loc::new(x, y))
    }

    pub fn vertical_line_segment_locs(&self) -> impl Iterator<Item=Loc> {
        let x = self.0.x;
        increasing_inclusive_range(self.0.y, self.1.y)
            .map(move |y| Loc::new(x, y))
    }

    pub fn diagonal_line_segment_locs(&self) -> impl Iterator<Item=Loc> {
        increasing_inclusive_range(self.0.x, self.1.x)
            .zip(increasing_inclusive_range(self.0.y, self.1.y))
            .map(|(x, y)| Loc::new(x, y))
    }
}

#[derive(Debug, PartialEq)]
struct Loc {
    pub x: u32,
    pub y: u32,
}

impl Loc {
    pub fn new(x: u32, y: u32) -> Self {
        Loc { x, y }
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::*;
    use crate::util::to_string_iter;

    #[test]
    fn parse_example_input_line() {
        assert_eq!(
            parse_input_line("957,596 -> 35,182"),
            LineSegment(Loc::new(957, 596), Loc::new(35, 182))
        );
    }

    #[test]
    fn location_grid_counts_dangerous_locs_for_simple_input_5a() {
        // . 1
        // 1 2
        let mut loc_grid = LocationGrid::new();
        loc_grid.add_vent_line(LineSegment(Loc::new(0, 1), Loc::new(1, 1)));
        loc_grid.add_vent_line(LineSegment(Loc::new(1, 1), Loc::new(1, 0)));
        assert_eq!(loc_grid.num_dangerous_locs(), 1);
    }

    #[test]
    fn location_grid_counts_each_dangerous_loc_only_once() {
        let mut loc_grid = LocationGrid::new();
        loc_grid.add_vent_line(LineSegment(Loc::new(0, 1), Loc::new(2, 1)));
        loc_grid.add_vent_line(LineSegment(Loc::new(1, 0), Loc::new(1, 1)));
        loc_grid.add_vent_line(LineSegment(Loc::new(1, 1), Loc::new(1, 2)));
        assert_eq!(loc_grid.num_dangerous_locs(), 1);
    }

    #[test]
    fn location_grid_expands_as_needed() {
        let mut loc_grid = LocationGrid::new();
        loc_grid.add_vent_line(LineSegment(Loc::new(900, 901), Loc::new(901, 901)));
        loc_grid.add_vent_line(LineSegment(Loc::new(901, 901), Loc::new(901, 900)));
        assert_eq!(loc_grid.num_dangerous_locs(), 1);
    }

    #[test]
    fn location_grid_does_not_shrink() {
        let mut loc_grid = LocationGrid::new();
        loc_grid.add_vent_line(LineSegment(Loc::new(2, 2), Loc::new(2, 3)));
        loc_grid.add_vent_line(LineSegment(Loc::new(0, 0), Loc::new(0, 1)));
        loc_grid.add_vent_line(LineSegment(Loc::new(2, 2), Loc::new(2, 3)));
        assert_eq!(loc_grid.num_dangerous_locs(), 2);
    }

    #[test]
    fn location_grid_adds_long_row() {
        let mut loc_grid = LocationGrid::new();
        loc_grid.add_vent_line(LineSegment(Loc::new(0, 0), Loc::new(5, 0)));
        loc_grid.add_vent_line(LineSegment(Loc::new(2, 0), Loc::new(2, 1)));
        assert_eq!(loc_grid.num_dangerous_locs(), 1);
    }

    #[test]
    fn location_grid_adds_long_row_reversed() {
        let mut loc_grid = LocationGrid::new();
        loc_grid.add_vent_line(LineSegment(Loc::new(5, 0), Loc::new(0, 0)));
        loc_grid.add_vent_line(LineSegment(Loc::new(2, 0), Loc::new(2, 1)));
        assert_eq!(loc_grid.num_dangerous_locs(), 1);
    }

    #[test]
    fn location_grid_adds_long_column() {
        let mut loc_grid = LocationGrid::new();
        loc_grid.add_vent_line(LineSegment(Loc::new(0, 0), Loc::new(0, 5)));
        loc_grid.add_vent_line(LineSegment(Loc::new(0, 2), Loc::new(1, 2)));
        assert_eq!(loc_grid.num_dangerous_locs(), 1);
    }

    #[test]
    fn location_grid_adds_long_column_reversed() {
        let mut loc_grid = LocationGrid::new();
        loc_grid.add_vent_line(LineSegment(Loc::new(0, 5), Loc::new(0, 0)));
        loc_grid.add_vent_line(LineSegment(Loc::new(0, 2), Loc::new(1, 2)));
        assert_eq!(loc_grid.num_dangerous_locs(), 1);
    }

    #[test]
    fn location_grid_5a_ignores_diagonal() {
        let mut loc_grid = LocationGrid::new();
        loc_grid.add_non_diagonal_vent_line(LineSegment(Loc::new(0, 0), Loc::new(0, 1)));
        loc_grid.add_non_diagonal_vent_line(LineSegment(Loc::new(0, 0), Loc::new(1, 1)));
        assert_eq!(loc_grid.num_dangerous_locs(), 0);
    }

    #[test]
    fn location_grid_5b_adds_two_loc_diagonal() {
        let mut loc_grid = LocationGrid::new();
        loc_grid.add_vent_line(LineSegment(Loc::new(0, 0), Loc::new(0, 1)));
        loc_grid.add_vent_line(LineSegment(Loc::new(0, 0), Loc::new(1, 1)));
        assert_eq!(loc_grid.num_dangerous_locs(), 1);
    }

    #[test]
    fn location_grid_5b_adds_long_decreasing_diagonal() {
        let mut loc_grid = LocationGrid::new();
        loc_grid.add_vent_line(LineSegment(Loc::new(5, 5), Loc::new(0, 0)));
        loc_grid.add_vent_line(LineSegment(Loc::new(0, 2), Loc::new(5, 2)));
        assert_eq!(loc_grid.num_dangerous_locs(), 1);
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

    #[test]
    #[ignore]
    fn example_input_5b() {
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
        assert_eq!(day5b(input), ("day5b", 12));
    }
}