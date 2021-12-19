use itertools::Itertools;

pub fn day3b(input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let input_bitvecs = bitstrs_to_bitvecs(input);
    let og_rating = calc_rating(&input_bitvecs, &og_rating_selection_fn);
    let cs_rating = calc_rating(&input_bitvecs, &cs_rating_selection_fn);
    ("day3b", og_rating * cs_rating)
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

fn calc_rating<F>(input_bitvecs: &[Vec<bool>], selection_fn: &F) -> i32
    where F: Fn(usize, usize) -> bool
{
    bits_to_decimal(&find_rating_bits(&input_bitvecs, selection_fn))
}

fn og_rating_selection_fn(true_bitvecs_len: usize, false_bitvecs_len: usize) -> bool {
    true_bitvecs_len >= false_bitvecs_len
}

fn cs_rating_selection_fn(true_bitvecs_len: usize, false_bitvecs_len: usize) -> bool {
    true_bitvecs_len < false_bitvecs_len
}

fn find_rating_bits<F>(input_bitvecs: &[Vec<bool>], selection_fn: &F) -> Vec<bool>
    where F: Fn(usize, usize) -> bool
{
    let mut bit_index = 0;
    let mut chosen_bitvecs = winnow_to_candidate_rating_bitvecs(input_bitvecs, bit_index, selection_fn);

    while chosen_bitvecs.len() > 1 {
        assert!(!chosen_bitvecs.is_empty());
        bit_index += 1;
        chosen_bitvecs = winnow_to_candidate_rating_bitvecs(&chosen_bitvecs, bit_index, selection_fn);
    }

    chosen_bitvecs[0].clone()
}

fn winnow_to_candidate_rating_bitvecs<F>(bitvecs: &[Vec<bool>], bit_index: usize, selection_fn: F) -> Vec<Vec<bool>>
    where F: Fn(usize, usize) -> bool
{
    let (true_bitvecs, false_bitvecs) = partition_bitvecs_by_bit_value(bitvecs, bit_index);

    if selection_fn(true_bitvecs.len(), false_bitvecs.len()) {
        true_bitvecs
    } else {
        false_bitvecs
    }
}

fn partition_bitvecs_by_bit_value(bitvecs: &[Vec<bool>], bit_index: usize) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
    bitvecs.iter().cloned().partition(|bitvec| bitvec[bit_index])
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
        assert_eq!(partition_bitvecs_by_bit_value(&input_bitvecs, 1),
                   (vec![vec![false, true, true], vec![true, true, true]],
                    vec![vec![true, false, false], vec![false, false, true]]))
    }

    #[test]
    fn simple_og_rating_bits() {
        let input_bitvecs = vec![vec![true, true]];
        assert_eq!(find_rating_bits(&input_bitvecs, &og_rating_selection_fn), vec![true, true])
    }

    #[test]
    fn single_bitvec_becomes_og_rating_bits_even_with_leading_false() {
        let input_bitvecs = vec![vec![false, true, false, false]];
        assert_eq!(find_rating_bits(&input_bitvecs, &og_rating_selection_fn), vec![false, true, false, false])
    }

    #[test]
    fn og_rating_bits_prefer_true_bit_if_equal_numbers_of_true_and_false() {
        let input_bitvecs = vec![
            vec![false, true],
            vec![true, false],
        ];
        assert_eq!(find_rating_bits(&input_bitvecs, &og_rating_selection_fn), vec![true, false])
    }

    #[test]
    fn og_rating_bits_prefers_majority_bit_value() {
        let input_bitvecs = vec![
            vec![false, false],
            vec![true, false],
            vec![false, true],
        ];
        assert_eq!(find_rating_bits(&input_bitvecs, &og_rating_selection_fn), vec![false, true])
    }

    #[test]
    fn cs_rating_bits_prefers_minority_bit_value() {
        let input_bitvecs = vec![
            vec![false, false],
            vec![true, false],
            vec![false, true],
        ];
        assert_eq!(find_rating_bits(&input_bitvecs, &cs_rating_selection_fn), vec![true, false])
    }

    #[test]
    fn example_bits_to_decimal() {
        assert_eq!(bits_to_decimal(&vec![true, true, false, true, false, true]),
                   32 + 16 + 4 + 1)
    }

    #[test]
    fn simple_og_rating() {
        let input_bitvecs = vec![vec![true, true]];
        assert_eq!(calc_rating(&input_bitvecs, &og_rating_selection_fn), 3);
    }

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
