use itertools::Itertools;

#[allow(dead_code)]
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

fn command_to_delta(command: &str) -> SubState {
    let split = command.split_whitespace().collect_vec();
    let arg: u32 = String::from(split[1]).parse().unwrap();
    if split[0] == "down" {
        SubState::new(0, arg as i32)
    } else if split[0] == "forward" {
        SubState::new(arg as i32, 0)
    } else if split[0] == "up" {
        SubState::new(0, -(arg as i32))
    } else {
        panic!("Unknown command {}", split[0]);
    }
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
    use crate::day2a::*;
    use crate::util::to_string_iter;

    #[test]
    fn down_command_to_delta() {
        assert_eq!(command_to_delta("down 4"), SubState::new(0, 4));
    }

    #[test]
    fn forward_command_to_delta() {
        assert_eq!(command_to_delta("forward 6"), SubState::new(6, 0));
    }

    #[test]
    fn up_command_to_delta() {
        assert_eq!(command_to_delta("up 5"), SubState::new(0, -5));
    }

    #[test]
    fn move_submarine_forward() {
        let input = to_string_iter(vec![
            "forward 6",
        ]);
        assert_eq!(move_submarine(input), SubState::new(6, 0));
    }

    #[test]
    fn move_submarine_forward_and_down() {
        let input = to_string_iter(vec![
            "forward 5",
            "down 3",
        ]);
        assert_eq!(move_submarine(input), SubState::new(5, 3));
    }

    #[test]
    fn day2a_forward() {
        let input = to_string_iter(vec![
            "forward 6",
        ]);
        assert_eq!(day2a(input), ("day2a", 0));
    }

    #[test]
    fn day2a_forward_and_down() {
        let input = to_string_iter(vec![
            "forward 5",
            "down 3",
        ]);
        assert_eq!(day2a(input), ("day2a", 15));
    }

    #[test]
    fn day2a_example_input() {
        let input = to_string_iter(vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]);
        assert_eq!(day2a(input), ("day2a", 150));
    }
}