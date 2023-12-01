static NUMBERS_INNER: [(&str, &str); 9] = [
    ("o1e", "one"),
    ("t2o", "two"),
    ("th3ee", "three"),
    ("f4ur", "four"),
    ("f5ve", "five"),
    ("s6x", "six"),
    ("se7en", "seven"),
    ("ei8ht", "eight"),
    ("n9ne", "nine"),
];

pub fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut new_line: String = line.into();
            for (digit, number) in NUMBERS_INNER {
                while let Some(location) = new_line.find(number) {
                    new_line.replace_range(location..location + number.len(), digit);
                }
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
