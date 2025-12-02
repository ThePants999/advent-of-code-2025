use advent_of_code_rust_runner::{DayImplementation, Result};

pub struct Day02;

pub struct Day02Context {
    ids: Vec<String>
}

impl DayImplementation for Day02 {
    type Output<'a> = u64;
    type Context<'a> = Day02Context;

    fn day(&self) -> u8 { 2 }
    fn example_input(&self) -> Option<&'static str> { Some("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124") }
    fn example_part_1_result(&self) -> Option<Self::Output<'static>> { Some(1227775554) }
    fn example_part_2_result(&self) -> Option<Self::Output<'static>> { Some(4174379265) }

    fn execute_part_1<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, Self::Context<'a>)> {
        let ids: Vec<String> = input
            .trim()
            .split(',')
            .map(|pair| {
                let separator_index = pair.find('-').expect("Invalid input: range missing '-' separator");
                (&pair[..separator_index], &pair[separator_index+1..])
            })
            .map(|(start, end)| (start.parse::<u64>().expect("Invalid input: range start not numeric"), end.parse::<u64>().expect("Invalid input: range end not numeric")))
            .flat_map(|(start, end)| start..=end)
            .map(|num| num.to_string())
            .collect();
        let sum: u64 = ids
            .iter()
            .filter(|id| id.len() % 2 == 0)
            .filter(|id| {
                let (first_half, second_half) = id.split_at(id.len() / 2);
                first_half == second_half
            })
            .map(|id| id.parse::<u64>().unwrap())
            .sum();
        Ok((sum, Day02Context { ids }))
    }

    fn execute_part_2<'a>(&self, _input: &'a str, ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        if ctx.ids.len() < 1000 {
            log::warn!("Invalid IDs: {:?}",
                ctx.ids
                    .iter()
                    .filter(|id| !id_valid_part_2(id))
                    .collect::<Vec<&String>>());
        }
        Ok(ctx.ids
            .iter()
            .filter(|id| !id_valid_part_2(id))
            .map(|id| id.parse::<u64>().unwrap())
            .sum())
    }
}

fn id_valid_part_2(id: &str) -> bool {
    for len in 1..=id.len()/2 {
        if id.len() % len != 0 {
            // ID doesn't divide evenly into segments of this length
            continue;
        }
        let chunk = &id[..len];
        let num_chunks = id.len() / len;
        for chunk_ix in 1..num_chunks {
            let start = chunk_ix * len;
            let end = start + len;
            if &id[start..end] != chunk {
                break;
            }
            if chunk_ix == num_chunks - 1 {
                return false;
            }
        }
    }
    true
}
