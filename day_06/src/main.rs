const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

#[derive(Clone, Copy)]
enum Operation {
    Product,
    Addition,
}

#[derive(Clone, Copy)]
enum Protocol {
    Part1,
    Part2,
}

fn do_homework(homework: &str, protocol: Protocol) -> usize {
    let lines = homework
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let operations = lines
        .last()
        .unwrap()
        .split(" ")
        .filter_map(|op| match op.trim() {
            "*" => Some(Operation::Product),
            "+" => Some(Operation::Addition),
            "" => None,
            _ => panic!("Expected operation, got {}", op.trim()),
        })
        .collect::<Vec<_>>();
    let mut result = operations
        .iter()
        .map(|op| match op {
            Operation::Product => 1,
            Operation::Addition => 0,
        })
        .collect::<Vec<_>>();
    match protocol {
        Protocol::Part1 => {
            for line in lines.iter().take(lines.len() - 1) {
                for (index, col) in line
                    .split(" ")
                    .filter(|col| !col.trim().is_empty())
                    .enumerate()
                {
                    let num: usize = col.trim().parse().unwrap();
                    match operations[index] {
                        Operation::Product => {
                            result[index] *= num;
                        }
                        Operation::Addition => {
                            result[index] += num;
                        }
                    }
                }
            }
        }
        Protocol::Part2 => {
            let mut numbers: Vec<Vec<usize>> = Vec::new();

            for (line_index, line) in lines.iter().rev().skip(1).enumerate() {
                let mut i = 0;
                let mut j = 0;
                for (char_index, (d, o)) in
                    line.chars().zip(lines.last().unwrap().chars()).enumerate()
                {
                    if o == '*' || o == '+' {
                        if char_index > 0 {
                            i += 1;
                        }
                        if line_index == 0 {
                            let mut inner_vec = Vec::new();
                            let remainder = lines
                                .last()
                                .unwrap()
                                .chars()
                                .skip(char_index + 1)
                                .collect::<String>();
                            inner_vec.resize(
                                remainder
                                    .chars()
                                    .position(|c| c == '*' || c == '+')
                                    .unwrap_or(remainder.chars().count() + 1),
                                0,
                            );
                            numbers.push(inner_vec);
                        }
                        j = 0;
                    }
                    let digit = d.to_digit(10);
                    if let Some(digit) = digit {
                        let power_of_ten = if numbers[i][j] == 0 {
                            0
                        } else {
                            numbers[i][j].ilog10() + 1
                        };
                        numbers[i][j] += digit as usize * 10_usize.pow(power_of_ten);
                    }
                    j += 1;
                }
            }
            for (index, (op, nums)) in operations.iter().zip(numbers).enumerate() {
                match op {
                    Operation::Product => {
                        result[index] = nums.iter().product();
                    }
                    Operation::Addition => {
                        result[index] = nums.iter().sum();
                    }
                }
            }
        }
    }
    result.iter().sum()
}

fn main() {
    println!(
        "Part 1 - test: {}",
        do_homework(TEST_INPUT, Protocol::Part1)
    );
    println!(
        "Part 1: {}",
        do_homework(include_str!("../input.txt"), Protocol::Part1)
    );
    println!(
        "Part 2 - test: {}",
        do_homework(TEST_INPUT, Protocol::Part2)
    );
    println!(
        "Part 2: {}",
        do_homework(include_str!("../input.txt"), Protocol::Part2)
    );
}
