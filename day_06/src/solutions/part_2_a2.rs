pub fn solution(input: &str) -> Option<u128> {
    let mut lines = input.lines();
    let (_, time_str) = lines.next().unwrap().split_once(":")?;
    let time = time_str.replace(" ", "").parse::<u128>().ok()?;
    let (_, distance_str) = lines.next().unwrap().split_once(":")?;
    let distance = distance_str.replace(" ", "").parse::<u128>().ok()?;
    let root = isqrt(time * time - 4 * distance);
    let mut start = (time - root).div_ceil(2);
    let mut end = (time + root) / 2;

    if start * (time - start) > distance {
        start -= 1;
    }
    if end * (time - end) > distance {
        end += 1;
    }

    Some(end - start - 1)
}

// https://en.wikipedia.org/wiki/Integer_square_root
fn isqrt(n: u128) -> u128 {
    if n < 2 {
        return n;
    }

    let small_candidate = isqrt(n >> 2) << 1;
    let large_candidate = small_candidate + 1;
    if large_candidate.pow(2) > n {
        small_candidate
    } else {
        large_candidate
    }
}
