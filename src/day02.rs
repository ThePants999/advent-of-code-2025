use advent_of_code_rust_runner::{DayImplementation, Result};

use itertools::{Itertools, Either};

pub struct Day02;

pub struct Day02Context {
    ranges: Vec<(u64,u64,String,String)>
}

impl DayImplementation for Day02 {
    type Output<'a> = u64;
    type Context<'a> = Day02Context;

    fn day(&self) -> u8 { 2 }
    fn example_input(&self) -> Option<&'static str> { Some("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124") }
    fn example_part_1_result(&self) -> Option<Self::Output<'static>> { Some(1227775554) }
    fn example_part_2_result(&self) -> Option<Self::Output<'static>> { Some(4174379265) }

    fn execute_part_1<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, Self::Context<'a>)> {
        let ranges: Vec<(u64,u64,String,String)> = input
            .trim()
            .split(',')
            .map(|pair| {
                let separator_index = pair.find('-').expect("Invalid input: range missing '-' separator");
                (&pair[..separator_index], &pair[separator_index+1..])
            })
            .flat_map(|(start, end)| {
                let (start_num, end_num) = (start.parse::<u64>().expect("Invalid input: range start not numeric"), end.parse::<u64>().expect("Invalid input: range end not numeric"));
                if start.len() == end.len() {
                    // Normal case
                    [Some((start_num, end_num, start.to_string(), end.to_string())), None]
                } else {
                    // Range spans different digit lengths; we need to split
                    // into two ranges
                    let split_point = 10u64.pow(start.len() as u32);
                    [Some((start_num, split_point - 1, start.to_string(), (split_point - 1).to_string())),
                     Some((split_point, end_num, split_point.to_string(), end.to_string()))]
                }
            })
            .flatten()
            .collect();

        Ok((calculate_sum(&ranges, false), Day02Context { ranges }))
    }

    fn execute_part_2<'a>(&self, _input: &'a str, ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        Ok(calculate_sum(&ctx.ranges, true))
    }
}

fn calculate_sum(ranges: &[(u64,u64,String,String)], part_2: bool) -> u64 {
    ranges
        .iter()
        // Figure out, for each range, the valid set of repeatable pattern sublengths.
        .flat_map(|(start, end, start_str, end_str)| lengths_to_check(start_str.len(), part_2).map(move |len| (start, end, start_str, end_str, len)))
        // Now figure out the actual patterns of each length.
        .flat_map(|(start, end, start_str, end_str, len)| {
            let first_val = start_str[..len].parse::<u64>().unwrap();
            let last_val = end_str[..len].parse::<u64>().unwrap();
            let repetitions = start_str.len() / len;
            (first_val..=last_val).map(move |prefix| (start, end, repetitions, prefix.to_string()))})
        // Generate the full IDs from the patterns.
        .map(|(start, end, repetitions, prefix)| (start, end, prefix.repeat(repetitions)))
        .map(|(start, end, id_str)| (start, end, id_str.parse::<u64>().unwrap()))
        // Filter to only those IDs which are actually in the range.
        .filter_map(|(start, end, id)| if id >= *start && id <= *end { Some(id) } else { None })
        // Remove duplicates, as (for example) length 6 could generate identical patterns of
        // lengths 1, 2 and 3.
        .unique()
        .sum()
}

fn lengths_to_check(id_len: usize, part_2: bool) -> impl Iterator<Item = usize> {
    if !part_2 {
        // Part 1 - only length to check is half the ID length, and only if even
        if id_len.is_multiple_of(2) {
            // We use itertools::Either to allow us to return different concrete
            // iterator types.
            Either::Left(Either::Left(std::iter::once(id_len / 2)))
        } else {
            Either::Left(Either::Right(std::iter::empty()))
        }
    } else {
        // Part 2 - check all factors
        Either::Right((1..=id_len / 2).filter(move |len| id_len.is_multiple_of(*len)))
    }
}