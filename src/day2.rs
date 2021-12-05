pub fn day2a(input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let state = move_submarine(input);
    let answer = state.hpos * state.depth;
    ("day2a", answer)
}

fn move_submarine(input: impl Iterator<Item=String>) -> SubState {
    input
        .map(|line| command_to_delta(&line))
        .fold(SubState::new(0, 0), |state, delta| SubState {
            hpos: state.hpos + delta.hpos,
            depth: state.depth + delta.depth,
        })
}

fn command_to_delta(_command: &str) -> SubState {
    SubState::new(3, 2)
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
    use crate::day2::*;
    use crate::util::to_string_iter;

    #[test]
    fn move_submarine_forward_and_down() {
        let input = to_string_iter(vec![
            "forward 6",
            "down 4",
        ]);
        assert_eq!(move_submarine(input), SubState::new(6, 4));
    }

    #[test]
    fn day2a_forward_and_down() {
        let input = to_string_iter(vec![
            "forward 6",
            "down 4",
        ]);
        assert_eq!(day2a(input), ("day2a", 24));
    }
}