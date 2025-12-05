use std::ops::RangeInclusive;

const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

fn count_fresh_ingredients(input: &str) -> usize {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .split("\n")
        .map(|line: &str| {
            let (start, end) = line.split_once("-").unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect::<Vec<RangeInclusive<usize>>>();
    let ingredients = ingredients
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line: &str| line.parse().unwrap())
        .collect::<Vec<usize>>();
    ingredients
        .into_iter()
        .filter(|ingredient| ranges.iter().any(|range| range.contains(&ingredient)))
        .count()
}

fn main() {
    println!("Part 1 - test: {}", count_fresh_ingredients(TEST_INPUT));
    println!(
        "Part 1: {}",
        count_fresh_ingredients(include_str!("../input.txt"))
    );
}
