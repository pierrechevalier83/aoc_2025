use geo::Covers;
use geo_types::{LineString, Polygon};
use geo_types::{Rect, coord};

const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

#[derive(Clone, Copy, Debug)]
enum Protocol {
    Part1,
    Part2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

fn rectangle_area(p: Point, q: Point) -> i64 {
    let x_max = p.x.max(q.x);
    let x_min = p.x.min(q.x);
    let y_max = p.y.max(q.y);
    let y_min = p.y.min(q.y);
    (x_max - x_min + 1) * (y_max - y_min + 1)
}

fn max_rectangle_area(input: &str, protocol: Protocol) -> i64 {
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
    let polygon = Polygon::new(
        LineString::from(
            points
                .iter()
                .cycle()
                .take(points.len() + 1)
                .map(|p| (p.x as f64, p.y as f64))
                .collect::<Vec<_>>(),
        ),
        vec![],
    );
    let distances = points
        .iter()
        .flat_map(|p| {
            points
                .iter()
                .map(|q| match protocol {
                    Protocol::Part1 => rectangle_area(*p, *q),
                    Protocol::Part2 => {
                        let rect = Rect::new(
                            coord! { x: p.x as f64, y: p.y as f64},
                            coord! { x: q.x as f64, y: q.y as f64},
                        );

                        if polygon.covers(&rect) {
                            rectangle_area(*p, *q)
                        } else {
                            0
                        }
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    *distances.iter().max().unwrap()
}

fn main() {
    println!(
        "Part 1 - test: {}",
        max_rectangle_area(TEST_INPUT, Protocol::Part1)
    );
    println!(
        "Part 1: {}",
        max_rectangle_area(include_str!("../input.txt"), Protocol::Part1)
    );
    println!(
        "Part 2 - test: {}",
        max_rectangle_area(TEST_INPUT, Protocol::Part2)
    );
    println!(
        "Part 2: {}",
        max_rectangle_area(include_str!("../input.txt"), Protocol::Part2)
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
