use pathfinding::prelude::bfs;
use std::fmt;

const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

fn toggle_bit(bits: &mut u16, index: usize) {
    *bits ^= 1 << index;
}

fn toggle_all_bits(bits: u16, mask: u16) -> u16 {
    bits ^ mask
}

#[derive(Clone)]
struct Machine {
    // Single digit index. Up-to 10 bits
    lights: u16,
    // A bitmask per set of buttons
    button_sets: Vec<u16>,
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Machine")
            .field("lights", &format!("{:010b}", self.lights))
            .field(
                "button_sets",
                &self
                    .button_sets
                    .iter()
                    .map(|b| format!("{:010b}", *b))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl Machine {
    fn from_str(line: &str) -> Self {
        let (lights_part, buttons_and_joltages_part) = line.split_once(' ').unwrap();
        let (buttons_part, _joltages_part) = buttons_and_joltages_part.rsplit_once(' ').unwrap();
        let mut lights: u16 = 0;
        for (i, c) in lights_part
            .chars()
            .filter(|c| match c {
                '.' | '#' => true,
                '[' | ']' => false,
                _ => {
                    panic!("Unexpected lights character: {c}");
                }
            })
            .enumerate()
        {
            if c == '#' {
                toggle_bit(&mut lights, i);
            }
        }
        let button_sets = buttons_part
            .split(' ')
            .map(|button_set| {
                let mut bitmask = 0;
                for c in button_set.chars().filter(|c| match c {
                    '(' | ')' | ',' => false,
                    c => {
                        if c.is_digit(10) {
                            true
                        } else {
                            panic!("Unexpected lights character: {c}");
                        }
                    }
                }) {
                    let button_index = c.to_digit(10).map(|digit| digit).unwrap() as usize;
                    toggle_bit(&mut bitmask, button_index)
                }
                bitmask
            })
            .collect();
        Self {
            lights,
            button_sets,
        }
    }
    fn min_flips(&self) -> usize {
        let path = bfs(
            &0,
            |&x| {
                self.button_sets
                    .iter()
                    .map(|mask| toggle_all_bits(x, *mask))
                    .collect::<Vec<_>>()
            },
            |&x| x == self.lights,
        )
        .unwrap();
        path.len() - 1
    }
}

fn part_1(input: &str) -> usize {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| Machine::from_str(line))
        .map(|machine| machine.min_flips())
        .sum()
}

fn main() {
    println!("Part 1 - test: {}", part_1(TEST_INPUT));
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
}
