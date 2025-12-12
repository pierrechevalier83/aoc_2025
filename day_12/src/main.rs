use fxhash::FxHashMap;
use rayon::prelude::*;

const TEST_INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

fn set_bit(bitmask: &mut u64, index: usize) {
    *bitmask |= 1 << index;
}

fn set_bits(bitmask: &mut u64, bits: u64, index: usize) {
    *bitmask |= bits << index;
}

fn read_bit(bitmask: u64, index: usize) -> bool {
    (bitmask & (1 << index)) != 0
}

fn rotate(shape: [u64; 3]) -> [u64; 3] {
    let mut rotated_shape = [0u64; 3];
    for row in 0..3 {
        for col in 0..3 {
            if read_bit(shape[row], col) {
                set_bit(&mut rotated_shape[2 - col], row);
            }
        }
    }
    rotated_shape
}

fn parse_puzzle_pieces(input: &str) -> Vec<[[u64; 3]; 4]> {
    input
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .flat_map(|block| {
            if block.chars().find(|c| *c == 'x').is_some() {
                None
            } else {
                let mut lines = block.split("\n").skip(1).map(|line| {
                    let mut bitmask = 0;
                    for (index, c) in line.chars().enumerate() {
                        if c == '#' {
                            set_bit(&mut bitmask, index);
                        }
                    }
                    bitmask
                });
                Some([
                    lines.next().unwrap(),
                    lines.next().unwrap(),
                    lines.next().unwrap(),
                ])
            }
        })
        .map(move |shape| {
            let mut rotations = (0..4).rev().map(move |n_rotations| {
                let mut rotated_shape = shape;
                for _ in 0..=n_rotations {
                    rotated_shape = rotate(rotated_shape.clone());
                }
                rotated_shape
            });
            [
                rotations.next().unwrap(),
                rotations.next().unwrap(),
                rotations.next().unwrap(),
                rotations.next().unwrap(),
            ]
        })
        .collect()
}

fn parse_puzzles(input: &str) -> Vec<((usize, usize), Vec<usize>)> {
    let block = input
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .last()
        .unwrap();
    block
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (sizes, nums) = line.split_once(": ").unwrap();
            let (num_cols, num_rows) = sizes.split_once('x').unwrap();
            let sizes = (num_rows.parse().unwrap(), num_cols.parse().unwrap());
            let nums = nums
                .split(' ')
                .map(|index| index.parse().unwrap())
                .collect();
            (sizes, nums)
        })
        .collect()
}

