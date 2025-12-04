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

fn count_neighbours(matrix: &[Vec<u8>], position: (usize, usize)) -> usize {
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

fn count_reachable_neighbours(matrix: &[Vec<u8>], next_matrix: &mut Vec<Vec<u8>>) -> usize {
    let size = (matrix[0].len(), matrix.len());
    let mut count = 0;
    for i in 0..size.0 {
        for j in 0..size.1 {
            if matrix[i][j] == b'@' && count_neighbours(matrix, (i, j)) < 4 {
                count += 1;
                next_matrix[i][j] = b'.';
            }
        }
    }
    count
}
fn part_1(input: &[u8]) -> usize {
    let matrix = input
        .split(|b| *b == b'\n')
        .filter(|line| line.len() > 0)
        .map(|line| line.iter().copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut next_matrix = matrix
        .iter()
        .map(|line| line.into_iter().copied().collect())
        .collect();
    count_reachable_neighbours(&matrix, &mut next_matrix)
}

fn part_2(input: &[u8]) -> usize {
    let mut matrix = input
        .split(|b| *b == b'\n')
        .filter(|line| line.len() > 0)
        .map(|line| line.iter().copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut next_matrix = matrix
        .iter()
        .map(|line| line.into_iter().copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut count = 0;
    loop {
        let next_count = count_reachable_neighbours(&matrix, &mut next_matrix);
        matrix = next_matrix.clone();
        if next_count == 0 {
            break;
        } else {
            count += next_count
        }
    }
    count
}

fn main() {
    println!("Part 1 - test: {}", part_1(TEST_INPUT));
    println!("Part 1: {}", part_1(include_bytes!("../input.txt")));
    println!("Part 2 - test: {}", part_2(TEST_INPUT));
    println!("Part 2: {}", part_2(include_bytes!("../input.txt")));
}
