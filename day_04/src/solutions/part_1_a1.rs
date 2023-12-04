use std::collections::HashSet;

pub fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.split_once(" | ").unwrap())
        .map(|(win_nums, play_nums)| (parse_nums(win_nums), parse_nums(play_nums)))
        .map(|(win_nums, play_nums)| {
            let n = win_nums.intersection(&play_nums).count();
            if n != 0 {
                1 << n - 1
            } else {
                0
            }
        })
        .sum()
}

fn parse_nums(nums: &str) -> HashSet<u32> {
    nums.split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect()
}
