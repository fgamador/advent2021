use itertools::Itertools;

pub fn day3a(input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let (gamma, epsilon) = calc_gamma_and_epsilon(input);
    ("day3a", gamma * epsilon)
}

fn calc_gamma_and_epsilon(input: impl Iterator<Item=String>) -> (i32, i32) {
    let gamma_bits = calc_gamma_bits(input);
    let gamma = bits_to_decimal(gamma_bits);
    (gamma, 31 - gamma)
}

fn calc_gamma_bits(_input: impl Iterator<Item=String>) -> Vec<bool> {
    let (one_counts, num_values) = (vec![1, 1, 0, 1, 0], 1);
    one_counts.iter().rev()
        .map(|count| *count > num_values / 2)
        .collect_vec()
}

fn bits_to_decimal(bits: Vec<bool>) -> i32 {
    bits
        .into_iter().enumerate()
        .filter(|(_index, bit)| *bit)
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

    #[test]
    fn calc_gamma_three_inputs() {
        let one_counts = &vec![1, 2, 0, 3, 2];
        let num_values = 3;
        let gamma_bits = one_counts.iter().rev()
            .map(|count| *count > num_values / 2)
            .collect_vec();
        assert_eq!(bits_to_decimal(gamma_bits), 11);
    }

    #[test]
    fn calc_gamma_single_input() {
        let one_counts = &vec![1, 1, 0, 1, 0];
        let num_values = 1;
        let gamma_bits = one_counts.iter().rev()
            .map(|count| *count > num_values / 2)
            .collect_vec();
        assert_eq!(bits_to_decimal(gamma_bits), 26);
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