use advent_of_code_rust_runner::{DayImplementation, Result, Context};
use std::collections::HashSet;

pub struct Day07;

pub struct Day07Context {
    width: usize,
    start: usize,
    splitter_sets: Vec<HashSet<usize>>,
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
        let splitter_sets: Vec<HashSet<usize>> = input[1..]
            .lines()
            .map(|line| {
                line
                    .chars()
                    .enumerate()
                    .filter(|&(_i, c)| c == '^')
                    .map(|(i, _c)| i)
                    .collect::<HashSet<usize>>()
            })
            .filter(|set| !set.is_empty())
            .collect();

        let first_line = input.lines().next().context("Empty input")?;
        let initial_index = first_line.find('S').context("No starting position found")?;
        let mut beams = HashSet::from([initial_index]);
        let mut splits = 0usize;
        for splitters in &splitter_sets {
            let split_points = beams.intersection(splitters).copied().collect::<HashSet<usize>>();
            splits += split_points.len();
            beams = beams
                .difference(&split_points)
                .copied()
                .collect::<HashSet<usize>>()
                .union(&split_points.iter().flat_map(|&i| [i-1, i+1]).collect::<HashSet<usize>>())
                .copied()
                .collect::<HashSet<usize>>();
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
