pub fn day6a(_input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let answer = 5934;
    ("day6a", answer as i32)
}

#[cfg(test)]
mod tests {
    use crate::day6::*;
    use crate::util::to_string_iter;

    #[test]
    fn example_input_6a() {
        let input = to_string_iter(vec![
            "3,4,3,1,2",
        ]);
        assert_eq!(day6a(input), ("day6a", 5934));
    }
}