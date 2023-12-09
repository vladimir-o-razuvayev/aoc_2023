pub fn solution(input: &str) -> i128 {
    input
        .lines()
        .map(|line| line.split(' ').map(|x| str::parse(x).unwrap()).collect())
        .map(|mut s: Vec<i128>| {
            s.reverse();
            difference(s)
        })
        .sum()
}

fn difference(sequence: Vec<i128>) -> i128 {
    let differences: Vec<i128> = sequence.iter().map_windows(|[&x, &y]| y - x).collect();
    if differences.iter().all(|&x| x == 0) {
        return *sequence.last().unwrap();
    } else {
        return sequence.last().unwrap() + difference(differences);
    }
}
