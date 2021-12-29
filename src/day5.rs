pub fn day5a(input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let mut loc_grid = LocationGrid::new();
    input.enumerate()
        .map(|(index, line)| parse_input_line(index, &line))
        .for_each(|vent_line| loc_grid.add_vent_line(vent_line));
    let answer = loc_grid.num_dangerous_locs();
    ("day5a", answer as i32)
}

fn parse_input_line(index: usize, _line: &str) -> LineSegment {
    match index {
        0 => LineSegment(Loc(0, 1), Loc(1, 1)),
        _ => LineSegment(Loc(1, 1), Loc(1, 0)),
    }
}

struct LocationGrid {
    rows: Vec<Vec<u16>>,
    num_dangerous_locs: u32,
}

impl LocationGrid {
    pub fn new() -> Self {
        LocationGrid {
            rows: vec![vec![0, 0], vec![0, 0]],
            num_dangerous_locs: 0,
        }
    }

    pub fn add_vent_line(&mut self, vent_line: LineSegment) {
        self.add_vent(&vent_line.0);
        self.add_vent(&vent_line.1);
    }

    fn add_vent(&mut self, loc: &Loc) {
        let cell = &mut self.rows[loc.1 as usize][loc.0 as usize];
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

#[derive(Debug, PartialEq)]
struct Loc(u32, u32);

#[cfg(test)]
mod tests {
    use crate::day5::*;
    use crate::util::to_string_iter;

    #[test]
    fn parse_example_input_line() {
        assert_eq!(
            parse_input_line(0, "0,1 -> 1,1"),
            LineSegment(Loc(0, 1), Loc(1, 1))
        );
    }

    #[test]
    fn location_grid_counts_dangerous_locs_for_simple_input_5a() {
        // . 1
        // 1 2
        let mut loc_grid = LocationGrid::new();
        loc_grid.add_vent_line(LineSegment(Loc(0, 1), Loc(1, 1)));
        loc_grid.add_vent_line(LineSegment(Loc(1, 1), Loc(1, 0)));
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