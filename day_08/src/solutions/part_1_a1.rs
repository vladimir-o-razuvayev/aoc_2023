use std::collections::HashMap;

pub fn solution(input: &str) -> u64 {
    let (instructions, network) = input.split_once("\n\n").unwrap();

    let map = parse(network);
    let mut distance = 0;
    let mut current_key = [b'A'; 3];
    let target_key = [b'Z'; 3];
    for char in instructions.chars().cycle() {
        distance += 1;
        if char == 'L' {
            current_key = map.get(&current_key).unwrap().0;
        } else {
            current_key = map.get(&current_key).unwrap().1;
        }
        if current_key == target_key {
            break;
        }
    }
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
