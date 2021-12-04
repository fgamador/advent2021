use itertools::Itertools;

pub fn _day1a(input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let answer = input
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows()
        .filter(|(d1, d2)| d2 > d1)
        .count();
    ("day1a", answer as i32)
}

pub fn _day1b(input: impl Iterator<Item=String>) -> (&'static str, i32) {
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
    use crate::day1::*;
    use crate::util::to_string_iter;

    #[test]
    fn day1a_empty_input() {
        let input = to_string_iter(vec![]);
        assert_eq!(_day1a(input), ("day1a", 0));
    }

    #[test]
    fn day1a_example_input() {
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
        assert_eq!(_day1a(input), ("day1a", 7));
    }

    #[test]
    fn day1b_example_input() {
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
        assert_eq!(_day1b(input), ("day1b", 5));
    }
}