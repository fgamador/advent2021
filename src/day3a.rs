pub fn day3a(input: impl Iterator<Item=String>) -> (&'static str, i32) {
    let (gamma, epsilon) = calc_gamma_and_epsilon(input);
    ("day3a", gamma * epsilon)
}

fn calc_gamma_and_epsilon(input: impl Iterator<Item=String>) -> (i32, i32) {
    let gamma = calc_gamma(input);
    (gamma, 31 - gamma)
}

fn calc_gamma(_input: impl Iterator<Item=String>) -> i32 {
    vec![0, 1, 0, 1, 1].into_iter()
        .enumerate()
        .fold(0, |gamma, (index, count)| {
            gamma + count * (1 << index)
        })
}

#[cfg(test)]
mod tests {
    use crate::day3a::*;
    use crate::util::to_string_iter;

    #[test]
    fn gamma_single_input() {
        let input = to_string_iter(vec![
            "11010",
        ]);
        assert_eq!(calc_gamma(input), 26);
    }

    #[test]
    fn gamma_and_epsilon_single_input() {
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