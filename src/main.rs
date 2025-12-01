mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

use advent_of_code_rust_runner::{Runner, Day};

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Warn)
        .filter_module("advent_of_code_rust_runner", log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let days: Vec<Box<dyn Day>> = vec![
        Box::new(day01::Day01 {}),
        Box::new(day02::Day02 {}),
        Box::new(day03::Day03 {}),
        Box::new(day04::Day04 {}),
        Box::new(day05::Day05 {}),
        Box::new(day06::Day06 {}),
        Box::new(day07::Day07 {}),
        Box::new(day08::Day08 {}),
        Box::new(day09::Day09 {}),
        Box::new(day10::Day10 {}),
        Box::new(day11::Day11 {}),
        Box::new(day12::Day12 {}),
    ];

    let runner = Runner::new("2025", days).unwrap_or_else(|e| {
        eprintln!("Failed to initialize runner: {e}");
        std::process::exit(1);
    });
    runner.run();
}
