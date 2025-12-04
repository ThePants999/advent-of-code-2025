use advent_of_code_rust_runner::{DayImplementation, Result};
use std::collections::HashSet;

pub struct Day04;

pub struct Day04Context {
    grid: Grid,
    rolls: Vec<Location>
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    row: usize,
    col: usize
}

enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft
}

impl Direction {
    fn iterator() -> std::slice::Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft
        ];
        DIRECTIONS.iter()
    }
}

struct Grid {
    rows: usize,
    cols: usize,
    cells: Vec<Vec<bool>>
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            cells: vec![vec![false; cols]; rows]
        }
    }

    fn add_roll(&mut self, loc: &Location) {
        self.cells[loc.row][loc.col] = true;
    }

    fn remove_roll(&mut self, loc: &Location) {
        self.cells[loc.row][loc.col] = false;
    }

    fn roll_at_loc(&self, loc: &Location) -> bool {
        self.cells[loc.row][loc.col]
    }

    fn roll_in_dir(&self, loc: &Location, dir: &Direction) -> bool {
        let target_loc = match dir {
            Direction::Up => {
                if loc.row == 0 {
                    return false;
                }
                Location { row: loc.row - 1, col: loc.col }
            },
            Direction::UpRight => {
                if loc.row == 0 || loc.col == self.cols - 1 {
                    return false;
                }
                Location { row: loc.row - 1, col: loc.col + 1 }
            },
            Direction::Right => {
                if loc.col == self.cols - 1 {
                    return false;
                }
                Location { row: loc.row, col: loc.col + 1 }
            },
            Direction::DownRight => {
                if loc.row == self.rows - 1 || loc.col == self.cols - 1
                {
                    return false;
                }
                Location { row: loc.row + 1, col: loc.col + 1 }
            },
            Direction::Down => {
                if loc.row == self.rows - 1 {
                    return false;
                }
                Location { row: loc.row + 1, col: loc.col }
            },
            Direction::DownLeft => {
                if loc.row == self.rows - 1 || loc.col == 0 {
                    return false;
                }
                Location { row: loc.row + 1, col: loc.col - 1 }
            },
            Direction::Left => {
                if loc.col == 0 {
                    return false;
                }
                Location { row: loc.row, col: loc.col - 1 }
            },
            Direction::UpLeft => {
                if loc.row == 0 || loc.col == 0 {
                    return false;
                }
                Location { row: loc.row - 1, col: loc.col - 1 }
            }
        };
        self.roll_at_loc(&target_loc)
    }

    fn adjacent_rolls(&self, loc: &Location) -> usize {
        Direction::iterator()
            .filter(|dir| self.roll_in_dir(loc, dir))
            .count()
    }
}

impl DayImplementation for Day04 {
    type Output<'a> = usize;
    type Context<'a> = Day04Context;

    fn day(&self) -> u8 { 4 }
    fn example_input(&self) -> Option<&'static str> { Some("..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.") }
    fn example_part_1_result(&self) -> Option<Self::Output<'static>> { Some(13) }
    fn example_part_2_result(&self) -> Option<Self::Output<'static>> { Some(43) }

    fn execute_part_1<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, Self::Context<'a>)> {
        let lines = input.lines().collect::<Vec<&str>>();
        let rows = lines.len();
        let cols = lines[0].len();
        let mut grid = Grid::new(rows, cols);
        let mut rolls = Vec::new();
        for (r, line) in lines.iter().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                if ch == '@' {
                    let loc = Location { row: r, col: c };
                    grid.add_roll(&loc);
                    rolls.push(loc);
                }
            }
        }

        let answer = rolls
            .iter()
            .map(|loc| grid.adjacent_rolls(loc))
            .filter(|&count| count < 4)
            .count();

        Ok((answer, Day04Context { grid, rolls }))
    }

    fn execute_part_2<'a>(&self, _input: &'a str, ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        let mut grid = ctx.grid;
        let mut rolls = ctx.rolls;
        let mut num_rolls_removed = 0;

        loop {
            let mut rolls_to_remove = HashSet::new();
            for loc in rolls.iter() {
                if grid.adjacent_rolls(loc) < 4 {
                    rolls_to_remove.insert(*loc);
                }
            }
            if rolls_to_remove.is_empty() {
                break;
            }
            num_rolls_removed += rolls_to_remove.len();

            rolls.retain(|loc| !rolls_to_remove.contains(&loc));
            rolls_to_remove.iter().for_each(|loc| grid.remove_roll(loc));
        }
        Ok(num_rolls_removed)
    }
}
