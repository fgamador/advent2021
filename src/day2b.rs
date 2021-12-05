use itertools::Itertools;

pub fn day2b(_input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let answer = 0;
    ("day2b", answer)
}

#[cfg(test)]
mod tests {
    use crate::day2b::*;
    use crate::util::to_string_iter;

    #[test]
    fn day2b_forward() {
        let input = to_string_iter(vec![
            "forward 6",
        ]);
        assert_eq!(day2b(input), ("day2b", 0));
    }
}