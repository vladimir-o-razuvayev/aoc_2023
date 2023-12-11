use std::collections::HashMap;

pub fn solution(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.len() / width;
    let mut hit_rows = vec![false; height];
    let mut hit_cols = vec![false; width];
    let mut galaxies = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, _) in line.bytes().enumerate().filter(|(_, b)| *b == b'#') {
            galaxies.push((x, y));
            hit_rows[y] = true;
            hit_cols[x] = true;
        }
    }

    let mut x_offset = 0;

    for (x, _) in hit_cols.iter().enumerate().filter(|(_, &b)| !b) {
        let mut expanded_galaxies = galaxies.clone();
        for (i, (gx, gy)) in galaxies
            .iter()
            .enumerate()
            .filter(|(_, (gx, _))| *gx > x + x_offset)
        {
            expanded_galaxies[i] = (gx + 1, *gy);
        }
        x_offset += 1;
        galaxies = expanded_galaxies;
    }

    let mut y_offset = 0;

    for (y, _) in hit_rows.iter().enumerate().filter(|(_, &b)| !b) {
        let mut expanded_galaxies = galaxies.clone();
        for (i, (gx, gy)) in galaxies
            .iter()
            .enumerate()
            .filter(|(_, (_, gy))| *gy > y + y_offset)
        {
            expanded_galaxies[i] = (*gx, gy + 1)
        }
        y_offset += 1;
        galaxies = expanded_galaxies;
    }

    let mut distances = HashMap::new();

    for (i, &gi) in galaxies.iter().enumerate() {
        for (j, &gj) in galaxies.iter().enumerate().filter(|(j, _)| i != *j) {
            let small = i.min(j);
            let big = i.max(j);
            distances.insert((small, big), gi.0.abs_diff(gj.0) + gi.1.abs_diff(gj.1));
        }
    }

    distances.values().sum()
}
