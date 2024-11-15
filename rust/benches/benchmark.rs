#![allow(unstable_features)]
#![feature(test)]

extern crate test;

macro_rules! benchmark_aoc {
    ($year:tt, $day:tt) => {
        mod $day {
            use puzzles::aoc::$year::$day::*;
            use test::Bencher;

            #[bench]
            fn part1_bench(b: &mut Bencher) {
                let year = puzzles::util::parse::extract_integer(stringify!($year)) as u16;
                let day = puzzles::util::parse::extract_integer(stringify!($day)) as u8;
                let input: String = puzzles::util::input::get_aoc_input(year, day);
                b.iter(|| part1(&input));
            }

            #[bench]
            fn part2_bench(b: &mut Bencher) {
                let year = puzzles::util::parse::extract_integer(stringify!($year)) as u16;
                let day = puzzles::util::parse::extract_integer(stringify!($day)) as u8;
                let input: String = puzzles::util::input::get_aoc_input(year, day);
                b.iter(|| part2(&input));
            }
        }
    };
}

mod aoc {
    mod year2015 {
        benchmark_aoc!(year2015, day01);
        benchmark_aoc!(year2015, day02);
        benchmark_aoc!(year2015, day03);
        benchmark_aoc!(year2015, day04);
    }
}
