const DIAL_STARTING_POSITION: i16 = 50;

fn rotate_dial(a: i16, b: i16, modulo: i16) -> i16 {
    (a + b).rem_euclid(modulo)
}

fn count_zeroes(mut position: i16, rotations: &str) -> usize {
    let mut num_zeroes = 0;
    for rotation in rotations.split("\n") {
        if position == 0 {
            num_zeroes += 1;
        }
        match rotation.chars().next() {
            Some('L') => {
                let num_clicks: i16 = rotation[1..].parse().unwrap();
                position = rotate_dial(position, -num_clicks, 100);
            }
            Some('R') => {
                let num_clicks: i16 = rotation[1..].parse().unwrap();
                position = rotate_dial(position, num_clicks, 100);
            }
            None => {
                break;
            }
            _ => {
                panic!("Invalid line: {}", rotation);
            }
        }
    }
    num_zeroes
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", count_zeroes(DIAL_STARTING_POSITION, input));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    use super::*;
    #[test]
    fn test_num_zeroes() {
        assert_eq!(3, count_zeroes(DIAL_STARTING_POSITION, TEST_INPUT));
    }
}
