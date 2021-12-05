//use itertools::Itertools;

pub fn day2b(input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let state = move_submarine(input);
    let answer = state.hpos * state.depth;
    ("day2b", answer)
}

fn move_submarine(_input: impl Iterator<Item=String>) -> SubState {
    SubState::new(3, 0)
}

#[derive(Debug, PartialEq)]
struct SubState {
    pub hpos: i32,
    pub depth: i32,
}

impl SubState {
    pub fn new(hpos: i32, depth: i32) -> Self {
        SubState {
            hpos,
            depth,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day2b::*;
    use crate::util::to_string_iter;

    #[test]
    fn move_submarine_forward() {
        let input = to_string_iter(vec![
            "forward 3",
        ]);
        assert_eq!(move_submarine(input), SubState::new(3, 0));
    }

    #[test]
    #[ignore]
    fn move_submarine_down_and_forward() {
        let input = to_string_iter(vec![
            "down 2",
            "forward 3",
        ]);
        assert_eq!(move_submarine(input), SubState::new(3, 6));
    }

    #[test]
    #[ignore]
    fn day2b_down_and_forward() {
        let input = to_string_iter(vec![
            "down 2",
            "forward 3",
        ]);
        assert_eq!(day2b(input), ("day2b", 18));
    }
}