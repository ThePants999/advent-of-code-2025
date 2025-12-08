use advent_of_code_rust_runner::{DayImplementation, Result};

pub struct Day08;

pub struct Day08Context {
    graph: Graph
}

struct Graph {
    boxes: Vec<JunctionBox>,
    circuits: Vec<Option<Vec<usize>>>,
    num_circuits: usize,
    pairs: Vec<BoxPair>
}

impl Graph {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let num_lines = lines.len();
        let mut graph = Graph {
            boxes: Vec::with_capacity(num_lines),
            circuits: Vec::with_capacity(num_lines),
            num_circuits: num_lines,
            pairs: Vec::with_capacity(num_lines * (num_lines - 1) / 2)
        };

        for (id, line) in lines.iter().enumerate() {
            let mut coords = line.split(',');
            let new_box = JunctionBox {
                x: coords.next().unwrap().parse().unwrap(),
                y: coords.next().unwrap().parse().unwrap(),
                z: coords.next().unwrap().parse().unwrap(),
                circuit: id
            };
            graph.boxes.push(new_box);
            graph.circuits.push(Some(vec![id]));

            for other_id in 0..id {
                let pair = graph.boxes[id].pair(id, &graph.boxes[other_id], other_id);
                graph.pairs.push(pair);
            }
        }
        graph.pairs.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

        graph
    }

    fn connect(&mut self, pair: &BoxPair) {
        let circuit_a_id = self.boxes[pair.box_a].circuit;
        let circuit_b_id = self.boxes[pair.box_b].circuit;

        if circuit_a_id == circuit_b_id {
            // Already connected
            return;
        }

        let mut big_circuit: Vec<usize>;
        let small_circuit: &Vec<usize>;
        let big_circuit_id: usize;
        let small_circuit_id: usize;
        if self.circuits[circuit_a_id].as_ref().unwrap().len() >= self.circuits[circuit_b_id].as_ref().unwrap().len() {
            big_circuit = self.circuits[circuit_a_id].take().unwrap();
            big_circuit_id = circuit_a_id;
            small_circuit = self.circuits[circuit_b_id].as_ref().unwrap();
            small_circuit_id = circuit_b_id;
        } else {
            big_circuit = self.circuits[circuit_b_id].take().unwrap();
            big_circuit_id = circuit_b_id;
            small_circuit = self.circuits[circuit_a_id].as_ref().unwrap();
            small_circuit_id = circuit_a_id;
        }

        for box_id in small_circuit {
            self.boxes[*box_id].circuit = big_circuit_id;
            big_circuit.push(*box_id);
        }

        self.circuits[big_circuit_id] = Some(big_circuit);
        self.circuits[small_circuit_id] = None;
        self.num_circuits -= 1;
    }
}

#[derive(Clone, Copy)]
struct BoxPair {
    box_a: usize,
    box_b: usize,
    distance: f64
}

struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
    circuit: usize
}

impl JunctionBox {
    fn pair(&self, self_id: usize, other: &JunctionBox, other_id: usize) -> BoxPair {
        let square_sum = ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64;
        let distance = square_sum.sqrt();
        BoxPair {
            box_a: self_id,
            box_b: other_id,
            distance
        }
    }
}

impl DayImplementation for Day08 {
    type Output<'a> = usize;
    type Context<'a> = Day08Context;

    fn day(&self) -> u8 { 8 }
    fn example_input(&self) -> Option<&'static str> { Some("162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689") }
    fn example_part_1_result(&self) -> Option<Self::Output<'static>> { Some(40) }
    fn example_part_2_result(&self) -> Option<Self::Output<'static>> { Some(25272) }

    fn execute_part_1<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, Self::Context<'a>)> {
        let mut graph = Graph::parse(input);
        let num_connections = if graph.boxes.len() < 30 { 10 } else { 1000 };
        for i in 0..num_connections {
            let pair = graph.pairs[i];
            graph.connect(&pair);
        }
        let mut circuits: Vec<usize> = graph.circuits.clone().iter().flatten().map(|circuit| circuit.len()).collect();
        circuits.sort_by(|a, b| b.cmp(a));
        let answer = circuits.iter().take(3).product();
        Ok((answer, Day08Context { graph }))
    }

    fn execute_part_2<'a>(&self, _input: &'a str, ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        let mut graph = ctx.graph;
        let mut connection_ix = if graph.boxes.len() < 30 { 10 } else { 1000 };
        let answer = loop {
            let pair = graph.pairs[connection_ix];
            graph.connect(&pair);
            if graph.num_circuits == 1 {
                break graph.boxes[pair.box_a].x as usize * graph.boxes[pair.box_b].x as usize
            }
            connection_ix += 1;
        };
        Ok(answer)
    }
}
