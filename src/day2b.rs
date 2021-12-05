use itertools::Itertools;

pub fn day2b(input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let state = move_submarine(input);
    let answer = state.hpos * state.depth;
    ("day2b", answer)
}

fn move_submarine(input: impl Iterator<Item=String>) -> SubState {
    input
        .map(|command| command_to_delta(&command))
        .next().unwrap()
}

fn command_to_delta(command: &str) -> SubState {
    let split = command.split_whitespace().collect_vec();
    let arg: u32 = String::from(split[1]).parse().unwrap();
    if split[0] == "down" {
        SubState::new(0, 0, arg as i32)
    } else if split[0] == "forward" {
        SubState::new(arg as i32, 0, 0)
    } else {
        panic!("Unknown command {}", split[0]);
    }
}

#[derive(Debug, PartialEq)]
struct SubState {
    pub hpos: i32,
    pub depth: i32,
    pub aim: i32,
}

impl SubState {
    pub fn new(hpos: i32, depth: i32, aim: i32) -> Self {
        SubState {
            hpos,
            depth,
            aim: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day2b::*;
    use crate::util::to_string_iter;

    #[test]
    fn down_command_to_delta() {
        assert_eq!(command_to_delta("down 2"), SubState::new(0, 0, 2));
    }

    #[test]
    fn forward_command_to_delta() {
        assert_eq!(command_to_delta("forward 3"), SubState::new(3, 0, 0));
    }

    #[test]
    fn move_submarine_forward() {
        let input = to_string_iter(vec![
            "forward 3",
        ]);
        assert_eq!(move_submarine(input), SubState::new(3, 0, 0));
    }

    #[test]
    #[ignore]
    fn move_submarine_down_and_forward() {
        let input = to_string_iter(vec![
            "down 2",
            "forward 3",
        ]);
        assert_eq!(move_submarine(input), SubState::new(3, 6, 0));
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