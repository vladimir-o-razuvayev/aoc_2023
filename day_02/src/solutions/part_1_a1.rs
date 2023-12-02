const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

pub fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (game, rounds) = line.split_once(": ").unwrap();
            if rounds.split("; ").all(|round| {
                round.split(", ").all(|pair| {
                    let (num, color) = pair.split_once(' ').unwrap();
                    let pn = num.parse::<u32>().unwrap();
                    match color {
                        "red" => pn <= RED,
                        "blue" => pn <= BLUE,
                        "green" => pn <= GREEN,
                        _ => panic!(),
                    }
                })
            }) {
                game.split_once(' ').unwrap().1.parse::<u32>().unwrap()
            } else {
                0
            }
        })
        .sum()
}
