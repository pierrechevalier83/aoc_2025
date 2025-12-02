#![feature(trim_prefix_suffix)]

#[derive(Clone, Copy)]
enum ValidityProtocol {
    Part1,
    Part2,
}

fn repeats(id: usize, num_digits: u32, chunks: u32) -> bool {
    if num_digits == chunks {
        true
    } else {
        let first_digits = id / 10_usize.pow(num_digits - chunks);
        let last_digits = id - id / 10_usize.pow(chunks) * 10_usize.pow(chunks);
        let all_but_first_digits = id - first_digits * 10_usize.pow(num_digits - chunks);
        first_digits == last_digits && repeats(all_but_first_digits, num_digits - chunks, chunks)
    }
}

fn is_invalid_id(id: usize, protocol: ValidityProtocol) -> bool {
    let num_digits = id.ilog10() + 1;
    match protocol {
        ValidityProtocol::Part1 => {
            if num_digits % 2 == 0 {
                repeats(id, num_digits, num_digits / 2)
            } else {
                false
            }
        }
        ValidityProtocol::Part2 => {
            (1..=(num_digits / 2)).any(|chunks| repeats(id, num_digits, chunks))
        }
    }
}

fn sum_invalid_ids(input: &str, protocol: ValidityProtocol) -> usize {
    input
        .trim_suffix("\n")
        .split(",")
        .flat_map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .filter(|id| is_invalid_id(*id, protocol))
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!(
        "Part 1: {}",
        sum_invalid_ids(input, ValidityProtocol::Part1)
    );
    println!(
        "Part 2: {}",
        sum_invalid_ids(input, ValidityProtocol::Part2)
    );
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    use super::*;
    #[test]
    fn test_repeats() {
        assert!(repeats(141414, 6, 2));
        assert!(!repeats(141141, 6, 2));
        assert!(repeats(141141, 6, 3));
        assert!(!repeats(101, 3, 1));
    }

    #[test]
    fn test_sum_invalid_ids() {
        assert_eq!(
            1227775554,
            sum_invalid_ids(TEST_INPUT, ValidityProtocol::Part1)
        );
        assert_eq!(
            4174379265,
            sum_invalid_ids(TEST_INPUT, ValidityProtocol::Part2)
        );
    }
}
