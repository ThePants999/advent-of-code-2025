use advent_of_code_rust_runner::{DayImplementation, Result, Context};

pub struct Day01;

pub struct Day01Context {
    turns: Vec<i32>
}

impl DayImplementation for Day01 {
    type Output<'a> = u32;
    type Context<'a> = Day01Context;

    fn day(&self) -> u8 { 1 }
    fn example_input(&self) -> Option<&'static str> { Some("L68
L30
R48
L5
R60
L55
L1
L99
R14
L82") }
    fn example_part_1_result(&self) -> Option<Self::Output<'static>> { Some(3) }
    fn example_part_2_result(&self) -> Option<Self::Output<'static>> { Some(6) }

    fn execute_part_1<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, Self::Context<'a>)> {
        let turns: Vec<i32> = input.lines()
            .map(|line| -> Result<i32> {
                if line.is_empty() {
                    anyhow::bail!("Invalid input: empty line");
                }
                let distance: i32 = line[1..].parse().context("Invalid input: distance not a number")?;
                Ok(match line.as_bytes()[0] {
                    b'L' => -distance,
                    b'R' => distance,
                    _ => anyhow::bail!("Invalid input: first letter not L or R"),
                })
            })
            .collect::<Result<Vec<i32>>>()?;

        let mut password: u32 = 0;
        let mut dial: i32 = 50;
        for turn in turns.iter() {
            dial += turn;
            if dial % 100 == 0 {
                password += 1;
            }
        }
        Ok((password, Day01Context { turns }))
    }

    fn execute_part_2<'a>(&self, _input: &'a str, context: Self::Context<'a>) -> Result<Self::Output<'a>> {
        let mut password: u32 = 0;
        let mut dial: i32 = 50;
        for turn in context.turns {
            let new_dial = dial + turn;
            log::debug!("Turn: {}, Dial: {} -> {}", turn, dial, new_dial);
            let mut password_delta = (new_dial / 100 - dial / 100).unsigned_abs();
            log::debug!("Password increase from hundreds change: {}", password_delta);
            if (dial < 0 && new_dial >= 0) || (dial > 0 && new_dial <= 0) {
                log::debug!("Crossed zero line, adding extra 1 to password");
                password_delta += 1;
            }
            password += password_delta;
            dial = new_dial % 100;
        }
        Ok(password)
    }
}