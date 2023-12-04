pub fn solution(input: &str) -> u32 {
    input
        .lines()
        .map_windows(|[top, mid, bot]| {
            let mut sum_num = 0;
            let mut in_num = false;
            let mut start_num = 0;
            for (i, c) in mid.bytes().enumerate() {
                if in_num {
                    if !c.is_ascii_digit() {
                        if special_around(top, mid, bot, start_num - 1, i + 1) {
                            let cur_num = mid.get(start_num..i).unwrap().parse::<u32>().unwrap();
                            // println!("{}", cur_num);
                            sum_num = sum_num + cur_num;
                        }
                        in_num = false;
                    }
                } else {
                    if c.is_ascii_digit() {
                        in_num = true;
                        start_num = i;
                    }
                }
            }
            sum_num
        })
        .sum()
}

fn special_around(top: &str, mid: &str, bot: &str, left: usize, right: usize) -> bool {
    [top, mid, bot]
        .into_iter()
        .map(|slice| slice.get(left..right).unwrap())
        .any(|slice| slice.bytes().any(|c| !(c.is_ascii_digit() || c == b'.')))
}
