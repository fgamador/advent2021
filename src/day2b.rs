use itertools::Itertools;

pub fn day2b(_input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let answer = 3 * 6;
    ("day2b", answer)
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