use itertools::Itertools;

pub fn day2b(_input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let state = SubState {
        hpos: 3,
        depth: 6,
    };
    let answer = state.hpos * state.depth;
    ("day2b", answer)
}

struct SubState {
    pub hpos: i32,
    pub depth: i32,
}

#[cfg(test)]
mod tests {
    use crate::day2b::*;
    use crate::util::to_string_iter;

    #[test]
    fn day2b_down_and_forward() {
        let input = to_string_iter(vec![
            "down 2",
            "forward 3",
        ]);
        assert_eq!(day2b(input), ("day2b", 18));
    }
}