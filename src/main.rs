mod day01;

use advent_of_code_rust_runner::{run_all, Day};

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Warn)
        .parse_default_env()
        .init();

    let days: Vec<Box<dyn Day>> = vec![
        Box::new(day01::Day01 {})
    ];
    run_all(&days);
}
