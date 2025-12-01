mod day01;
mod day02;

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
    ];

    let runner = Runner::new("2025", days).unwrap_or_else(|e| {
        eprintln!("Failed to initialize runner: {e}");
        std::process::exit(1);
    });
    runner.run();
}
