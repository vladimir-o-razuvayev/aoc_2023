#![feature(test)]

use std::str;
use std::str::Lines;

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

static DIGITS: [(u8, &str); 9] = [
    (b'1', "one"),
    (b'2', "two"),
    (b'3', "three"),
    (b'4', "four"),
    (b'5', "five"),
    (b'6', "six"),
    (b'7', "seven"),
    (b'8', "eight"),
    (b'9', "nine"),
];

fn main() {
    part_1(include_str!("input.txt").lines());
    part_2(include_str!("input.txt").lines());
    part_2_optimized(include_str!("input.txt").lines());
    part_2_swvs(include_str!("input.txt").lines());
}

fn part_1(lines: Lines<'_>) {
    sum_digits(lines.collect())
}

fn part_2(lines: Lines<'_>) {
    sum_digits(
        lines
            .map(|line| {
                let mut new_line: String = line.into();
                for (digit, number) in NUMBERS {
                    new_line = new_line.replace(number, digit);
                }
                // println!("old: {}, new: {}", line, new_line);
                new_line
            })
            .collect(),
    )
}

fn part_2_optimized(lines: Lines<'_>) {
    sum_digits(
        lines
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
            .collect(),
    )
}

// Sliding Window Variable Size
fn part_2_swvs(lines: Lines<'_>) {
    let result = lines.fold(0, |sum, line| {
        let mut digits = [b'0'; 2];
        let mut l = 0;
        for r in 0..line.len() {
            if (line.as_bytes()[r] as char).is_numeric() {
                if digits[0] == b'0' {
                    digits[0] = line.as_bytes()[r];
                }
                digits[1] = line.as_bytes()[r];
                l = r + 1;
            } else {
                let slice = &line.as_bytes()[l..r + 1];
                if slice.len() > 2 {
                    if let Some((value, _)) =
                        DIGITS.iter().find(|(_i, d)| slice.ends_with(d.as_bytes()))
                    {
                        if digits[0] == b'0' {
                            digits[0] = *value;
                        }
                        digits[1] = *value;
                        l = r;
                    }
                }
            }
        }
        sum + str::from_utf8(&digits).unwrap().parse::<u32>().unwrap()
    });
    println!("{}", result);
}

fn sum_digits<T: AsRef<str>>(lines: Vec<T>) {
    let result: u32 = lines
        .iter()
        .map(|line| {
            let (first, last) = line.as_ref().chars().fold((10, 10), |t, c| {
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
        .sum();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let lines = include_str!("input.txt").lines();
        b.iter(|| crate::part_1(lines.to_owned()));
    } // 84,237 ns/iter (+/- 2,691)

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let lines = include_str!("input.txt").lines();
        b.iter(|| crate::part_2(lines.to_owned()));
    } // 1,400,767 ns/iter (+/- 28,149)

    #[bench]
    fn bench_part_2_optimized(b: &mut Bencher) {
        let lines = include_str!("input.txt").lines();
        b.iter(|| crate::part_2_optimized(lines.to_owned()));
    } // 637,571 ns/iter (+/- 21,023)

    #[bench]
    fn bench_part_2_swvs(b: &mut Bencher) {
        let lines = include_str!("input.txt").lines();
        b.iter(|| crate::part_2_swvs(lines.to_owned()));
    } // 107,035 ns/iter (+/- 3,847)
}
