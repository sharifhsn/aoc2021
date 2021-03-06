#![feature(array_windows)]
#![feature(array_zip)]
#![feature(int_abs_diff)]

use color_eyre::Report;
use tracing::info;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

fn main() -> Result<(), Report> {
    common::log_setup()?;
    // info!("Day 1, Part 1: {}", day1::p1("day1.txt")?);
    // info!("Day 1, Part 2: {}", day1::p2("day1.txt")?);
    // info!("Day 2, Part 1: {}", day2::p1("day2.txt")?);
    // info!("Day 2, Part 2: {}", day2::p2("day2.txt")?);
    // info!("Day 3, Part 1: {}", day3::p1("day3.txt")?);
    // info!("Day 3, Part 2: {}", day3::p2("day3.txt")?);
    // info!("Day 4, Part 1: {}", day4::p1("day4.txt")?);
    // info!("Day 4, Part 2: {}", day4::p2("day4.txt")?);
    // info!("Day 5, Part 1: {}", day5::p1("day5.txt")?);
    // info!("Day 5, Part 2: {}", day5::p2("day5.txt")?);
    // info!("Day 6, Part 1: {}", day6::p1("day6.txt")?);
    // info!("Day 6, Part 2: {}", day6::p2("day6.txt")?);
    // info!("Day 7, Part 1: {}", day7::p1("day7.txt")?);
    // info!("Day 7, Part 2: {}", day7::p2("day7.txt")?);
    // info!("Day 8, Part 1: {}", day8::p1("day8.txt")?);
    // info!("Day 8, Part 2: {}", day8::p2("day8.txt")?);
    // info!("Day 9, Part 1: {}", day9::p1("day9.txt")?);
    // info!("Day 9, Part 2: {}", day9::p2("day9.txt")?);
    // info!("Day 10, Part 1: {}", day10::p1("day10.txt")?);
    // info!("Day 10, Part 2: {}", day10::p2("day10.txt")?);
    info!("Day 11, Part 1: {}", day11::p1("day11.txt")?);
    info!("Day 11, Part 2: {}", day11::p2("day11.txt")?);
    Ok(())
}
