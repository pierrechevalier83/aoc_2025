use pathfinding::prelude::count_paths;

const TEST_INPUT_PART_1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

fn parse_input(input: &str) -> (Vec<&str>, Vec<Vec<usize>>) {
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
    (labels, links)
}

fn part_1(input: &str) -> usize {
    let (labels, links) = parse_input(input);
    let me = labels.binary_search(&"you").unwrap();
    let out = labels.binary_search(&"out").unwrap();

    count_paths(me, |x| links[*x].clone(), |x| x == &out)
}

const TEST_INPUT_PART_2: &str = "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

fn part_2(input: &str) -> usize {
    let (labels, links) = parse_input(input);

    let svr = labels.binary_search(&"svr").unwrap();
    let out = labels.binary_search(&"out").unwrap();
    let dac = labels.binary_search(&"dac").unwrap();
    let fft = labels.binary_search(&"fft").unwrap();

    let n_svr_dac = count_paths(svr, |x| links[*x].clone(), |x| x == &dac);
    let n_dac_fft = count_paths(dac, |x| links[*x].clone(), |x| x == &fft);
    let n_fft_out = count_paths(fft, |x| links[*x].clone(), |x| x == &out);
    let n_svr_fft = count_paths(svr, |x| links[*x].clone(), |x| x == &fft);
    let n_fft_dac = count_paths(fft, |x| links[*x].clone(), |x| x == &dac);
    let n_dac_out = count_paths(dac, |x| links[*x].clone(), |x| x == &out);
    n_svr_dac * n_dac_fft * n_fft_out + n_svr_fft * n_fft_dac * n_dac_out
}

fn main() {
    println!("Part 1 - Test: {}", part_1(TEST_INPUT_PART_1));
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2 - Test: {}", part_2(TEST_INPUT_PART_2));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
