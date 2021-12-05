use itertools::Itertools;

#[allow(dead_code)]
pub fn day1b(input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let answer = input
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows::<(_, _, _)>()
        .map(|(d1, d2, d3)| d1 + d2 + d3)
        .tuple_windows()
        .filter(|(d1, d2)| d2 > d1)
        .count();
    ("day1b", answer as i32)
}

#[cfg(test)]
mod tests {
    use crate::day1b::*;
    use crate::util::to_string_iter;

    #[test]
    fn empty_input() {
        let input = to_string_iter(vec![]);
        assert_eq!(day1b(input), ("day1b", 0));
    }

    #[test]
    fn example_input() {
        let input = to_string_iter(vec![
            "199",
            "200",
            "208",
            "210",
            "200",
            "207",
            "240",
            "269",
            "260",
            "263",
        ]);
        assert_eq!(day1b(input), ("day1b", 5));
    }
}