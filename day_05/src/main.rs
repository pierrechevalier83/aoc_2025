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

fn merge_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    ranges.sort_by_key(|&(start, _)| start);

    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
                continue;
            }
        }
        merged.push((start, end));
    }
    merged
}

fn count_fresh_ingredient_ids(input: &str) -> usize {
    let (ranges, _ingredients) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .split("\n")
        .map(|line: &str| {
            let (start, end) = line.split_once("-").unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect::<Vec<(usize, usize)>>();
    merge_ranges(ranges)
        .into_iter()
        .map(|range| range.1 - range.0 + 1)
        .sum()
}

fn main() {
    println!("Part 1 - test: {}", count_fresh_ingredients(TEST_INPUT));
    println!(
        "Part 1: {}",
        count_fresh_ingredients(include_str!("../input.txt"))
    );
    println!("Part 2 - test: {}", count_fresh_ingredient_ids(TEST_INPUT));
    println!(
        "Part 2: {}",
        count_fresh_ingredient_ids(include_str!("../input.txt"))
    );
}
