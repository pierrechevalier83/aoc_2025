#![feature(trim_prefix_suffix)]

fn is_invalid_id(id: &usize) -> bool {
    let num_digits = (*id).ilog10() + 1;
    num_digits % 2 == 0
        && id / 10_usize.pow(num_digits / 2)
            == id - id / 10_usize.pow(num_digits / 2) * 10_usize.pow(num_digits / 2)
}

fn sum_invalid_ids(input: &str) -> usize {
    input
        .trim_suffix("\n")
        .split(",")
        .flat_map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .filter(is_invalid_id)
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", sum_invalid_ids(input));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    use super::*;
    #[test]
    fn test_sum_invalid_ids() {
        assert_eq!(1227775554, sum_invalid_ids(TEST_INPUT));
    }
}
