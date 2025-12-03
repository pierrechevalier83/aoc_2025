const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

fn total_output_joltage(input: &str) -> u64 {
    input
        .split("\n")
        .map(|line| {
            if line.is_empty() {
                0
            } else {
                let first_digit = line.chars().take(line.len() - 1).max().unwrap();
                let first_digit_pos = line.chars().position(|c| c == first_digit).unwrap();
                let second_digit = line.chars().skip(first_digit_pos + 1).max().unwrap();
                format!("{first_digit}{second_digit}").parse().unwrap()
            }
        })
        .sum()
}

fn main() {
    println!("test: {}", total_output_joltage(TEST_INPUT));
    println!(
        "Part 1: {}",
        total_output_joltage(include_str!("../input.txt"))
    );
}
