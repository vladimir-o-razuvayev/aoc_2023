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

// Sliding Window Variable Size
pub fn solution(input: &str) -> u32 {
    input.lines().fold(0, |sum, line| {
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
    })
}
