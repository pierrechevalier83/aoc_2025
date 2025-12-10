use good_lp::*;
use pathfinding::prelude::bfs;
use rayon::prelude::*;
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

fn read_bit(mask: u16, index: usize) -> bool {
    (mask & (1 << index)) != 0
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Machine {
    // Single digit index. Up-to 10 bits
    lights: u16,
    // A bitmask per set of buttons
    button_sets: Vec<u16>,
    joltages: Vec<isize>,
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
            .field("joltages", &format!("{:?}", self.joltages))
            .finish()
    }
}

impl Machine {
    fn from_str(line: &str) -> Self {
        let (lights_part, buttons_and_joltages_part) = line.split_once(' ').unwrap();
        let (buttons_part, joltages_part) = buttons_and_joltages_part.rsplit_once(' ').unwrap();
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
        let joltages = joltages_part
            .chars()
            .filter(|c| match c {
                '{' | '}' => false,
                ',' => true,
                _ => {
                    if c.is_digit(10) {
                        true
                    } else {
                        panic!("Unexpected lights character: {c}");
                    }
                }
            })
            .collect::<String>()
            .split(',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect();
        Self {
            lights,
            button_sets,
            joltages,
        }
    }
    fn min_flips_to_initialize(&self) -> usize {
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
    fn min_flips_to_reach_target_joltages(&self) -> usize {
        let mut problem = ProblemVariables::new();

        let n_flips: Vec<_> = problem.add_all(self.button_sets.iter().map(|_button_set| {
            variable()
                .integer()
                .min(0)
                .max(*self.joltages.iter().max().unwrap() as f64)
        }));
        let objective: Expression = n_flips.iter().sum();
        let mut model = problem.minimise(&objective).using(scip);
        for (joltage_index, target_joltage) in self.joltages.iter().enumerate() {
            let mut lhs_expression = Expression::default();

            for (button_index, button_set) in self.button_sets.iter().enumerate() {
                if read_bit(*button_set, joltage_index) {
                    lhs_expression += &n_flips[button_index];
                }
            }

            model.add_constraint(lhs_expression.eq(*target_joltage as f64));
        }
        let solution = model.solve().unwrap();
        solution.eval(&objective) as usize
    }
}

fn part_1(input: &str) -> usize {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| Machine::from_str(line))
        .map(|machine| machine.min_flips_to_initialize())
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .par_iter()
        .map(|line| Machine::from_str(line))
        .map(|machine| {
            let cost = machine.min_flips_to_reach_target_joltages();
            cost
        })
        .sum()
}

fn main() {
    println!("Part 1 - test: {}", part_1(TEST_INPUT));
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2 - test: {}", part_2(TEST_INPUT));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