#[allow(unused)]
fn print_grid((n_rows, n_cols): (usize, usize), grid: &[u64]) {
    for row in 0..n_rows {
        for col in 0..n_cols {
            if read_bit(grid[row], col) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn place_piece(
    (row_index, col_index): (usize, usize),
    piece: [u64; 3],
    grid: &[u64],
) -> Option<Vec<u64>> {
    let n_tiles_in_piece: usize = piece.iter().map(|row| row.count_ones() as usize).sum();
    let n_tiles_in_grid: usize = grid.iter().map(|row| row.count_ones() as usize).sum();
    let mut grid = grid.to_vec();
    for (piece_index, bits) in piece.into_iter().enumerate() {
        set_bits(&mut grid[row_index + piece_index], bits, col_index);
    }
    if grid
        .iter()
        .map(|row| row.count_ones() as usize)
        .sum::<usize>()
        < n_tiles_in_grid + n_tiles_in_piece
    {
        return None;
    }
    Some(grid)
}

fn fill_unreachable_bits(
    min_pt: (usize, usize),
    (n_rows, n_cols): (usize, usize),
    grid: &mut [u64],
) {
    let mut last_grid = Vec::new();
    let mut grid = grid.to_vec();
    while grid != last_grid {
        last_grid = grid.clone();
        for (row_index, bits) in last_grid.iter().enumerate() {
            for col_index in 0..n_cols {
                if [-1isize, 1]
                    .into_iter()
                    .zip([-1isize, 1])
                    .filter_map(|(r, c)| {
                        if row_index <= min_pt.0 && r == -1 {
                            None
                        } else if row_index >= n_rows - 1 && r == 1 {
                            None
                        } else if col_index <= min_pt.1 && c == -1 {
                            None
                        } else if col_index >= n_cols - 1 && c == 1 {
                            None
                        } else {
                            Some((
                                ((row_index) as isize + r) as usize,
                                ((col_index) as isize + c) as usize,
                            ))
                        }
                    })
                    .all(|(r, c)| read_bit(grid[r], c))
                {
                    set_bits(&mut grid[row_index], *bits, col_index);
                }
            }
        }
    }
}

// Just a trick to encode a grid in less space than the full grid for memoization
fn sign_grid(grid: &[u64]) -> usize {
    // A nice optimization would be for symmetric images of a grid to share a signature
    grid.iter()
        .enumerate()
        .map(|(row_index, row)| row_index * *row as usize)
        .sum::<usize>()
}
fn sign_nums(grid: &[usize]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(row_index, row)| row_index * *row)
        .sum::<usize>()
}

fn solve(
    memo: &mut FxHashMap<(usize, (usize, usize), ((usize, usize), usize)), Option<usize>>,
    grid: Vec<u64>,
    min_pt: (usize, usize),
    ((n_rows, n_cols), nums): ((usize, usize), Vec<usize>),
    puzzle_pieces: &[[[u64; 3]; 4]],
) -> Option<usize> {
    // Optimization: memoization
    let state = (
        sign_grid(&grid),
        min_pt,
        ((n_rows, n_cols), sign_nums(&nums)),
    );
    if let Some(precomputed) = memo.get(&state) {
        return precomputed.clone();
    }
    // Optimization: quick exit out of small grids
    if (n_rows <= 3 + min_pt.0) || (n_cols <= 3 + min_pt.1) {
        return None;
    }
    if nums.iter().all(|num| *num == 0) {
        memo.insert(state.clone(), Some(0));
        return Some(0);
    }
    // Optimization: Don't explore if it won't fit
    let bits_to_place = nums
        .iter()
        .enumerate()
        .map(|(index, num)| {
            num * puzzle_pieces[index][0]
                .iter()
                .map(|row| row.count_ones() as usize)
                .sum::<usize>()
        })
        .sum::<usize>();
    let spots = n_rows * n_cols
        - grid
            .iter()
            .map(|row| row.count_ones() as usize)
            .sum::<usize>();
    if spots < bits_to_place {
        memo.insert(state.clone(), None);
        return None;
    }
    // Optimization: divide and conquer
    // Note: solving for all nums, in each half feels like a bug, but somehow I get the correct solution.
    //
    //
    // First horizontally:
    if let Some(full_row_pos) = grid
        .iter()
        .position(|row| row.count_ones() as usize == n_rows)
    {
        return solve(
            memo,
            grid.clone(),
            min_pt,
            ((full_row_pos, n_cols), nums.clone()),
            puzzle_pieces,
        )
        .and_then(|top_result| {
            solve(
                memo,
                grid,
                (full_row_pos + 1, min_pt.1),
                ((n_rows, n_cols), nums.clone()),
                puzzle_pieces,
            )
            .map(|bottom_result| top_result + bottom_result)
        });
    }
    // Then vertically:
    if let Some(full_col_pos) =
        (0..n_cols).find(|col_index| grid.iter().all(|row| read_bit(*row, *col_index)))
    {
        return solve(
            memo,
            grid.clone(),
            min_pt,
            ((n_rows, full_col_pos), nums.clone()),
            puzzle_pieces,
        )
        .and_then(|top_result| {
            solve(
                memo,
                grid,
                (min_pt.0, full_col_pos + 1),
                ((n_rows, n_cols), nums.clone()),
                puzzle_pieces,
            )
            .map(|bottom_result| top_result + bottom_result)
        });
    }

    for (index, _num) in nums.iter().enumerate().filter(|(_index, num)| **num != 0) {
        let pieces = puzzle_pieces[index];
        for row in (min_pt.0)..=(n_rows - 3) {
            for col in (min_pt.1)..=(n_cols - 3) {
                for piece in pieces {
                    if let Some(mut next_grid) = place_piece((row, col), piece, &grid) {
                        fill_unreachable_bits(min_pt, (n_rows, n_cols), &mut next_grid);
                        let mut next_nums = nums.clone();
                        next_nums[index] -= 1;
                        if let Some(solution) = solve(
                            memo,
                            next_grid,
                            min_pt,
                            ((n_rows, n_cols), next_nums),
                            puzzle_pieces,
                        )
                        .map(|n| n + 1)
                        {
                            memo.insert(state.clone(), Some(solution));
                            return Some(solution);
                        }
                    }
                }
            }
        }
    }
    memo.insert(state.clone(), None);
    None
}

fn is_solvable(puzzle: ((usize, usize), Vec<usize>), puzzle_pieces: &[[[u64; 3]; 4]]) -> bool {
    let n_rows = puzzle.0.0;
    let grid = (0..n_rows).map(|_line_index| 0u64).collect::<Vec<_>>();
    let mut memo = FxHashMap::default();
    let min_pt = (0, 0);
    solve(&mut memo, grid, min_pt, puzzle, puzzle_pieces).is_some()
}

fn part_1(input: &str) -> usize {
    let puzzle_pieces = parse_puzzle_pieces(input);
    let puzzles = parse_puzzles(input);
    puzzles
        .par_iter()
        .filter(|puzzle| is_solvable((*puzzle).clone(), &puzzle_pieces))
        .count()
}

fn main() {
    println!("Part 1 - test: {}", part_1(TEST_INPUT));
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
}
