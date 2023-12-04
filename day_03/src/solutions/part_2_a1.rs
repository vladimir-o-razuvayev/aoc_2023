use std::collections::HashMap;

pub fn solution(input: &str) -> u32 {
    let mut gear_ratios: HashMap<(usize, usize), u32> = HashMap::new();
    input
        .lines()
        .enumerate()
        .map_windows(|[(_, top), (ln, mid), (_, bot)]| {
            let mut in_num = false;
            let mut start_num = 0;
            let mut sum_num = 0;
            for (i, c) in mid.bytes().enumerate() {
                if in_num {
                    if !c.is_ascii_digit() {
                        // println!("{} {}", start_num, i);
                        if let Some(star) = gear_around(top, mid, bot, start_num - 1, i + 1, *ln) {
                            // println!("{:?}", star);
                            let cur_num = mid.get(start_num..i).unwrap().parse::<u32>().unwrap();
                            if let Some(old_num) = gear_ratios.get(&star) {
                                // println!("old num: {}, new_num: {}", old_num, cur_num);
                                sum_num = sum_num + (old_num * cur_num);
                            } else {
                                gear_ratios.insert(star, cur_num);
                            }
                        }
                        in_num = false;
                    }
                } else if c.is_ascii_digit() {
                    in_num = true;
                    start_num = i;
                }
            }
            sum_num
        })
        .sum()
}

fn gear_around(
    top: &str,
    mid: &str,
    bot: &str,
    left: usize,
    right: usize,
    mid_line_number: usize,
) -> Option<(usize, usize)> {
    for (i, slice) in [top, mid, bot]
        .into_iter()
        .map(|slice| slice.get(left..right).unwrap())
        .enumerate()
    {
        if let Some((j, _)) = slice.bytes().enumerate().find(|(_, c)| *c == b'*') {
            return Some((left + j, mid_line_number + i - 1));
        }
    }
    None
}
