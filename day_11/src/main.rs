#![feature(test)]
#![feature(iter_map_windows)]
#![feature(iter_array_chunks)]
#![feature(iter_next_chunk)]
#![feature(iterator_try_collect)]
#![feature(ascii_char)]
mod solutions;

use solutions::part_1_a1;
use solutions::part_1_a2;
use solutions::part_2_a1;
use solutions::part_2_a2;

fn main() {
    println!(
        "Part 1 Attempt 1: {}\n",
        part_1_a1::solution(include_str!("input.txt"))
    );
    println!(
        "Part 1 Attempt 2: {}\n",
        part_1_a2::solution(include_str!("input.txt"))
    );
    println!(
        "Part 2 Attempt 1: {}\n",
        part_2_a1::solution(include_str!("input.txt"))
    );
    println!(
        "Part 2 Attempt 2: {}\n",
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
    } // 7,091,174 ns/iter (+/- 633,490)

    #[bench]
    fn bench_part_1_a2(b: &mut Bencher) {
        let input = include_str!("input.txt");
        b.iter(|| crate::part_1_a2::solution(input));
    } // 6,001,241 ns/iter (+/- 784,756)

    #[bench]
    fn bench_part_2_a1(b: &mut Bencher) {
        let input = include_str!("input.txt");
        b.iter(|| crate::part_2_a1::solution(input));
    } // 7,115,004 ns/iter (+/- 699,885)

    #[bench]
    fn bench_part_2_a2(b: &mut Bencher) {
        let input = include_str!("input.txt");
        b.iter(|| crate::part_2_a2::solution(input));
    } // 5,910,520 ns/iter (+/- 385,270)
}
