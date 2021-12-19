use itertools::Itertools;

pub fn day3b(input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let input_bitvecs = bitstrs_to_bitvecs(input);
    let og_rating = calc_og_rating(&input_bitvecs);
    ("day3b", og_rating * 10)
}

fn calc_og_rating(input_bitvecs: &[Vec<bool>]) -> i32 {
    let og_rating_bits = find_og_rating_bits(&input_bitvecs);
    bits_to_decimal(&og_rating_bits)
}

fn bitstrs_to_bitvecs(input_bitstrs: impl Iterator<Item=String>) -> Vec<Vec<bool>> {
    input_bitstrs
        .map(|input_bitstr| bitstr_to_bitvec(&input_bitstr))
        .collect_vec()
}

fn bitstr_to_bitvec(bitstr: &str) -> Vec<bool> {
    bitstr.chars()
        .map(|bitchar| bitchar == '1')
        .collect_vec()
}

fn find_og_rating_bits(input_bitvecs: &[Vec<bool>]) -> Vec<bool> {
    if input_bitvecs.len() == 1 {
        return input_bitvecs[0].clone();
    }

    let (true_bitvecs, false_bitvecs) = partition_bitvecs(input_bitvecs, 0);

    if true_bitvecs.len() == false_bitvecs.len() {
        true_bitvecs[0].clone()
    } else {
        false_bitvecs[0].clone()
    }
}

fn partition_bitvecs(input_bitvecs: &[Vec<bool>], bit_index: usize) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
    (input_bitvecs.iter().filter(|bitvec| bitvec[bit_index]).cloned().collect_vec(),
     input_bitvecs.iter().filter(|bitvec| !bitvec[bit_index]).cloned().collect_vec())
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
    fn example_bitstr_to_bitvec() {
        assert_eq!(bitstr_to_bitvec("1101010"), vec![true, true, false, true, false, true, false]);
    }

    #[test]
    fn single_bitstr_to_bitvec() {
        let input_bitstrs = to_string_iter(vec![
            "11",
        ]);
        assert_eq!(bitstrs_to_bitvecs(input_bitstrs), vec![vec![true, true]]);
    }

    #[test]
    fn partition_bitvecs_by_second_bit() {
        let input_bitvecs = vec![
            vec![false, true, true],
            vec![true, false, false],
            vec![false, false, true],
            vec![true, true, true],
        ];
        assert_eq!(partition_bitvecs(&input_bitvecs, 1),
                   (vec![vec![false, true, true], vec![true, true, true]],
                    vec![vec![true, false, false], vec![false, false, true]]))
    }

    #[test]
    fn simple_og_rating_bits() {
        let input_bitvecs = vec![vec![true, true]];
        assert_eq!(find_og_rating_bits(&input_bitvecs), vec![true, true])
    }

    #[test]
    fn single_bitvec_becomes_og_rating_bits_even_with_leading_false() {
        let input_bitvecs = vec![vec![false, true, false, false]];
        assert_eq!(find_og_rating_bits(&input_bitvecs), vec![false, true, false, false])
    }

    #[test]
    fn og_rating_bits_prefer_true_bit_if_equal_numbers_of_true_and_false() {
        let input_bitvecs = vec![
            vec![false, true],
            vec![true, false],
        ];
        assert_eq!(find_og_rating_bits(&input_bitvecs), vec![true, false])
    }

    #[test]
    #[ignore]
    fn og_rating_bits_prefers_majority_bit_value() {
        let input_bitvecs = vec![
            vec![false, false],
            vec![true, false],
            vec![false, true],
        ];
        assert_eq!(find_og_rating_bits(&input_bitvecs), vec![false, true])
    }

    #[test]
    fn example_bits_to_decimal() {
        assert_eq!(bits_to_decimal(&vec![true, true, false, true, false, true]),
                   32 + 16 + 4 + 1)
    }

    #[test]
    fn simple_og_rating() {
        let input_bitvecs = vec![vec![true, true]];
        assert_eq!(calc_og_rating(&input_bitvecs), 3);
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
