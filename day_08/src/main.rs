use union_find::{QuickUnionUf, UnionBySize, UnionFind};

const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

#[derive(Clone, Copy, Debug)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    fn square_straight_line_distance(self, other: Point) -> u64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

#[derive(Debug)]
struct Points {
    points: Vec<Point>,
    distances: Vec<Vec<u64>>,
}

impl Points {
    fn new(input: &str) -> Self {
        let points = input
            .split('\n')
            .filter_map(|line| {
                if line.is_empty() {
                    None
                } else {
                    let mut coords = line.splitn(3, ',').map(|coord| coord.parse().unwrap());
                    let x = coords.next().unwrap();
                    let y = coords.next().unwrap();
                    let z = coords.next().unwrap();
                    Some(Point { x, y, z })
                }
            })
            .collect::<Vec<_>>();
        let distances = points
            .iter()
            .map(|p| {
                points
                    .iter()
                    .map(|q| p.square_straight_line_distance(*q))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Points { points, distances }
    }
    fn circuits_after_n_connections(&self, n: usize) -> impl Iterator<Item = usize> {
        let mut circuits = QuickUnionUf::<UnionBySize>::new(self.points.len());
        let mut index = 0;
        let mut min_distance = 0;
        while index < n {
            let next_connection = self
                .distances
                .iter()
                .enumerate()
                .flat_map(|(i, neighbours)| {
                    neighbours
                        .iter()
                        .enumerate()
                        .filter(|(_j, neighbour)| neighbour > &&min_distance)
                        .map(|(j, d)| (i, j, *d))
                        .collect::<Vec<_>>()
                })
                .min_by(|l, r| l.2.partial_cmp(&r.2).unwrap())
                .unwrap();
            min_distance = next_connection.2;
            let (i, j, _d) = next_connection;
            circuits.union(i, j);
            index += 1;
        }

        let mut sizes = Vec::new();
        sizes.resize(self.points.len(), 0);
        for i in 0..self.points.len() {
            sizes[circuits.find(i)] += 1;
        }
        sizes.sort();
        sizes.into_iter().rev()
    }
}

fn main() {
    let test: usize = Points::new(TEST_INPUT)
        .circuits_after_n_connections(10)
        .take(3)
        .product();
    println!("Part 1 - test: {}", test);
    let actual: usize = Points::new(include_str!("../input.txt"))
        .circuits_after_n_connections(1000)
        .take(3)
        .product();
    println!("Part 1: {}", actual);
}
