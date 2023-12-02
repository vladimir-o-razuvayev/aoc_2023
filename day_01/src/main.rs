#![feature(test)]
mod solutions;

use solutions::part_1_a1;
use solutions::part_1_a2;
use solutions::part_2_a1;
use solutions::part_2_a2;
use solutions::part_2_a3;
use solutions::part_2_a4;

fn main() {
    println!(
        "Part 1 Attempt 1: {}",
        part_1_a1::solution(include_str!("input.txt"))
    );
    println!(
        "Part 1 Attempt 2: {}",
        part_1_a2::solution(include_str!("input.txt"))
    );
    println!(
        "Part 2 Attempt 1: {}",
        part_2_a1::solution(include_str!("input.txt"))
    );
    println!(
        "Part 2 Attempt 2: {}",
        part_2_a2::solution(include_str!("input.txt"))
    );
    println!(
        "Part 2 Attempt 3: {}",
        part_2_a3::solution(include_str!("input.txt"))
    );
    println!(
        "Part 2 Attempt 4: {}",
        part_2_a4::solution(include_str!("input.txt"))
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
    } // 85,428 ns/iter (+/- 2,533)

    #[bench]
    fn bench_part_1_a2(b: &mut Bencher) {
        let input = include_str!("input.txt");
        b.iter(|| crate::part_1_a2::solution(input));
    } // 23,738 ns/iter (+/- 7,055)

    #[bench]
    fn bench_part_2_a1(b: &mut Bencher) {
        let input = include_str!("input.txt");
        b.iter(|| crate::part_2_a1::solution(input));
    } // 1,313,424 ns/iter (+/- 32,726)

    #[bench]
    fn bench_part_2_a2(b: &mut Bencher) {
        let input = include_str!("input.txt");
        b.iter(|| crate::part_2_a2::solution(input));
    } // 608,747 ns/iter (+/- 27,332)

    #[bench]
    fn bench_part_2_a3(b: &mut Bencher) {
        let input = include_str!("input.txt");
        b.iter(|| crate::part_2_a3::solution(input));
    } // 96,481 ns/iter (+/- 8,335)

    #[bench]
    fn bench_part_2_a4(b: &mut Bencher) {
        let input = include_str!("input.txt");
        b.iter(|| crate::part_2_a4::solution(input));
    } // 33,174 ns/iter (+/- 8,023)
}
