const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

enum Protocol {
    Part1,
    Part2,
}

fn find_max_and_pos_until_index(index: usize, skip_start: usize, line: &str) -> (char, usize) {
    let digit = line
        .chars()
        .take(index)
        .skip(skip_start)
        .max()
        .expect("line len should be greater");
    let pos = line
        .chars()
        .skip(skip_start)
        .position(|c| c == digit)
        .expect("this digit should exist");
    (digit, pos + skip_start)
}

fn output_joltage_part2(line: &str, n_digits: usize) -> u64 {
    let mut output = String::new();
    let mut skip_start = 0;
    for n in (1..=n_digits).rev() {
        let (nth_digit, nth_digit_pos) =
            find_max_and_pos_until_index(line.len() - n + 1, skip_start, line);
        skip_start = nth_digit_pos + 1;
        output.push(nth_digit);
    }
    output.parse().unwrap()
}

fn total_output_joltage(input: &str, protocol: Protocol) -> u64 {
    input
        .split("\n")
        .map(|line| {
            if line.is_empty() {
                0
            } else {
                match protocol {
                    Protocol::Part1 => output_joltage_part2(line, 2),

                    Protocol::Part2 => output_joltage_part2(line, 12),
                }
            }
        })
        .sum()
}

fn main() {
    println!(
        "test: {}",
        total_output_joltage(TEST_INPUT, Protocol::Part1)
    );
    println!(
        "Part 1: {}",
        total_output_joltage(include_str!("../input.txt"), Protocol::Part1)
    );
    println!(
        "test: {}",
        total_output_joltage(TEST_INPUT, Protocol::Part2)
    );
    println!(
        "Part 2: {}",
        total_output_joltage(include_str!("../input.txt"), Protocol::Part2)
    );
}
