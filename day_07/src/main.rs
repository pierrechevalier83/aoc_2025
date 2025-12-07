#![feature(slice_split_once)]

const TEST_INPUT: &[u8] = b".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

fn count_splits(input: &[u8]) -> usize {
    let mut rays: Vec<bool> = input
        .split_once(|b| *b == b'\n')
        .unwrap()
        .0
        .iter()
        .map(|b| *b == b'S')
        .collect();
    input
        .split(|b| *b == b'\n')
        .skip(1)
        .map(|line| {
            let mut next_rays = rays.clone();
            let mut num_splits: usize = 0;
            for (index, (b, ray)) in line.into_iter().zip(&rays).enumerate() {
                if *b == b'^' && *ray {
                    next_rays[index] = false;
                    next_rays[index + 1] = true;
                    next_rays[index - 1] = true;
                    num_splits += 1;
                }
            }

            rays = next_rays.clone();
            num_splits
        })
        .sum()
}

fn count_timelines(input: &[u8]) -> usize {
    let mut rays: Vec<usize> = input
        .split_once(|b| *b == b'\n')
        .unwrap()
        .0
        .iter()
        .map(|b| if *b == b'S' { 1 } else { 0 })
        .collect();
    input.split(|b| *b == b'\n').skip(1).for_each(|line| {
        let mut next_rays = rays.clone();
        for (index, (b, ray)) in line.into_iter().zip(&rays).enumerate() {
            if *b == b'^' && *ray > 0 {
                next_rays[index] = 0;
                next_rays[index + 1] += ray;
                next_rays[index - 1] += ray;
            }
        }

        rays = next_rays.clone();
    });
    rays.iter().sum()
}

fn main() {
    println!("Part 1 - test: {}", count_splits(TEST_INPUT));
    println!("Part 1: {}", count_splits(include_bytes!("../input.txt")));
    println!("Part 2 - test: {}", count_timelines(TEST_INPUT));
    println!(
        "Part 2 - test: {}",
        count_timelines(include_bytes!("../input.txt"))
    );
}
