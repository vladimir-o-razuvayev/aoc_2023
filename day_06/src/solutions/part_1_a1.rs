pub fn solution(input: &str) -> u32 {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse::<u32>().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse::<u32>().unwrap());

    times.zip(distances).fold(1, |acc, (time, distance)| {
        let sqr = isqrt(time * time - 4 * distance);
        let mut start = (time - sqr).div_ceil(2);
        let mut end = (time + sqr) / 2;

        if start * (time - start) > distance {
            start -= 1;
        }
        if end * (time - end) > distance {
            end += 1;
        }
        acc * (end - start - 1)
    })
}

fn isqrt(n: u32) -> u32 {
    let mut bit = 1 << (n.ilog2() / 2);
    let mut root = bit;

    while bit > 1 {
        bit >>= 1;
        let next = root | bit;
        if next * next <= n {
            root = next;
        }
    }

    root
}
