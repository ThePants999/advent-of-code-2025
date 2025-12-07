use advent_of_code_rust_runner::{DayImplementation, Result, Context};

pub struct Day07;

pub struct Day07Context {
    state: Vec<usize>
}

impl DayImplementation for Day07 {
    type Output<'a> = usize;
    type Context<'a> = Day07Context;

    fn day(&self) -> u8 { 7 }
    fn example_input(&self) -> Option<&'static str> { Some(".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............") }
    fn example_part_1_result(&self) -> Option<Self::Output<'static>> { Some(21) }
    fn example_part_2_result(&self) -> Option<Self::Output<'static>> { Some(40) }

    fn execute_part_1<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, Self::Context<'a>)> {
        let mut lines = input.lines();
        let first_line = lines.next().context("Empty input")?;
        let initial_index = first_line.find('S').context("No starting position found")?;
        let splitter_sets: Vec<Vec<usize>> = lines
            .skip(1)
            .step_by(2)
            .map(|line| {
                line
                    .chars()
                    .enumerate()
                    .filter(|&(_i, c)| c == '^')
                    .map(|(i, _c)| i)
                    .collect::<Vec<usize>>()
            })
            .collect();

        let mut state = vec![0usize; first_line.len()];
        state[initial_index] = 1;

        let mut splits = 0usize;
        for splitters in splitter_sets {
            for splitter_ix in splitters {
                if state[splitter_ix] > 0 {
                    splits += 1;
                }

                state[splitter_ix-1] += state[splitter_ix];
                state[splitter_ix+1] += state[splitter_ix];
                state[splitter_ix] = 0;
            }
        }

        Ok((splits, Day07Context { state }))
    }

    fn execute_part_2<'a>(&self, _input: &'a str, ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        Ok(ctx.state.iter().sum())
    }
}
