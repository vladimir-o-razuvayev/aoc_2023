use std::str;

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

// Recursive
pub fn solution(input: &str) -> u32 {
    input.lines().fold(0, |sum, line| {
        let mut digits = [b'0'; 2];
        digits[0] = find_first_number(line);
        digits[1] = find_last_number(line);
        sum + str::from_utf8(&digits).unwrap().parse::<u32>().unwrap()
    })
}

fn find_first_number(line: &str) -> u8 {
    if (line.as_bytes()[0] as char).is_ascii_digit() {
        line.as_bytes()[0]
    } else if let Some((value, _)) = DIGITS
        .iter()
        .find(|(_i, d)| line.as_bytes().starts_with(d.as_bytes()))
    {
        *value
    } else {
        find_first_number(&line[1..line.len()])
    }
}

fn find_last_number(line: &str) -> u8 {
    if (line.as_bytes()[line.len() - 1] as char).is_ascii_digit() {
        line.as_bytes()[line.len() - 1]
    } else if let Some((value, _)) = DIGITS
        .iter()
        .find(|(_i, d)| line.as_bytes().ends_with(d.as_bytes()))
    {
        *value
    } else {
        find_last_number(&line[0..line.len() - 1])
    }
}
