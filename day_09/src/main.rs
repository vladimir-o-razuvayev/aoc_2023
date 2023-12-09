#![feature(test)]
#![feature(iter_map_windows)]
#![feature(iter_array_chunks)]
#![feature(iter_next_chunk)]
#![feature(iterator_try_collect)]
#![feature(ascii_char)]
mod solutions;

use solutions::part_1_a1;
use solutions::part_2_a1;

fn main() {
    println!(
        "Part 1 Attempt 1: {}\n",
        part_1_a1::solution(include_str!("input.txt"))
    );
    println!(
        "Part 2 Attempt 1: {}\n",
        part_2_a1::solution(include_str!("input.txt"))
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
    } // 273,964 ns/iter (+/- 6,427)

    #[bench]
    fn bench_part_2_a1(b: &mut Bencher) {
        let input = include_str!("input.txt");
        b.iter(|| crate::part_2_a1::solution(input));
    } // 265,307 ns/iter (+/- 10,450)
}
