use advent_of_code_rust_runner::{DayImplementation, Result};

pub struct Day02;

impl DayImplementation for Day02 {
    type Output<'a> = &'a str;
    type Context<'a> = ();

    fn day(&self) -> u8 { 2 }

    fn execute_part_1<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, Self::Context<'a>)> {
        let words: Vec<&'a str> = input.split_whitespace().collect();
        Ok((words[0], ()))
    }

    fn execute_part_2<'a>(&self, input: &'a str, _ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        let words: Vec<&'a str> = input.split_whitespace().collect();
        Ok(words[1])
    }
}