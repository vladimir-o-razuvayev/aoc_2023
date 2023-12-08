use std::collections::HashMap;
use std::str;

pub fn solution(input: &str) -> u64 {
    let (instructions, network) = input.split_once("\n\n").unwrap();
    let map = parse(network);

    let current_keys = map
        .keys()
        .filter(|k| k[2] == b'A')
        .map(|k| *k)
        .collect::<Vec<[u8; 3]>>();
    // print_current_keys(&current_keys);
    // print!("Cycles: ");
    let distance = current_keys
        .iter()
        .map(|key| {
            let mut distance = 0;
            let mut current_key = key.clone();
            for char in instructions.chars().cycle() {
                distance += 1;
                if char == 'L' {
                    current_key = map.get(&current_key).unwrap().0;
                } else {
                    current_key = map.get(&current_key).unwrap().1;
                }
                if current_key[2] == b'Z' {
                    break;
                }
            }
            distance
        })
        // .inspect(|d| print!("{} ", d))
        .reduce(lcm)
        .unwrap();
    // println!();
    distance
}

fn parse(network: &str) -> HashMap<[u8; 3], ([u8; 3], [u8; 3])> {
    let mut map = HashMap::<[u8; 3], ([u8; 3], [u8; 3])>::new();
    for line in network.lines() {
        let mut iter = line.bytes();
        let key: [u8; 3] = iter.next_chunk().expect("key parse");
        iter.next_chunk::<4>().expect("first delineator parse");
        let left: [u8; 3] = iter.next_chunk().expect("left parse");
        iter.next_chunk::<2>().expect("second delineator parse");
        let right: [u8; 3] = iter.next_chunk().expect("right parse");
        map.insert(key, (left, right));
    }
    map
}

// fn print_current_keys(keys: &Vec<[u8; 3]>) {
//     println!(
//         "Keys: {:?}",
//         keys.iter()
//             .map(|k| str::from_utf8(k).unwrap())
//             .collect::<Vec<&str>>()
//     );
// }

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min)
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
