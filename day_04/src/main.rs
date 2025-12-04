const TEST_INPUT: &[u8] = b"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

fn count_neighbours(matrix: &[&[u8]], position: (usize, usize)) -> usize {
    let size = (matrix[0].len() as isize, matrix.len() as isize);
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            let neighbour_position = (position.0 as isize + i, position.1 as isize + j);
            if i != 0 || j != 0 {
                if neighbour_position.0 >= 0
                    && neighbour_position.0 < size.0
                    && neighbour_position.1 >= 0
                    && neighbour_position.1 < size.1
                {
                    count += if matrix[neighbour_position.0 as usize][neighbour_position.1 as usize]
                        == b'@'
                    {
                        1
                    } else {
                        0
                    };
                }
            }
        }
    }
    count
}

fn count_reachable_neighbours(matrix: &[&[u8]]) -> usize {
    let size = (matrix[0].len(), matrix.len());
    let mut count = 0;
    for i in 0..size.0 {
        for j in 0..size.1 {
            if matrix[i][j] == b'@' && count_neighbours(matrix, (i, j)) < 4 {
                count += 1;
            }
        }
    }
    count
}
fn part_1(input: &[u8]) -> usize {
    let matrix = input
        .split(|b| *b == b'\n')
        .filter(|line| line.len() > 0)
        .collect::<Vec<_>>();
    count_reachable_neighbours(&matrix)
}

fn main() {
    println!("Part 1 - test: {}", part_1(TEST_INPUT));
    println!("Part 1: {}", part_1(include_bytes!("../input.txt")));
}
