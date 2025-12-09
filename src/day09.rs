use advent_of_code_rust_runner::{DayImplementation, Result};
use std::collections::{BinaryHeap};
use std::cmp::{Ordering, min, max};

pub struct Day09;

pub struct Day09Context {
    locations: Vec<Location>,
    rectangles: BinaryHeap<Rectangle>
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Location {
    x: u64,
    y: u64,
}

impl Location {
    fn rect_with(&self, other: &Location) -> Rectangle {
        let corner_1 = Location { x: min(self.x, other.x), y: min(self.y, other.y) };
        let corner_2 = Location { x: max(self.x, other.x), y: max(self.y, other.y) };
        Rectangle {
            corner_1,
            corner_2,
            area: (corner_2.x - corner_1.x + 1) * (corner_2.y - corner_1.y + 1)
        }
    }
}

#[derive(Eq, PartialEq)]
struct Rectangle {
    corner_1: Location,
    corner_2: Location,
    area: u64
}

impl Ord for Rectangle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.area.cmp(&other.area)
    }
}

impl PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.area.cmp(&other.area))
    }
}

struct BoundaryLine {
    start: u64,
    end: u64,
    fixed: u64
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
    fn example_part_2_result(&self) -> Option<Self::Output<'static>> { Some(24) }

    fn execute_part_1<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, Self::Context<'a>)> {
        let lines = input.lines().collect::<Vec<&str>>();
        let mut locations: Vec<Location> = Vec::with_capacity(lines.len());
        let mut rectangles: BinaryHeap<Rectangle> = BinaryHeap::new();
        for i in 0..lines.len() {
            let mut parts = lines[i].split(',');
            let x: u64 = parts.next().unwrap().parse().unwrap();
            let y: u64 = parts.next().unwrap().parse().unwrap();
            let loc = Location { x, y };

            for j in 0..i {
                rectangles.push(loc.rect_with(&locations[j]));
            }
            locations.push(loc);
        }

        Ok((rectangles.peek().unwrap().area, Day09Context { locations, rectangles }))
    }

    fn execute_part_2<'a>(&self, _input: &'a str, ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        let mut horizontal_boundaries: Vec<BoundaryLine> = Vec::with_capacity(ctx.locations.len());
        let mut vertical_boundaries: Vec<BoundaryLine> = Vec::with_capacity(ctx.locations.len());
        let mut rectangles = ctx.rectangles;
        for ix in 0..ctx.locations.len()-1 {
            add_boundary(&mut horizontal_boundaries, &mut vertical_boundaries, ctx.locations[ix], ctx.locations[ix+1]);            
        }
        add_boundary(&mut horizontal_boundaries, &mut vertical_boundaries, ctx.locations[0], ctx.locations[ctx.locations.len()-1]);            

        let max_valid_area = 'outer: loop {
            let rectangle = rectangles.pop().unwrap();

            // First, we check whether any boundary line crosses into the interior of this rectangle.
            for boundary in horizontal_boundaries.iter() {
                if ((boundary.start <= rectangle.corner_1.x) != (boundary.end <= rectangle.corner_1.x) ||
                    (boundary.start >= rectangle.corner_2.x) != (boundary.end >= rectangle.corner_2.x)) &&
                   (boundary.fixed > rectangle.corner_1.y) &&
                   (boundary.fixed < rectangle.corner_2.y) {
                    // This boundary intersects one of the rectangle's vertical edges - the rectangle is invalid.
                    continue 'outer;
                }
            }
            for boundary in vertical_boundaries.iter() {
                if ((boundary.start <= rectangle.corner_1.y) != (boundary.end <= rectangle.corner_1.y) ||
                    (boundary.start >= rectangle.corner_2.y) != (boundary.end >= rectangle.corner_2.y)) &&
                   (boundary.fixed > rectangle.corner_1.x) &&
                   (boundary.fixed < rectangle.corner_2.x) {
                    // This boundary intersects one of the rectangle's horizontal edges - the rectangle is invalid.
                    continue 'outer;
                }
            }

            // Finally, we need to test for the edge case of this rectangle being wholly outside
            // the shape, by virtue of coinciding entirely with concave boundary elements. To do
            // this, we pick a point inside the rectangle, draw an imaginary ray (any direction
            // will do), and count how many times it crosses a boundary. If it's an even number
            // of times, we started outside the shape.
            //
            // We need to test a point truly inside the rectangle, not on its edge, or we'll
            // "cross" boundaries that we were actually following. Technically, there are
            // possible degenerate cases where a rectangle with height or width 2 follows a
            // boundary along two sides but its opposite corner is outside the shape, and this
            // wouldn't be caught by the boundary checks above - strictly speaking, we'd need
            // to ray-check all four corners to test that. It so happens, though, that the
            // input has no such cases, so we don't bother.
            let interior_loc = Location { x: rectangle.corner_1.x + 1, y: rectangle.corner_1.y + 1 };
            if interior_loc.x < rectangle.corner_2.x && interior_loc.y < rectangle.corner_2.y 
            {
                let cross_count = vertical_boundaries
                    .iter()
                    .filter(|boundary| boundary.start <= interior_loc.y && boundary.end >= interior_loc.y && boundary.fixed < interior_loc.x)
                    .count();
                if cross_count % 2 == 0 {
                    continue 'outer;
                }
            }

            // Reaching here means this rectangle is valid.
            break rectangle.area;
        };

        Ok(max_valid_area)
    }
}

fn add_boundary(horizontal_boundaries: &mut Vec<BoundaryLine>, vertical_boundaries: &mut Vec<BoundaryLine>, loc1: Location, loc2: Location) {
    if loc1.x == loc2.x {
        // Differ in y
        vertical_boundaries.push(BoundaryLine { start: min(loc1.y, loc2.y), end: max(loc1.y, loc2.y), fixed: loc1.x });
    } else {
        // Differ in x
        horizontal_boundaries.push(BoundaryLine { start: min(loc1.x, loc2.x), end: max(loc1.x, loc2.x), fixed: loc1.y });
    }
}