use advent_of_code_rust_runner::{DayImplementation, Result, Context};

pub struct Day07;

pub struct Day07Context {
    width: usize,
    start: usize,
    splitter_sets: Vec<Vec<usize>>,
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
        let splitter_sets: Vec<Vec<usize>> = input
            .lines()
            .skip(2)
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

        let first_line = input.lines().next().context("Empty input")?;
        let initial_index = first_line.find('S').context("No starting position found")?;
        let mut state = vec![false; first_line.len()];
        state[initial_index] = true;

        let mut splits = 0usize;
        for splitters in splitter_sets.iter() {
            for splitter_ix in splitters {
                if state[*splitter_ix] {
                    splits += 1;
                }

                state[*splitter_ix-1] = true;
                state[*splitter_ix+1] = true;
                state[*splitter_ix] = false;
            }
        }

        Ok((splits, Day07Context { width: first_line.len(), start: initial_index, splitter_sets }))
    }

    fn execute_part_2<'a>(&self, _input: &'a str, ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        let mut state = vec![0usize; ctx.width];
        state[ctx.start] = 1;
        for splitters in ctx.splitter_sets {
            for splitter_ix in splitters {
                state[splitter_ix-1] += state[splitter_ix];
                state[splitter_ix+1] += state[splitter_ix];
                state[splitter_ix] = 0;
            }
        }
        let timelines = state.iter().sum();
        Ok(timelines)
    }
}
