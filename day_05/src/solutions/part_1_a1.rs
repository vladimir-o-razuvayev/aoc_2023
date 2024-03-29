pub fn solution(input: &str) -> u64 {
    let mut split_input = input.splitn(8, "\n\n");
    let seeds = parse_seeds(split_input.next().unwrap());

    split_input
        .fold(seeds, |acc, map| map_path(acc, &parse_maps(map)))
        .iter()
        .min()
        .unwrap()
        .to_owned()
}

fn map_path(origin: Vec<u64>, map: &Vec<(u64, u64, u64)>) -> Vec<u64> {
    origin
        .into_iter()
        .map(|seed| {
            for &(d, s, r) in map {
                if (seed >= s) && (seed < s + r) {
                    return seed + d - s;
                }
            }
            seed
        })
        .collect()
}

fn parse_seeds(nums: &str) -> Vec<u64> {
    nums.split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect()
}

fn parse_maps(nums: &str) -> Vec<(u64, u64, u64)> {
    nums.split_once(":\n")
        .unwrap()
        .1
        .lines()
        .map(|line| {
            let mut i = line
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap());
            (i.next().unwrap(), i.next().unwrap(), i.next().unwrap())
        })
        .collect()
}
