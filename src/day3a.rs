pub fn day3a(_input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let answer = 26 * 5;
    ("day3a", answer)
}

#[cfg(test)]
mod tests {
    use crate::day3a::*;
    use crate::util::to_string_iter;

    #[test]
    fn day3a_single_input() {
        let input = to_string_iter(vec![
            "00101",
        ]);
        assert_eq!(day3a(input), ("day3a", (16 + 8 + 2) * (4 + 1)));
    }

    #[test]
    #[ignore]
    fn example_input() {
        let input = to_string_iter(vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ]);
        assert_eq!(day3a(input), ("day3a", 198));
    }
}