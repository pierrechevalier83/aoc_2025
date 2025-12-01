const DIAL_STARTING_POSITION: i16 = 50;

#[derive(Clone, Copy)]
enum PasswordProtocol {
    Part1,
    Part2,
}

fn rotate_dial(
    position: i16,
    travel: i16,
    modulo: i16,
    protocol: PasswordProtocol,
) -> (i16, isize) {
    let next_position = (position + travel).rem_euclid(modulo);
    let count = match protocol {
        PasswordProtocol::Part1 => {
            if next_position == 0 {
                1
            } else {
                0
            }
        }
        PasswordProtocol::Part2 => {
            if travel >= 0 {
                (position + travel).div_euclid(modulo) as isize
            } else {
                if -travel == position {
                    1
                } else if -travel < position {
                    0
                } else {
                    (position + travel).abs().div_euclid(modulo) as isize + {
                        if position > 0 { 1 } else { 0 }
                    }
                }
            }
        }
    };
    (next_position, count)
}

fn count_zeroes(mut position: i16, rotations: &str, protocol: PasswordProtocol) -> usize {
    let mut num_zeroes = 0;
    let mut extra_zeroes = 0;
    for rotation in rotations.split("\n") {
        num_zeroes += extra_zeroes as usize;
        match rotation.chars().next() {
            Some('L') => {
                let num_clicks: i16 = rotation[1..].parse().unwrap();
                (position, extra_zeroes) = rotate_dial(position, -num_clicks, 100, protocol);
            }
            Some('R') => {
                let num_clicks: i16 = rotation[1..].parse().unwrap();
                (position, extra_zeroes) = rotate_dial(position, num_clicks, 100, protocol);
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
    println!(
        "Part 1: {}",
        count_zeroes(DIAL_STARTING_POSITION, input, PasswordProtocol::Part1)
    );
    println!(
        "Part 2: {}",
        count_zeroes(DIAL_STARTING_POSITION, input, PasswordProtocol::Part2)
    );
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
        assert_eq!(
            3,
            count_zeroes(DIAL_STARTING_POSITION, TEST_INPUT, PasswordProtocol::Part1)
        );
        assert_eq!(
            6,
            count_zeroes(DIAL_STARTING_POSITION, TEST_INPUT, PasswordProtocol::Part2)
        );
    }
    #[test]
    fn test_rotate_dial() {
        assert_eq!((0, 0), rotate_dial(0, 0, 5, PasswordProtocol::Part2));
        assert_eq!((4, 0), rotate_dial(0, 4, 5, PasswordProtocol::Part2));
        assert_eq!((0, 1), rotate_dial(0, 5, 5, PasswordProtocol::Part2));
        assert_eq!((1, 1), rotate_dial(0, 6, 5, PasswordProtocol::Part2));
        assert_eq!((0, 1), rotate_dial(4, 1, 5, PasswordProtocol::Part2));
        assert_eq!((1, 11), rotate_dial(0, 56, 5, PasswordProtocol::Part2));
        assert_eq!((2, 11), rotate_dial(1, 56, 5, PasswordProtocol::Part2));
        assert_eq!((0, 1), rotate_dial(1, -1, 5, PasswordProtocol::Part2));
        assert_eq!((4, 1), rotate_dial(1, -2, 5, PasswordProtocol::Part2));
        assert_eq!((4, 0), rotate_dial(0, -1, 5, PasswordProtocol::Part2));
        assert_eq!((4, 10), rotate_dial(0, -51, 5, PasswordProtocol::Part2));
        assert_eq!((3, 10), rotate_dial(4, -51, 5, PasswordProtocol::Part2));
    }
}
