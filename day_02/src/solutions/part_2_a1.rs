use std::cmp;

pub fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, rounds) = line.split_once(": ").unwrap();
            let (mut max_red, mut max_blue, mut max_green) = (0, 0, 0);
            rounds.split("; ").for_each(|round| {
                round.split(", ").for_each(|pair| {
                    let (num, color) = pair.split_once(' ').unwrap();
                    let pn = num.parse::<u32>().unwrap();
                    match color {
                        "red" => max_red = cmp::max(max_red, pn),
                        "blue" => max_blue = cmp::max(max_blue, pn),
                        "green" => max_green = cmp::max(max_green, pn),
                        _ => panic!(),
                    }
                })
            });
            // println!("red: {}, blue: {}, green: {}", max_red, max_blue, max_green);
            max_red * max_blue * max_green
        })
        .sum()
}
