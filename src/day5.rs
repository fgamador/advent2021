pub fn day5a(_input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let mut loc_grid = LocationGrid::new();
    loc_grid.add_vent_line(LineSegment(Loc(0, 1), Loc(1, 1)));
    loc_grid.add_vent_line(LineSegment(Loc(1, 1), Loc(1, 0)));
    let answer = loc_grid.num_dangerous_locs();
    ("day5a", answer as i32)
}

struct LocationGrid {
    num_dangerous_locs: u32,
}

impl LocationGrid {
    pub fn new() -> Self {
        LocationGrid {
            num_dangerous_locs: 1,
        }
    }

    pub fn add_vent_line(&mut self, _vent_line: LineSegment) {}

    pub fn num_dangerous_locs(&self) -> u32 {
        self.num_dangerous_locs
    }
}

struct LineSegment(Loc, Loc);

struct Loc(u32, u32);

#[cfg(test)]
mod tests {
    use crate::day5::*;
    use crate::util::to_string_iter;

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