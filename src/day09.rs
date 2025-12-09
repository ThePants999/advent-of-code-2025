use advent_of_code_rust_runner::{DayImplementation, Result};

pub struct Day09;

pub struct Day09Context {
    locations: Vec<Location>,
    max_x: u64,
    max_y: u64,
}

#[derive(Clone, Copy)]
struct Location {
    x: u64,
    y: u64,
}

impl Location {
    fn rect_with(&self, other: &Location) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

#[derive(Clone, Copy)]
enum Corners {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Copy)]
struct Part2Location {
    x: u64,
    y: u64,
    corner: Corners,
    x_limit: Option<u64>,
    y_limit: Option<u64>,
}

impl DayImplementation for Day09 {
    type Output<'a> = u64;
    type Context<'a> = Day09Context;

    fn day(&self) -> u8 { 9 }
    fn example_input(&self) -> Option<&'static str> { Some("7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3") }
    fn example_part_1_result(&self) -> Option<Self::Output<'static>> { Some(50) }
    fn example_part_2_result(&self) -> Option<Self::Output<'static>> { Some(0) }

    fn execute_part_1<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, Self::Context<'a>)> {
        let lines = input.lines().collect::<Vec<&str>>();
        let mut locations: Vec<Location> = Vec::with_capacity(lines.len());
        let mut min_x = u64::MAX;
        let mut max_x = u64::MIN;
        let mut min_y = u64::MAX;
        let mut max_y = u64::MIN;
        for line in lines {
            let mut parts = line.split(',');
            let x: u64 = parts.next().unwrap().parse().unwrap();
            let y: u64 = parts.next().unwrap().parse().unwrap();
            if x < min_x { min_x = x; }
            if x > max_x { max_x = x; }
            if y < min_y { min_y = y; }
            if y > max_y { max_y = y; }
            locations.push(Location { x, y });
        }

        let x_offset = min_x - 1;
        let y_offset = min_y - 1;
        for loc in &mut locations {
            loc.x -= x_offset;
            loc.y -= y_offset;
        }
        max_x -= x_offset - 1;
        max_y -= y_offset - 1;

        let max_rect = locations
            .iter()
            .map(|loc1| {
                locations.iter().map(|loc2| loc1.rect_with(loc2)).max().unwrap()
            })
            .max()
            .unwrap();

        Ok((max_rect, Day09Context { locations, max_x, max_y }))
    }

    fn execute_part_2<'a>(&self, _input: &'a str, ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        let mut locations = ctx.locations;
        locations.insert(0, locations[locations.len() - 1]);
        locations.push(locations[0]);
        let mut p2locs: Vec<Part2Location> =
            std::iter::once(Part2Location { x: 0, y: 0, corner: Corners::TopLeft, x_limit: None, y_limit: None })
            .chain((1..locations.len()-2)
                .map(|ix| {
                    let loc = locations[ix];
                    let prev_loc = locations[ix-1];
                    let next_loc = locations[ix+1];
                    let corner = if (prev_loc.x < loc.x && loc.y < next_loc.y) || (next_loc.x < loc.x && loc.y < prev_loc.y) {
                        Corners::TopRight
                    } else if (prev_loc.x > loc.x && loc.y < next_loc.y) || (next_loc.x > loc.x && loc.y < prev_loc.y) {
                        Corners::TopLeft
                    } else if (prev_loc.x < loc.x && loc.y > next_loc.y) || (next_loc.x < loc.x && loc.y > prev_loc.y) {
                        Corners::BottomRight
                    } else if (prev_loc.x > loc.x && loc.y > next_loc.y) || (next_loc.x > loc.x && loc.y > prev_loc.y) {
                        Corners::BottomLeft
                    } else {
                        panic!("Nope, not a corner");
                    };
                    Part2Location {
                        x: loc.x,
                        y: loc.y,
                        corner,
                        x_limit: None,
                        y_limit: None,
                    }
                }))
            .chain(std::iter::once(Part2Location { x: 0, y: 0, corner: Corners::TopLeft, x_limit: None, y_limit: None }))
            .collect();

        Ok(0)
    }
}
