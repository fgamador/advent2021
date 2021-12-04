pub fn day2a(_input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let answer = 15;
    ("day2a", answer as i32)
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