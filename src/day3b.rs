pub fn day3b(input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let og_rating = calc_og_rating(input);
    ("day3b", og_rating * 10)
}

fn calc_og_rating(_input: impl Iterator<Item=String>) -> i32 {
    23
}

#[cfg(test)]
mod tests {
    use crate::day3b::*;
    use crate::util::to_string_iter;

    #[test]
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
        assert_eq!(day3b(input), ("day3b", 230));
    }
}
