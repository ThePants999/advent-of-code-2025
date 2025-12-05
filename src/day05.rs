use advent_of_code_rust_runner::{DayImplementation, Result};

pub struct Day05;

pub struct Day05Context {
    ranges: Vec<(u64, u64)>
}

impl DayImplementation for Day05 {
    type Output<'a> = usize;
    type Context<'a> = Day05Context;

    fn day(&self) -> u8 { 5 }
    fn example_input(&self) -> Option<&'static str> { Some("3-5
10-14
16-20
12-18

1
5
8
11
17
32") }
    fn example_part_1_result(&self) -> Option<Self::Output<'static>> { Some(3) }
    fn example_part_2_result(&self) -> Option<Self::Output<'static>> { Some(14) }

    fn execute_part_1<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, Self::Context<'a>)> {
        let mut ranges = Vec::new();
        let mut lines = input.lines().into_iter();
        loop {
            let line = lines.next().unwrap();
            if line.trim().is_empty() {
                break;
            }
            let mut parts = line.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            ranges.push((start, end));
        }
        let ingredients = lines
            .map(|line| line.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        // Now we rationalise the ranges. Sort, then merge.
        ranges.sort_by(|(start_a, end_a), (start_b, end_b)| {
            if start_a == start_b {
                end_a.cmp(end_b)
            } else {
                start_a.cmp(start_b)
            }
        });

        let mut range_ix = 0usize;
        while range_ix < ranges.len() {
            let (start_a, end_a) = ranges[range_ix];
            let mut merged = false;
            let mut check_ix = range_ix + 1;
            while check_ix < ranges.len() {
                let (start_b, end_b) = ranges[check_ix];
                if (start_a <= end_b && end_a >= start_b) || (start_b <= end_a && end_b >= start_a) {
                    // They overlap, merge them.
                    let new_start = start_a.min(start_b);
                    let new_end = end_a.max(end_b);
                    ranges[range_ix] = (new_start, new_end);
                    ranges.remove(check_ix);
                    merged = true;
                } else {
                    check_ix += 1;
                }
            }
            if !merged {
                range_ix += 1;
            }
        }

        let num_fresh = ingredients.iter()
            .filter(|&&ingredient| ingredient_is_fresh(ingredient, &ranges))
            .count();

        Ok((num_fresh, Day05Context { ranges }))
    }

    fn execute_part_2<'a>(&self, _input: &'a str, ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        let count = ctx.ranges.iter()
            .map(|(start, end)| end - start + 1)
            .sum::<u64>() as usize;
        Ok(count)
    }
}

fn ingredient_is_fresh(ingredient: u64, ranges: &[(u64, u64)]) -> bool {
    for (start, end) in ranges {
        if ingredient >= *start && ingredient <= *end {
            return true;
        }
    }
    false
}
