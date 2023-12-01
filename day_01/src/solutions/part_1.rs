pub fn solution(input: &str) -> u32 {
    input
        .lines()
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
