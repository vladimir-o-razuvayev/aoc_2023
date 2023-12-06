#![feature(test)]
#![feature(iter_map_windows)]
#![feature(iter_array_chunks)]
mod solutions;

use solutions::part_1_a1;
use solutions::part_2_a1;
use solutions::part_2_a2;

fn main() {
    println!(
        "Part 1 Attempt 1: {}",
        part_1_a1::solution(include_str!("input.txt"))
    );
    println!(
        "Part 2 Attempt 1: {:?}",
        part_2_a1::solution(include_str!("input.txt"))
    );
    println!(
        "Part 2 Attempt 2: {:?}",
        part_2_a2::solution(include_str!("input.txt"))
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
    } // 115 ns/iter (+/- 5)

    #[bench]
    fn bench_part_2_a1(b: &mut Bencher) {
        let input = include_str!("input.txt");
        b.iter(|| crate::part_2_a1::solution(input));
    } // 447 ns/iter (+/- 6)

    #[bench]
    fn bench_part_2_a2(b: &mut Bencher) {
        let input = include_str!("input.txt");
        b.iter(|| crate::part_2_a2::solution(input));
    } // 444 ns/iter (+/- 11)
}
