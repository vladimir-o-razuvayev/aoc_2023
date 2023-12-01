static NUMBERS: [(&str, &str); 12] = [
    ("82", "eightwo"),
    ("18", "oneight"),
    ("21", "twone"),
    ("1", "one"),
    ("2", "two"),
    ("3", "three"),
    ("4", "four"),
    ("5", "five"),
    ("6", "six"),
    ("7", "seven"),
    ("8", "eight"),
    ("9", "nine"),
];

pub fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut new_line: String = line.into();
            for (digit, number) in NUMBERS {
                new_line = new_line.replace(number, digit);
            }
            // println!("old: {}, new: {}", line, new_line);
            new_line
        })
        .collect::<Vec<String>>()
        .iter()
        .map(|line| {
            let (first, last) = line.chars().fold((10, 10), |t, c| {
                if let Some(d) = c.to_digit(10) {
                    if t.0 > 9 {
                        (d, d)
                    } else {
                        (t.0, d)
                    }
                } else {
                    t
                }
            });
            // println!("f: {}, l: {}", first, last);
            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .sum()
}
