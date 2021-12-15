pub fn day3b(input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let og_rating = calc_og_rating(input);
    ("day3b", og_rating * 10)
}

fn calc_og_rating(_input: impl Iterator<Item=String>) -> i32 {
    bits_to_decimal(&find_og_rating_bits(&vec![vec![true, true]]))
}

fn find_og_rating_bits(input: &[Vec<bool>]) -> Vec<bool> {
    input[0].clone()
}

fn bits_to_decimal(bits: &[bool]) -> i32 {
    bits
        .iter().rev().enumerate()
        .filter(|(_index, bit)| **bit)
        .fold(0, |result, (index, _bit)| result + (1 << index))
}

#[cfg(test)]
mod tests {
    use crate::day3b::*;
    use crate::util::to_string_iter;

    #[test]
    fn example_bits_to_decimal() {
        assert_eq!(bits_to_decimal(&vec![true, true, false, true, false, true]),
                   32 + 16 + 4 + 1)
    }

    #[test]
    fn simple_og_rating() {
        let input = to_string_iter(vec![
            "11",
        ]);
        assert_eq!(calc_og_rating(input), 3);
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
        assert_eq!(day3b(input), ("day3b", 230));
    }
}