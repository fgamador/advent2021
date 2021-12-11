use itertools::Itertools;

pub fn day3a(input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let (gamma, epsilon) = calc_gamma_and_epsilon(input);
    ("day3a", gamma * epsilon)
}

fn calc_gamma_and_epsilon(input: impl Iterator<Item=String>) -> (i32, i32) {
    let (one_counts, num_values) = count_one_bits(input);
    let gamma_bits = one_counts_to_gamma_bits(&one_counts, num_values);
    let gamma = bits_to_decimal(&gamma_bits);
    (gamma, 31 - gamma)
}

fn count_one_bits(input: impl Iterator<Item=String>) -> (Vec<i32>, i32) {
    input.map(|bitstr| bitstr_to_bits(&bitstr))
        .fold((vec![0, 0, 0, 0, 0], 0), |(mut ones_counts, line_count), bits| {
            increment_elements(&mut ones_counts, &bits);
            (ones_counts, line_count + 1)
        })
}

fn bitstr_to_bits(bitstr: &str) -> Vec<bool> {
    let mut bits: Vec<bool> = Vec::with_capacity(5);
    for bitchar in bitstr.chars() {
        bits.push(bitchar == '1');
    }
    bits
}

fn increment_elements(counts: &mut [i32], bits: &[bool]) {
    counts.iter_mut().enumerate()
        .filter(|(index, _element)| bits[*index])
        .for_each(|(_index, element)| *element += 1);
}

fn one_counts_to_gamma_bits(one_counts: &[i32], num_values: i32) -> Vec<bool> {
    one_counts.iter()
        .map(|count| *count > num_values / 2)
        .collect_vec()
}

fn bits_to_decimal(bits: &[bool]) -> i32 {
    bits
        .iter().rev().enumerate()
        .filter(|(_index, bit)| **bit)
        .fold(0, |result, (index, _bit)| result + (1 << index))
}

#[cfg(test)]
mod tests {
    use crate::day3a::*;
    use crate::util::to_string_iter;

    #[test]
    fn example_bitstr_to_bits() {
        assert_eq!(bitstr_to_bits("11010"), vec![true, true, false, true, false]);
    }

    #[test]
    fn increment_ones_counts_by_example_bits() {
        let mut ones_counts = vec![0, 1, 2, 3, 4];
        increment_elements(&mut ones_counts, &vec![true, true, false, true, false]);
        assert_eq!(ones_counts, vec![1, 2, 2, 4, 4]);
    }

    #[test]
    fn count_one_bits_one_input() {
        let input = to_string_iter(vec![
            "11010",
        ]);
        assert_eq!(count_one_bits(input), (vec![1, 1, 0, 1, 0], 1));
    }

    #[test]
    #[ignore]
    fn count_one_bits_three_inputs() {
        let input = to_string_iter(vec![
            "11001",
            "01010",
            "11010",
        ]);
        assert_eq!(count_one_bits(input), (vec![2, 3, 0, 2, 1], 3));
    }

    #[test]
    fn calc_gamma_bits_three_inputs() {
        let gamma_bits = one_counts_to_gamma_bits(&vec![2, 3, 0, 2, 1], 3);
        assert_eq!(gamma_bits, vec![true, true, false, true, false]);
    }

    #[test]
    fn example_bits_to_decimal() {
        assert_eq!(bits_to_decimal(&vec![true, true, false, true, false]), 26)
    }

    #[test]
    fn count_gamma_and_epsilon_single_input() {
        let input = to_string_iter(vec![
            "11010",
        ]);
        assert_eq!(calc_gamma_and_epsilon(input), (26, 5));
    }

    #[test]
    fn day3a_single_input() {
        let input = to_string_iter(vec![
            "11010",
        ]);
        assert_eq!(day3a(input), ("day3a", (16 + 8 + 2) * (4 + 1)));
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
        assert_eq!(day3a(input), ("day3a", 198));
    }
}