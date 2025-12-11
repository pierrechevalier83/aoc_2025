use pathfinding::prelude::count_paths;

const TEST_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

fn part_1(input: &str) -> usize {
    let mut labels = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(": ").unwrap().0)
        .collect::<Vec<_>>();
    labels.push("out");
    labels.sort();
    let mut links: Vec<Vec<usize>> = Vec::new();
    links.resize(labels.len(), Vec::new());
    for (source, targets) in input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(": ").unwrap())
        .map(|(source, targets)| {
            (
                labels.binary_search(&source).unwrap(),
                targets
                    .split(" ")
                    .map(|target| labels.binary_search(&target).unwrap())
                    .collect::<Vec<_>>(),
            )
        })
    {
        links[source].extend(targets);
    }
    let me = labels.binary_search(&"you").unwrap();
    let out = labels.binary_search(&"out").unwrap();

    count_paths(me, |x| links[*x].clone(), |x| x == &out)
}

fn main() {
    println!("Part 1 - Test: {}", part_1(TEST_INPUT));
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
}
