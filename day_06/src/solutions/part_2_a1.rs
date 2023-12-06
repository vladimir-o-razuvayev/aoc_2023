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
    let mut shift = 2;
    while (n >> shift) != 0 {
        shift += 2;
    }

    let mut result: u128 = 0;
    while shift >= 0 {
        result = result << 1;
        let large_candidate = result + 1;
        if large_candidate.pow(2) <= n >> shift {
            result = large_candidate;
        }
        shift -= 2;
    }

    result
}
