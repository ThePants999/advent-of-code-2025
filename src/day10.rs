use advent_of_code_rust_runner::{DayImplementation, Result};
use std::collections::{VecDeque, HashSet};

pub struct Day10;

pub struct Day10Context {
    machines: Vec<Machine>
}

struct State {
    lights: u16,
    pressed_buttons: u32
}

struct Machine {
    desired_state: u16,
    buttons: Vec<u16>,
    joltages: Vec<u32>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        let mut desired_state = 0;
        let lights_bytes = parts[0].as_bytes();
        for i in 1..lights_bytes.len()-1 {
            if lights_bytes[i] == b'#' {
                desired_state |= 1 << (i - 1);
            }
        }

        let buttons = parts[1..parts.len() - 1]
            .iter()
            .map(|btn_str| {
                let mut button: u16 = 0;
                let btn_bytes = btn_str.as_bytes();
                btn_bytes.iter().skip(1).step_by(2).for_each(|&b| {
                    let index = b.wrapping_sub(b'0') as usize;
                    button |= 1 << index;
                });
                button
            })
            .collect();

        let joltages = parts[parts.len() - 1]
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|j| j.parse::<u32>().unwrap())
            .collect();

        Machine {
            desired_state,
            buttons,
            joltages
        }
    }
}

impl DayImplementation for Day10 {
    type Output<'a> = u32;
    type Context<'a> = Day10Context;

    fn day(&self) -> u8 { 10 }
    fn example_input(&self) -> Option<&'static str> { Some("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}") }
    fn example_part_1_result(&self) -> Option<Self::Output<'static>> { Some(7) }
    fn example_part_2_result(&self) -> Option<Self::Output<'static>> { Some(0) }

    fn execute_part_1<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, Self::Context<'a>)> {
        let machines: Vec<Machine> = input.lines().map(Machine::parse).collect();

        let num_presses = machines
            .iter()
            .map(|machine| {
                let mut states: VecDeque<State> = VecDeque::new();
                states.push_back(State {
                    lights: 0,
                    pressed_buttons: 0
                });
                let mut seen_states: HashSet<u16> = HashSet::new();
                seen_states.insert(0);
                loop {
                    let state = states.pop_front().unwrap();
                    if state.lights == machine.desired_state {
                        return state.pressed_buttons;
                    }

                    for button in &machine.buttons {
                        let new_lights = state.lights ^ button;

                        // Check whether this button actually improves anything.
                        let wrong_lights = state.lights ^ machine.desired_state;
                        let state_better = (button & wrong_lights) != 0;

                        if state_better && seen_states.insert(new_lights) {
                            states.push_back(State {
                                lights: new_lights,
                                pressed_buttons: state.pressed_buttons + 1
                            });
                        }
                    }
                }
            })
            .sum();

        Ok((num_presses, Day10Context { machines }))
    }

    fn execute_part_2<'a>(&self, _input: &'a str, ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        Ok(0)
    }
}
