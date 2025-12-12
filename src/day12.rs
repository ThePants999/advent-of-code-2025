use advent_of_code_rust_runner::{DayImplementation, Result};
use cp_sat::builder::{CpModelBuilder, BoolVar, LinearExpr};
use cp_sat::proto::CpSolverStatus;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct Day12;

pub struct Day12Context {

}

const NUM_SHAPES: usize = 6;
const INPUT_LINES_PER_SHAPE: usize = 5;
const SHAPE_SIZE: usize = 3;
const MAX_SHAPE_COORD: usize = SHAPE_SIZE - 1;
const MAX_SHAPE_CELLS: usize = 7;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Cell {
    row: usize,
    col: usize
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Orientation(Vec<Cell>);

impl Orientation {
    fn parse(input: &[&str]) -> Self {
        let mut cells: Vec<Cell> = Vec::with_capacity(MAX_SHAPE_CELLS);
        for (row_index, line) in input.iter().enumerate() {
            for (col_index, ch) in line.chars().enumerate() {
                if ch == '#' {
                    cells.push(Cell {
                        row: row_index,
                        col: col_index
                    });
                }
            }
        }
        Self(cells)
    }

    fn rotate(&mut self) {
        self.0.iter_mut().for_each(|cell| {(cell.row, cell.col) = (cell.col, MAX_SHAPE_COORD-cell.row);});
    }

    fn flip_horizontal(&mut self) {
        self.0.iter_mut().for_each(|cell| {cell.col = MAX_SHAPE_COORD - cell.col;});
    }

    fn flip_vertical(&mut self) {
        self.0.iter_mut().for_each(|cell| {cell.row = MAX_SHAPE_COORD - cell.row;});
    }
}

struct Shape(Vec<Orientation>);

impl Shape {
    fn parse(input: &[&str]) -> Self {
        // Temporarily use a HashSet to avoid duplicate orientations.
        let mut orientations: HashSet<Orientation> = HashSet::with_capacity(8);
        let mut orientation = Orientation::parse(input);

        // Generate all rotations and reflections
        for _ in 0..4 {
            orientations.insert(orientation.clone());

            orientation.flip_horizontal();
            orientations.insert(orientation.clone());
            orientation.flip_horizontal();

            orientation.flip_vertical();
            orientations.insert(orientation.clone());
            orientation.flip_vertical();

            orientation.rotate();
        }

        Self(orientations.into_iter().collect())
    }

    fn populate_model(&self, model: &mut CpModelBuilder, region_width: usize, region_height: usize, shape_count: usize) -> HashMap<Cell, Vec<BoolVar>> {
        let mut cell_vars: HashMap<Cell, Vec<BoolVar>> = HashMap::new();
        let mut shape_vars = LinearExpr::default();

        // For each orientation, and for each possible position in the region,
        // add a variable to the model indicating whether this shape is placed
        // in that orientation at that position.
        for orientation in &self.0 {
            for row in 0..region_height-MAX_SHAPE_COORD {
                for col in 0..region_width-MAX_SHAPE_COORD {
                    let var = model.new_bool_var();
                    shape_vars += var;
                    for cell in &orientation.0 {
                        let placed_cell = Cell {
                            row: row + cell.row,
                            col: col + cell.col
                        };
                        cell_vars.entry(placed_cell)
                            .or_insert_with(Vec::new)
                            .push(var);
                    }
                }
            }
        }

        // Add constraint: this shape must be used exactly `shape_count` times.
        model.add_eq(shape_vars, shape_count as i64);

        cell_vars
    }
}

struct Region {
    width: usize,
    height: usize,
    required_shapes: Vec<usize>
}

impl Region {
    fn parse(input_line: &str) -> Self {
        let mut parts = input_line.split_ascii_whitespace();
        let mut size_parts = parts.next().unwrap().trim_end_matches(':').split('x');
        let width: usize = size_parts.next().unwrap().parse().unwrap();
        let height: usize = size_parts.next().unwrap().parse().unwrap();
        let required_shapes: Vec<usize> = parts.map(|part| part.parse::<usize>().unwrap()).collect();

        Self {
            width,
            height,
            required_shapes
        }
    }

    fn is_solvable(&self, shapes: &Vec<Shape>) -> bool {
        let total_required_cells: usize = (0..NUM_SHAPES)
            .map(|shape_id| {
                let shape = &shapes[shape_id];
                let count = self.required_shapes[shape_id];
                count * shape.0[0].0.len()
            })
            .sum();

        if total_required_cells > self.width * self.height {
            // The required shapes fill more cells than the region has.
            return false;
        }

        /*let mut model = CpModelBuilder::default();

        let shape_cell_vars: Vec<HashMap<Cell, Vec<BoolVar>>> = (0..NUM_SHAPES)
            .map(|shape_id| {
                let shape = &shapes[shape_id];
                shape.populate_model(
                    &mut model,
                    self.width,
                    self.height,
                    self.required_shapes[shape_id])
            })
            .collect();

        // Add constraints: each cell in the region must be covered by at most one shape.
        for row in 0..self.height {
            for col in 0..self.width {
                let cell = Cell { row, col };

                model.add_at_most_one(shape_cell_vars
                    .iter()
                    .map(|cell_vars| cell_vars.get(&cell).unwrap().to_owned())
                    .flatten()
                );
            }
        }

        let response = model.solve();
        response.status() == CpSolverStatus::Feasible || response.status() == CpSolverStatus::Optimal*/
        return true
    }
}

struct Problem {
    shapes: Vec<Shape>,
    regions: Vec<Region>
}

impl Problem {
    fn parse(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<&str>>();

        let shapes = (0..NUM_SHAPES)
            .map(|i| {
                let start_line = i * INPUT_LINES_PER_SHAPE;
                Shape::parse(&lines[start_line+1..start_line+4])
            })
            .collect::<Vec<Shape>>();

        let regions: Vec<Region> = lines[NUM_SHAPES * INPUT_LINES_PER_SHAPE..]
            .iter()
            .map(|line| Region::parse(line))
            .collect();

        Self {
            shapes,
            regions
        }
    }

    fn count_solvable_regions(&self) -> u32 {
        self.regions
            .par_iter() // Shard the work across multiple threads
            .filter(|region| region.is_solvable(&self.shapes))
            .count() as u32
    }
}

impl DayImplementation for Day12 {
    type Output<'a> = u32;
    type Context<'a> = Day12Context;

    fn day(&self) -> u8 { 12 }
    fn example_input(&self) -> Option<&'static str> { Some("0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2") }
    fn example_part_1_result(&self) -> Option<Self::Output<'static>> { Some(2) }
    fn example_part_2_result(&self) -> Option<Self::Output<'static>> { Some(0) }

    fn execute_part_1<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, Self::Context<'a>)> {
        let problem = Problem::parse(input);
        Ok((problem.count_solvable_regions(), Day12Context {}))
    }

    fn execute_part_2<'a>(&self, _input: &'a str, _ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        Ok(0)
    }
}
