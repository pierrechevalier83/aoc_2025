const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn rectangle_area(p: Point, q: Point) -> usize {
    let x_max = p.x.max(q.x);
    let x_min = p.x.min(q.x);
    let y_max = p.y.max(q.y);
    let y_min = p.y.min(q.y);
    (x_max - x_min + 1) * (y_max - y_min + 1)
}

fn max_rectangle_area(input: &str) -> usize {
    let points = input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                let mut coords = line.splitn(2, ',').map(|coord| coord.parse().unwrap());
                let x = coords.next().unwrap();
                let y = coords.next().unwrap();
                Some(Point { x, y })
            }
        })
        .collect::<Vec<_>>();
    let distances = points
        .iter()
        .flat_map(|p| {
            points
                .iter()
                .map(|q| rectangle_area(*p, *q))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    *distances.iter().max().unwrap()
}

fn main() {
    println!("Part 1 - test: {}", max_rectangle_area(TEST_INPUT));
    println!(
        "Part 1: {}",
        max_rectangle_area(include_str!("../input.txt"))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        assert_eq!(
            24,
            rectangle_area(Point { x: 2, y: 5 }, Point { x: 9, y: 7 })
        );
        assert_eq!(
            6,
            rectangle_area(Point { x: 7, y: 3 }, Point { x: 2, y: 3 })
        );
    }
}
