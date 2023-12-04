use std::collections::HashSet;

pub fn solution(input: &str) -> u32 {
    let mut card_mult: Vec<u32> = vec![1; input.lines().count()];
    input
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.split_once(" | ").unwrap())
        .map(|(win_nums, play_nums)| (parse_nums(win_nums), parse_nums(play_nums)))
        .enumerate()
        .for_each(|(i, (win_nums, play_nums))| {
            let n = win_nums.intersection(&play_nums).count();
            if n != 0 {
                for j in (i + 1)..=(i + n as usize) {
                    card_mult[j] = card_mult[j] + card_mult[i];
                }
            }
            // println!("Card {}: {:?}", i + 1, card_mult);
        });
    card_mult.into_iter().sum()
}

fn parse_nums(nums: &str) -> HashSet<u32> {
    nums.split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect()
}
