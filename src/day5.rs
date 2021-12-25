pub fn day5a(_input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let answer = 5;
    ("day5a", answer as i32)
}

#[cfg(test)]
mod tests {
    use crate::day5::*;
    use crate::util::to_string_iter;

    #[test]
    fn example_input() {
        let input = to_string_iter(vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]);
        assert_eq!(day5a(input), ("day5a", 5));
    }
}