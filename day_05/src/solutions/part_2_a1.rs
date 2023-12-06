pub fn solution(input: &str) -> u64 {
    let mut split_input = input.splitn(8, "\n\n");
    let seeds = parse_seeds(split_input.next().unwrap());

    split_input
        .fold(seeds, |acc, map| map_path(acc, &parse_maps(map)))
        .iter()
        .min_by(|x, y| x[0].cmp(&y[0]))
        .unwrap()
        .first()
        .unwrap()
        .to_owned()
}

fn map_path(mut origin: Vec<[u64; 2]>, map: &Vec<(u64, u64, u64)>) -> Vec<[u64; 2]> {
    let mut destination: Vec<[u64; 2]> = vec![];
    while origin.len() > 0 {
        // println!("origin: {:?}", origin);
        let mut not_found = true;
        let [seed, range] = origin.pop().unwrap();
        for &(d, s, r) in map {
            // println!("map: {} {} {}", d, s, r);
            if seed >= s + r {
                ();
            } else if seed + range < s {
                ();
            } else if seed >= s && seed + range <= s + r {
                // println!("inside: {} {}", seed, range);
                not_found = false;
                destination.push([seed + d - s, range]);
                break;
            } else if seed < s && seed + range >= s + r {
                // println!("envelop: {} {}", seed, range);
                not_found = false;
                destination.push([d, r]);
                origin.push([seed, s - seed]);
                origin.push([s + r, seed + range - s - r]);
                break;
            } else if seed < s && seed + range > s {
                // println!("overlap left: {} {}", seed, range);
                not_found = false;
                destination.push([d, seed + range - s]);
                origin.push([seed, s - seed]);
                break;
            } else if seed >= s && seed + range > s + r {
                // println!("overlap right: {} {}", seed, range);
                not_found = false;
                destination.push([seed + d - s, s + r - seed]);
                origin.push([s + r, seed + range - s - r]);
                break;
            }
        }
        if not_found {
            destination.push([seed, range]);
        }
    }
    // println!("destination: {:?}", destination);
    destination
}

fn parse_seeds(nums: &str) -> Vec<[u64; 2]> {
    nums.split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .array_chunks::<2>()
        .collect()
}

fn parse_maps(nums: &str) -> Vec<(u64, u64, u64)> {
    let (_title, maps) = nums.split_once(":\n").unwrap();
    // println!("{:?}", title);
    maps.lines()
        .map(|line| {
            let mut i = line
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap());
            (i.next().unwrap(), i.next().unwrap(), i.next().unwrap())
        })
        .collect()
}
