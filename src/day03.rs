use advent_of_code_rust_runner::{DayImplementation, Result};

pub struct Day03;

pub struct Day03Context {
    banks: Vec<Vec<u64>>
}

impl DayImplementation for Day03 {
    type Output<'a> = u64;
    type Context<'a> = Day03Context;

    fn day(&self) -> u8 { 3 }
    fn example_input(&self) -> Option<&'static str> { Some("987654321111111
811111111111119
234234234234278
818181911112111") }
    fn example_part_1_result(&self) -> Option<Self::Output<'static>> { Some(357) }
    fn example_part_2_result(&self) -> Option<Self::Output<'static>> { Some(3121910778619) }

    fn execute_part_1<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, Self::Context<'a>)> {
        let banks = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect()
            })
            .collect::<Vec<Vec<u64>>>();

        Ok((get_total_joltage(&banks, 2), Day03Context { banks }))
    }

    fn execute_part_2<'a>(&self, _input: &'a str, ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        Ok(get_total_joltage(&ctx.banks, 12))
    }
}

fn get_total_joltage(banks: &Vec<Vec<u64>>, num_batteries: usize) -> u64 {
    banks
        .iter()
        .map(|bank| get_joltage(bank, num_batteries))
        .sum()
}

fn get_joltage(bank: &Vec<u64>, num_batteries: usize) -> u64 {
    let mut joltage: u64 = 0;
    let mut remaining_batteries = num_batteries;
    let mut start_index: usize = 0;
    while remaining_batteries > 0 {
        let (index, digit) = bank[start_index..bank.len() - remaining_batteries + 1]
            .iter()
            .enumerate()
            .reduce(|(max_index, max_digit), (current_index, current_digit)| {
                if current_digit > max_digit {
                    (current_index, current_digit)
                } else {
                    (max_index, max_digit)
                }
            }).unwrap();
        joltage = joltage * 10 + digit;
        remaining_batteries -= 1;
        start_index = start_index + index + 1;
    }
    joltage
}