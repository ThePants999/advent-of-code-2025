use advent_of_code_rust_runner::{DayImplementation, Result};
use std::collections::{VecDeque, HashSet, HashMap};

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

struct MatrixRow {
    variables: Vec<i64>,
    rhs: i64
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
        let total_presses: u32 = ctx.machines
            .iter()
            .enumerate()
            .map(|(machine_index, machine)| {
                // Turn the buttons and joltages into a system of linear equations,
                // expressed as a matrix.
                let num_variables = machine.buttons.len();
                let num_equations = machine.joltages.len();
                let mut matrix = machine
                    .joltages
                    .iter()
                    .enumerate()
                    .map(|(joltage_index, &joltage)| {
                        let mut row = MatrixRow {
                            variables: vec![0; num_variables],
                            rhs: joltage as i64
                        };
                        for (var_index, &button) in machine.buttons.iter().enumerate() {
                            if button & (1 << joltage_index) != 0 {
                                row.variables[var_index] = 1;
                            }
                        }
                        row
                    })
                    .collect::<Vec<MatrixRow>>();

                // Solve the system using Gaussian elimination.

                // Step 1: forward elimination.
                let mut pivot_row = 0usize;
                for pivot_variable in 0..num_variables {
                    // Find a row with a non-zero entry in the pivot column.
                    let mut pivot_source = None;
                    for row_index in pivot_row..num_equations {
                        if matrix[row_index].variables[pivot_variable] != 0 {
                            pivot_source = Some(row_index);
                            break;
                        }
                    }
                    if let Some(pivot_source_index) = pivot_source {
                        // Swap the pivot row into position.
                        matrix.swap(pivot_source_index, pivot_row);

                        let pivot_coefficient = matrix[pivot_row].variables[pivot_variable];
                        // Eliminate entries below the pivot.
                        for row_index in (pivot_row + 1)..num_equations {
                            let target_coefficient = matrix[row_index].variables[pivot_variable];
                            if target_coefficient != 0 {
                                let lcm = num::integer::lcm(pivot_coefficient.abs() as u64, target_coefficient.abs() as u64) as i64;
                                let pivot_multiplier = lcm / pivot_coefficient;
                                let target_multiplier = lcm / target_coefficient;
                                for col_index in pivot_variable..num_variables {
                                    matrix[row_index].variables[col_index] =
                                        (pivot_multiplier * matrix[row_index].variables[col_index]) -
                                        (target_multiplier * matrix[pivot_row].variables[col_index]);
                                }
                                matrix[row_index].rhs = (pivot_multiplier * matrix[row_index].rhs) -
                                                         (target_multiplier * matrix[pivot_row].rhs);
                            }
                        }

                        pivot_row += 1;
                    }
                }

                // Step 2: back substitution.
                for row_index in (1..num_equations).rev() {
                    let pivot_col = matrix[row_index]
                        .variables
                        .iter()
                        .position(|&val| val != 0);
                    if let Some(col_index) = pivot_col {
                        let mut pivot_coefficient = matrix[row_index].variables[col_index];
                        if pivot_coefficient < 0 || pivot_coefficient > 1 {
                            let divisor = if matrix[row_index]
                                .variables
                                .iter()
                                .skip(col_index)
                                .all(|&val| val % pivot_coefficient == 0)
                                && matrix[row_index].rhs % pivot_coefficient == 0 {
                                    Some(pivot_coefficient)
                                } else if pivot_coefficient < 0 {
                                    Some(-1)
                                } else {
                                    None
                                };

                            if let Some(divisor) = divisor {
                                for k in col_index..num_variables {
                                    matrix[row_index].variables[k] /= divisor;
                                }
                                matrix[row_index].rhs /= divisor;
                                pivot_coefficient /= divisor;
                            }
                        }

                        for sub_row_index in 0..row_index {
                            let target_coefficient = matrix[sub_row_index].variables[col_index];
                            if target_coefficient != 0 {
                                let lcm = num::integer::lcm(pivot_coefficient.abs() as u64, target_coefficient.abs() as u64) as i64;
                                let pivot_multiplier = lcm / pivot_coefficient;
                                let target_multiplier = lcm / target_coefficient;
                                for k in 0..num_variables {
                                    matrix[sub_row_index].variables[k] =
                                        pivot_multiplier * matrix[sub_row_index].variables[k] -
                                         target_multiplier * matrix[row_index].variables[k];
                                }
                                matrix[sub_row_index].rhs =
                                    pivot_multiplier * matrix[sub_row_index].rhs -
                                     target_multiplier * matrix[row_index].rhs;
                            }
                        }
                    }
                }

                // Step 3: free variables handling (if any).
                let mut base_presses = 0i64;
                let mut free_variables: HashSet<usize> = HashSet::new();
                let mut free_variable_values: HashMap<usize, i64> = HashMap::new();
                for row_index in 0..num_equations {
                    let pivot_col = matrix[row_index]
                        .variables
                        .iter()
                        .position(|&val| val != 0);
                    if let Some(col_index) = pivot_col {
                        base_presses += matrix[row_index].rhs;
                        for other_col_index in col_index + 1..num_variables {
                            let coefficient = matrix[row_index].variables[other_col_index] * -1;
                            if coefficient != 0 {
                                // This variable affects this equation, so it can't be fixed.
                                free_variables.insert(other_col_index);
                                *free_variable_values.entry(other_col_index).or_insert(0) += coefficient;
                            }
                        }
                    } else {
                        assert!(matrix[row_index].rhs == 0, "Inconsistent system of equations");
                    }
                }

                // Account for the fact that, in summing the matrix, we didn't include
                // free variables.
                for (_, val) in free_variable_values.iter_mut() {
                    *val += 1;
                }
                println!("Machine {} (line {})", machine_index, machine_index + 1);
                println!("Base presses: {}", base_presses);
                println!("Free variables: {:?}", free_variables);
                println!("Free variable values: {:?}", free_variable_values);

                assert!(base_presses >= 0, "Negative base presses calculated");
                base_presses as u32
            })
            .sum();
        Ok(total_presses)
    }
}
