use std::collections::HashMap;

static SCALE: usize = 1000000;

pub fn solution(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.len() / width;
    let mut hit_rows = vec![std::usize::MAX; height];
    let mut hit_cols = vec![std::usize::MAX; width];
    let mut galaxies = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, _) in line.bytes().enumerate().filter(|(_, b)| *b == b'#') {
            galaxies.push((x, y));
            hit_rows[y] = 1;
            hit_cols[x] = 1;
        }
    }

    for i in 1..height {
        if hit_rows[i] < std::usize::MAX {
            hit_rows[i] = hit_rows[i - 1] + 1;
        } else {
            hit_rows[i] = hit_rows[i - 1] + SCALE;
        }
    }

    for i in 1..width {
        if hit_cols[i] < std::usize::MAX {
            hit_cols[i] = hit_cols[i - 1] + 1;
        } else {
            hit_cols[i] = hit_cols[i - 1] + SCALE;
        }
    }

    let mut distances = HashMap::with_capacity(galaxies.len().pow(2));

    for (i, &gi) in galaxies.iter().enumerate() {
        for (j, &gj) in galaxies.iter().enumerate().filter(|(j, _)| i != *j) {
            let small = i.min(j);
            let big = i.max(j);
            let (x0, y0) = (hit_cols[gi.0], hit_rows[gi.1]);
            let (x1, y1) = (hit_cols[gj.0], hit_rows[gj.1]);
            distances.insert((small, big), x0.abs_diff(x1) + y0.abs_diff(y1));
        }
    }

    distances.values().sum()
}
