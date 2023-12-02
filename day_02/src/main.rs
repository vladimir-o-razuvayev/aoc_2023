#![feature(test)]
mod solutions;

use solutions::part_1_a1;

fn main() {
    println!(
        "Part 1 Attempt 1: {}",
        part_1_a1::solution(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1_a1(b: &mut Bencher) {
        let input = include_str!("input.txt");
        b.iter(|| crate::part_1_a1::solution(input));
    } // 22,246 ns/iter (+/- 477)
}
