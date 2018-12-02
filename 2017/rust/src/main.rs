#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
use simplelog::*;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod aoc_util;
use std::time::Instant;

fn main() {
    CombinedLogger::init(vec![TermLogger::new(LogLevelFilter::Error).unwrap()]).unwrap();

    let days: Vec<fn()> = vec![
        day1::run,
        day2::run,
        day3::run,
        day4::run,
        day5::run,
        day6::run,
        day7::run,
        day8::run,
        day9::run,
        day10::run,
        day11::run,
        day12::run,
        day13::run,
        day14::run,
        day15::run,
        day16::run,
        day17::run,
        day18::run,
        day19::run,
        day20::run,
        day21::run,
        day22::run,
        day23::run,
        day24::run,
        day25::run,
    ];

    for (index, &day) in days.iter().enumerate() {
        run_day(index + 1, day);
    }
}

fn run_day(day: usize, func: fn()) {
    println!("Day {}", day);
    let now = Instant::now();
    func();
    println!(
        "Took {}.{}s\n",
        now.elapsed().as_secs(),
        now.elapsed().subsec_millis()
    );
}
