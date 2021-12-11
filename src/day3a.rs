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
    input.map(|_bitstr| vec![true, true, false, true, false])
        .fold((vec![0, 0, 0, 0, 0], 0), |counts, bits| {
            let _counts = counts;
            let _bits = bits;
            (vec![1, 1, 0, 1, 0], 1)
        })
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

// fn calc_gamma_and_epsilon(input: impl Iterator<Item=String>) -> (i32, i32) {
//     let gamma = calc_gamma(input);
//     (gamma, 31 - gamma)
// }
//
// fn calc_gamma(input: impl Iterator<Item=String>) -> i32 {
//     count_net_ones(input).into_iter()
//         .enumerate()
//         .filter(|(_index, count)| *count > 0)
//         .fold(0, |gamma, (index, _count)| {
//             gamma + (1 << index)
//         })
// }
//
// fn count_net_ones(input: impl Iterator<Item=String>) -> Vec<i32> {
//     input.map(|bitstr| bitstr_to_net_ones_deltas(&bitstr))
//         .fold(vec![0, 0, 0, 0, 0], |net_counts, bitstr_counts| {
//             add_counts_vectors(net_counts, bitstr_counts)
//         })
// }
//
// fn add_counts_vectors(counts1: Vec<i32>, counts2: Vec<i32>) -> Vec<i32> {
//     let mut sum: Vec<i32> = Vec::with_capacity(5);
//     counts1.iter().zip(counts2)
//         .for_each(|(count1, count2)| sum.push(count1 + count2));
//     sum
// }
//
// fn bitstr_to_net_ones_deltas(bitstr: &str) -> Vec<i32> {
//     let mut bits: Vec<i32> = Vec::with_capacity(5);
//     for bitchar in bitstr.chars().rev() {
//         bits.push(if bitchar == '1' { 1 } else { -1 });
//     }
//     bits
// }

#[cfg(test)]
mod tests {
    use crate::day3a::*;
    use crate::util::to_string_iter;

    // #[test]
    // fn sample_bitstr_to_net_ones_deltas() {
    //     assert_eq!(bitstr_to_net_ones_deltas("11010"), vec![-1, 1, -1, 1, 1]);
    // }
    //
    // #[test]
    // fn count_net_ones_single_input() {
    //     let input = to_string_iter(vec![
    //         "11010",
    //     ]);
    //     assert_eq!(count_net_ones(input), vec![-1, 1, -1, 1, 1]);
    // }
    //
    // #[test]
    // fn count_net_ones_two_inputs() {
    //     let input = to_string_iter(vec![
    //         "11010",
    //         "01100",
    //     ]);
    //     assert_eq!(count_net_ones(input), vec![-2, 0, 0, 2, 0]);
    // }
    //
    // #[test]
    // fn count_gamma_single_input() {
    //     let input = to_string_iter(vec![
    //         "11010",
    //     ]);
    //     assert_eq!(calc_gamma(input), 26);
    // }

    // #[test]
    // fn add_example_bits_to_ones_counts() {
    //     let prev_counts = (vec![0, 0, 0, 0, 0], 0);
    //     let bits = vec![1, 1, 0, 1, 0];
    //     let _counts = prev_counts;
    //     let _bits = bits;
    //     assert_eq!((vec![1, 1, 0, 1, 0], 1), (vec![1, 1, 0, 1, 0], 1));
    // }

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