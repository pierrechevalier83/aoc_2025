const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

#[derive(Clone, Copy)]
enum Operation {
    Product,
    Addition,
}

fn do_homework(homework: &str) -> usize {
    let lines = homework.trim_end().split("\n").collect::<Vec<_>>();
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
    result.iter().sum()
}

fn main() {
    println!("Part 1 - test: {}", do_homework(TEST_INPUT));
    println!("Part 1: {}", do_homework(include_str!("../input.txt")));
}
