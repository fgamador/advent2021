pub fn day1a(_input: impl Iterator<Item=String>) -> (&'static str, i32) {
    ("day1a", 0)
}

#[cfg(test)]
mod tests {
    use crate::day1::day1a;
    use crate::util::to_string_iter;

    #[test]
    fn day1a_empty_input() {
        let input = to_string_iter(vec![
        ]);
        assert_eq!(day1a(input), ("day1a", 0));
    }
}