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

enum ConstraintRHS {
    IsPositive,
    DivisibleBy(i16)
}

struct Constraint<'a> {
    coefficients: &'a [i16],
    rhs: ConstraintRHS
}

impl<'a> Constraint<'a> {
    fn is_satisfied(&self, variable_values: &[i16]) -> bool {
        let lhs: i16 = self.coefficients
            .iter()
            .zip(variable_values.iter())
            .map(|(&coeff, &val)| coeff * val)
            .sum::<i16>();
        match self.rhs {
            ConstraintRHS::IsPositive => lhs >= 0,
            ConstraintRHS::DivisibleBy(divisor) => lhs % divisor == 0
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
    fn example_part_2_result(&self) -> Option<Self::Output<'static>> { Some(33) }

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
            .map(|(_machine_index, machine)| {
                // Turn the buttons and joltages into a system of linear equations,
                // expressed as a matrix.
                let num_variables = machine.buttons.len();
                let num_equations = machine.joltages.len();
                let mut matrix = machine
                    .joltages
                    .iter()
                    .enumerate()
                    .map(|(joltage_index, &joltage)| {
                        let mut variables = vec![0; num_variables + 1];
                        for (var_index, &button) in machine.buttons.iter().enumerate() {
                            if button & (1 << joltage_index) != 0 {
                                variables[var_index] = 1;
                            }
                        }
                        variables[num_variables] = joltage as i16;
                        variables
                    })
                    .collect::<Vec<Vec<i16>>>();

                // Solve the system using Gaussian elimination.

                // Step 1: forward elimination.
                let mut pivot_row = 0usize;
                for pivot_variable in 0..num_variables {
                    // Find a row with a non-zero entry in the pivot column.
                    let mut pivot_source = None;
                    for row_index in pivot_row..num_equations {
                        if matrix[row_index][pivot_variable] != 0 {
                            pivot_source = Some(row_index);
                            break;
                        }
                    }
                    if let Some(pivot_source_index) = pivot_source {
                        // Swap the pivot row into position.
                        matrix.swap(pivot_source_index, pivot_row);

                        // Eliminate entries below the pivot.
                        let pivot_coefficient = matrix[pivot_row][pivot_variable];
                        for row_index in (pivot_row + 1)..num_equations {
                            let target_coefficient = matrix[row_index][pivot_variable];
                            if target_coefficient != 0 {
                                // Maintain integer arithmetic by using LCM to find multipliers.
                                let lcm = num::integer::lcm(pivot_coefficient.abs() as u64, target_coefficient.abs() as u64) as i16;
                                let pivot_multiplier = lcm / pivot_coefficient;
                                let target_multiplier = lcm / target_coefficient;
                                for col_index in pivot_variable..num_variables+1 {
                                    matrix[row_index][col_index] =
                                        (target_multiplier * matrix[row_index][col_index]) -
                                        (pivot_multiplier * matrix[pivot_row][col_index]);
                                }
                            }
                        }

                        pivot_row += 1;
                    }
                }

                // Step 2: back substitution.
                let mut pivot_cols: HashMap<usize, usize> = HashMap::new();
                pivot_cols.insert(0, 0);
                for row_index in (1..num_equations).rev() {
                    let pivot_col = matrix[row_index]
                        .iter()
                        .position(|&val| val != 0);
                    if let Some(col_index) = pivot_col {
                        // Normalize the pivot row by dividing by the GCD of all its entries,
                        // and ensuring the pivot column is positive.
                        pivot_cols.insert(row_index, col_index);
                        let mut gcd = matrix[row_index]
                            .iter()
                            .skip(col_index)
                            .fold(0, |acc, &val| num::integer::gcd(acc, val));
                        if matrix[row_index][col_index] < 0 {
                            gcd *= -1;
                        }

                        if gcd != 1 {
                            assert!(gcd != 0, "GCD should not be zero");
                            for k in col_index..num_variables+1 {
                                matrix[row_index][k] /= gcd;
                            }
                        }

                        // Eliminate the pivot column from entries above the pivot.
                        let pivot_coefficient = matrix[row_index][col_index];
                        for sub_row_index in 0..row_index {
                            let target_coefficient = matrix[sub_row_index][col_index];
                            if target_coefficient != 0 {
                                // Maintain integer arithmetic by using LCM to find multipliers.
                                let lcm = num::integer::lcm(pivot_coefficient.abs() as u64, target_coefficient.abs() as u64) as i16;
                                let pivot_multiplier = lcm / pivot_coefficient;
                                let target_multiplier = lcm / target_coefficient;
                                for k in 0..num_variables+1 {
                                    matrix[sub_row_index][k] =
                                        (target_multiplier * matrix[sub_row_index][k]) -
                                        (pivot_multiplier * matrix[row_index][k]);
                                }
                            }
                        }
                    }
                }

                // Step 3: scale the rows to the LCM of the pivot coefficients, pull
                // out the RHS values and free variable coefficients, and flip the
                // free variable coefficient signs to reflect moving them to the RHS
                // of the equations.
                let lcm = pivot_cols
                    .iter()
                    .map(|(&row_index, &col_index)| matrix[row_index][col_index])
                    .fold(1, |acc, val| num::integer::lcm(acc, val));

                let mut free_variables: HashSet<usize> = HashSet::new();
                // We initialize the free variable coefficients to the LCM, reflecting
                // the fact that the matrix is going to effectively end up giving us
                // the sum of the fixed variables multiplied by the LCM, so we need
                // to add the missing free variables.
                let mut total_coefficients = vec![lcm; num_variables+1];
                total_coefficients[num_variables] = 0; // Base presses
                for (&row_index, &pivot_col) in pivot_cols.iter() {
                    let scale = lcm / matrix[row_index][pivot_col];
                    if scale != 1 {
                        for col_index in pivot_col..num_variables+1 {
                            matrix[row_index][col_index] *= scale;
                        }
                    }
                    total_coefficients[num_variables] += matrix[row_index][num_variables];
                    for other_col_index in pivot_col + 1..num_variables {
                        let coefficient = matrix[row_index][other_col_index];
                        if coefficient != 0 {
                            // This variable affects this equation, so it can't be fixed.
                            free_variables.insert(other_col_index);
                            // Flip the sign of the coefficient, as we're moving it
                            // from the LHS of the equation to the RHS.
                            matrix[row_index][other_col_index] = -coefficient;
                            // Pull it out.
                            total_coefficients[other_col_index] -= coefficient;
                        }
                    }
                }
                let free_variables: Vec<usize> = free_variables.into_iter().collect();

                // Step 4: establish the constraints on the free variables.
                let constraints: Vec<Constraint> = pivot_cols
                    .keys()
                    .flat_map(|&row_index| {
                        std::iter::once(Constraint {
                            coefficients: &matrix[row_index],
                            rhs: ConstraintRHS::IsPositive
                        })
                        .chain((lcm > 1).then(|| {
                            Constraint {
                                coefficients: &matrix[row_index],
                                rhs: ConstraintRHS::DivisibleBy(lcm)
                            }
                        }))
                    .chain(std::iter::once(Constraint {
                        coefficients: &total_coefficients,
                        rhs: ConstraintRHS::DivisibleBy(lcm)
                    }))})
                    .collect();

                // Step 5: determine boundaries on the free variables from the IsPositive constraints.
                let bounds = calculate_bounds(
                    num_variables,
                    &constraints,
                    &free_variables
                );

                // Step 6: find the minimal non-negative assignment to the free variables
                // that satisfies all constraints.
                let total = find_optimal_total(
                    &constraints,
                    &total_coefficients,
                    lcm,
                    &free_variables,
                    &bounds
                ) as u32;
                println!("Total presses for machine {}: {}", _machine_index, total);
                total
            })
            .sum();
        Ok(total_presses)
    }
}

fn calculate_bounds(
    num_variables: usize,
    constraints: &[Constraint],
    free_variables: &[usize],
) -> Vec<(i16, i16)> {
    // Start with widest possible bounds
    let mut bounds = vec![(0i16, i16::MAX); num_variables];

    // Tighten bounds from IsPositive constraints
    for constraint in constraints {
        if let ConstraintRHS::IsPositive = constraint.rhs {
            let constant_term = *constraint.coefficients.last().unwrap();

            // For each free variable, try to extract a bound
            for &var_idx in free_variables.iter() {
                let coeff = constraint.coefficients[var_idx];

                if coeff == 0 {
                    continue;
                }

                // Constraint: constant + sum(coeff_i * x_i) >= 0
                // For this variable: coeff * x_var >= -(constant + sum_others)

                // Calculate sum of contributions from other free vars at their extremes
                let mut min_others = constant_term;
                let mut max_others = constant_term;

                for &other_var_idx in free_variables.iter() {
                    if other_var_idx == var_idx {
                        continue;
                    }
                    let other_coeff = constraint.coefficients[other_var_idx];
                    let (other_min, other_max) = bounds[other_var_idx];

                    if other_coeff > 0 {
                        min_others = min_others.saturating_add(other_coeff.saturating_mul(other_min));
                        max_others = max_others.saturating_add(other_coeff.saturating_mul(other_max));
                    } else if other_coeff < 0 {
                        min_others = min_others.saturating_add(other_coeff.saturating_mul(other_max));
                        max_others = max_others.saturating_add(other_coeff.saturating_mul(other_min));
                    }
                }

                // Now isolate this variable
                if coeff > 0 {
                    // x_var >= -max_others / coeff
                    if let Some(numerator) = max_others.checked_neg() {
                        let lower = ((numerator + coeff - 1) / coeff).max(0);
                        bounds[var_idx].0 = bounds[var_idx].0.max(lower);
                    }
                    // If overflow, this constraint doesn't provide a useful bound
                } else {
                    // x_var <= -min_others / coeff
                    if let Some(numerator) = min_others.checked_neg() {
                        let upper = numerator / coeff;
                        bounds[var_idx].1 = bounds[var_idx].1.min(upper);
                    }
                }
            }
        }
    }

    bounds
}

fn find_optimal_total(
    constraints: &[Constraint],
    coefficients: &[i16],
    lcm: i16,
    free_variables: &[usize],
    bounds: &[(i16, i16)]
) -> i16 {

    let mut best_total = i16::MAX;

    fn search(
        free_var_idx: usize,
        free_variable_values: &mut Vec<i16>,
        constraints: &[Constraint],
        coefficients: &[i16],
        lcm: i16,
        free_variables: &[usize],
        bounds: &[(i16, i16)],
        best_total: &mut i16,
    ) {
        // Base case: all free variables assigned
        if free_var_idx == free_variables.len() {
            // Check constraints
            if !constraints.iter().all(|c| c.is_satisfied(free_variable_values)) {
                return;
            }

            // Calculate total
            let total: i16 = coefficients
                .iter()
                .zip(free_variable_values.iter())
                .map(|(c, v)| c * v)
                .sum::<i16>()
                / lcm;

            if total < *best_total {
                *best_total = total;
            }
            return;
        }

        // Get the actual variable index and its contribution
        let var_idx = free_variables[free_var_idx];
        let (min_val, max_val) = bounds[var_idx];

        if coefficients[var_idx] < 0 {
            // Loop backwards for negative contribution
            for val in (min_val..=max_val).rev() {
                free_variable_values[var_idx] = val;
                search(
                    free_var_idx + 1,
                    free_variable_values,
                    constraints,
                    coefficients,
                    lcm,
                    free_variables,
                    bounds,
                    best_total,
                );
            }
        } else {
            // Loop forwards for positive or zero contribution
            for val in min_val..=max_val {
                free_variable_values[var_idx] = val;
                search(
                    free_var_idx + 1,
                    free_variable_values,
                    constraints,
                    coefficients,
                    lcm,
                    free_variables,
                    bounds,
                    best_total,
                );
            }
        }
    }

    let mut free_variable_values = vec![0; coefficients.len()];
    free_variable_values[coefficients.len() - 1] = 1;  // Set RHS position to 1

    search(
        0,
        &mut free_variable_values,
        constraints,
        coefficients,
        lcm,
        free_variables,
        bounds,
        &mut best_total,
    );

    best_total
}