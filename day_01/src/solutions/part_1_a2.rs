use std::str;

// Recursive
pub fn solution(input: &str) -> u32 {
    input.lines().fold(0, |sum, line| {
        sum + (((find_first_number(line) - b'0') * 10) + find_last_number(line) - b'0') as u32
    })
}

fn find_first_number(line: &str) -> u8 {
    if (line.as_bytes()[0] as char).is_ascii_digit() {
        line.as_bytes()[0]
    } else {
        find_first_number(&line[1..line.len()])
    }
}

fn find_last_number(line: &str) -> u8 {
    if (line.as_bytes()[line.len() - 1] as char).is_ascii_digit() {
        line.as_bytes()[line.len() - 1]
    } else {
        find_last_number(&line[0..line.len() - 1])
    }
}
