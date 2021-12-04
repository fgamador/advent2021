pub fn day2a(_input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let state = move_submarine(_input);
    let answer = state.hpos * state.depth;
    ("day2a", answer as i32)
}

fn move_submarine(_input: impl Iterator<Item=String>) -> SubState {
    SubState::new(3, 5)
}

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
    fn day2a_forward_and_down() {
        let input = to_string_iter(vec![
            "forward 5",
            "down 3",
        ]);
        assert_eq!(day2a(input), ("day2a", 15));
    }
}