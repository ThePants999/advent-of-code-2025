use advent_of_code_rust_runner::{DayImplementation, Result};

pub struct Day04;

pub struct Day04Context {

}

impl DayImplementation for Day04 {
    type Output<'a> = u32;
    type Context<'a> = Day04Context;

    fn day(&self) -> u8 { 4 }
    fn example_input(&self) -> Option<&'static str> { Some("") }
    fn example_part_1_result(&self) -> Option<Self::Output<'static>> { Some(0) }
    fn example_part_2_result(&self) -> Option<Self::Output<'static>> { Some(0) }

    fn execute_part_1<'a>(&self, _input: &'a str) -> Result<(Self::Output<'a>, Self::Context<'a>)> {
        Ok((0, Day04Context {}))
    }

    fn execute_part_2<'a>(&self, _input: &'a str, _ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        Ok(0)
    }
}
