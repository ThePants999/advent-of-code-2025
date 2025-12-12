use advent_of_code_rust_runner::{DayImplementation, Result};
use std::collections::{HashMap, VecDeque};

pub struct Day11;

pub struct Day11Context {
    devices: Vec<Device>,
    ordering: Vec<usize>,
}

const OUT_DEVICE_INDEX: usize = 0;
const YOU_DEVICE_INDEX: usize = 1;
const SVR_DEVICE_INDEX: usize = 2;
const DAC_DEVICE_INDEX: usize = 3;
const FFT_DEVICE_INDEX: usize = 4;
const NUM_FIXED_DEVICES: usize = 5;
const FIXED_DEVICES: [&str; NUM_FIXED_DEVICES] = [
    "out",
    "you",
    "svr",
    "dac",
    "fft"
];
const SEEN_NEITHER: usize = 0;
const SEEN_DAC: usize = 1;
const SEEN_FFT: usize = 2;
const SEEN_BOTH: usize = 3;
const NUM_SEEN_STATES: usize = 4;

#[derive(Clone)]
struct Device {
    id: usize,
    outputs: Option<Vec<usize>>,
    direct_routes_in: usize,
    paths_to_node: [u64; NUM_SEEN_STATES], // Bitmask for seen special nodes
}

impl Device {
    fn new(id: usize) -> Self {
        Self {
            id,
            outputs: None,
            direct_routes_in: 0,
            paths_to_node: [0u64; NUM_SEEN_STATES],
        }
    }

    fn reset(&mut self) {
        self.paths_to_node = [0u64; NUM_SEEN_STATES];
    }
}

impl DayImplementation for Day11 {
    type Output<'a> = u64;
    type Context<'a> = Day11Context;

    fn day(&self) -> u8 { 11 }
    fn example_input(&self) -> Option<&'static str> { Some("aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out") }
    fn example_part_1_result(&self) -> Option<Self::Output<'static>> { Some(5) }
    fn example_part_2_result(&self) -> Option<Self::Output<'static>> { Some(2) }

    fn execute_part_1<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, Self::Context<'a>)> {
        let mut devices = parse_input(input);
        let ordering = generate_ordered_graph(&devices);
        let number_of_paths = count_paths(
            &mut devices,
            &ordering,
            YOU_DEVICE_INDEX,
            OUT_DEVICE_INDEX,
            None);
        devices.iter_mut().for_each(|d| d.reset());

        Ok((number_of_paths, Day11Context { devices, ordering }))
    }

    fn execute_part_2<'a>(&self, input: &'a str, ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        // Annoyingly, my runner doesn't support different inputs for parts
        // 1 and 2, so we have to detect the example input here and replace it.
        let (mut devices, ordering) = if input.len() < 200 {
            let devices = parse_input("svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out");
            let ordering = generate_ordered_graph(&devices);
            (devices, ordering)
        } else {
            (ctx.devices, ctx.ordering)
        };

        Ok(count_paths(
            &mut devices,
            &ordering,
            SVR_DEVICE_INDEX,
            OUT_DEVICE_INDEX,
            Some(SEEN_BOTH)))
    }
}

fn parse_input(input: &str) -> Vec<Device> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut devices: Vec<Device> = Vec::with_capacity(lines.len());
    let mut devices_dict: HashMap<&str, usize> = HashMap::with_capacity(lines.len());

    for (i, &device_name) in FIXED_DEVICES.iter().enumerate() {
        devices_dict.insert(device_name, i);
        devices.push(Device::new(i));
    }

    for line in lines.iter() {
        let mut parts = line.split_ascii_whitespace();

        let device_name = parts.next().unwrap().trim_end_matches(":");
        let outputs = Some(
            parts
            .map(|output_name| devices_dict
                .entry(output_name)
                .or_insert_with(|| {
                    let new_index = devices.len();
                    devices.push(Device::new(new_index));
                    new_index
                })
                .to_owned())
            .collect::<Vec<usize>>());

        for &output_index in outputs.as_ref().unwrap().iter() {
            devices[output_index].direct_routes_in += 1;
        }

        if let Some(device_index) = devices_dict.get(device_name) {
            devices[*device_index].outputs = outputs;
        } else {
            let new_index = devices.len();
            devices_dict.insert(device_name, new_index);
            let mut device = Device::new(new_index);
            device.outputs = outputs;
            devices.push(device);
        }
    }
    devices
}

fn generate_ordered_graph(devices: &[Device]) -> Vec<usize> {
    // Kahn's algorithm for topological sorting
    let mut indegrees: Vec<usize> = devices.iter().map(|d| d.direct_routes_in).collect();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut ordered_graph: Vec<usize> = Vec::with_capacity(devices.len());

    for (i, &indegree) in indegrees.iter().enumerate() {
        if indegree == 0 {
            queue.push_back(i);
        }
    }

    while let Some(device_index) = queue.pop_front() {
        ordered_graph.push(device_index);

        let device = &devices[device_index];
        if let Some(outputs) = &device.outputs {
            for &output_index in outputs.iter() {
                indegrees[output_index] -= 1;
                if indegrees[output_index] == 0 {
                    queue.push_back(output_index);
                }
            }
        }
    }

    ordered_graph
}

fn visit_device(devices: &mut [Device], device_index: usize) {
    let device = &mut devices[device_index];
    if device.id == DAC_DEVICE_INDEX {
        assert!(device.paths_to_node[SEEN_DAC] == 0);
        assert!(device.paths_to_node[SEEN_BOTH] == 0);
        device.paths_to_node[SEEN_BOTH] += device.paths_to_node[SEEN_FFT];
        device.paths_to_node[SEEN_FFT] = 0;
        device.paths_to_node[SEEN_DAC] += device.paths_to_node[SEEN_NEITHER];
        device.paths_to_node[SEEN_NEITHER] = 0;
    } else if device.id == FFT_DEVICE_INDEX {
        assert!(device.paths_to_node[SEEN_FFT] == 0);
        assert!(device.paths_to_node[SEEN_BOTH] == 0);
        device.paths_to_node[SEEN_BOTH] += device.paths_to_node[SEEN_DAC];
        device.paths_to_node[SEEN_DAC] = 0;
        device.paths_to_node[SEEN_FFT] += device.paths_to_node[SEEN_NEITHER];
        device.paths_to_node[SEEN_NEITHER] = 0;
    }

    let device = device.clone();

    if let Some(outputs) = &device.outputs {
        for &output_index in outputs.iter() {
            let output_device = &mut devices[output_index];
            for seen_state in 0..NUM_SEEN_STATES {
                output_device.paths_to_node[seen_state] += device.paths_to_node[seen_state];
            }
        }
    }
}

fn count_paths(
    devices: &mut [Device],
    ordering: &[usize],
    from_idx: usize,
    to_idx: usize,
    seen_requirement: Option<usize>
) -> u64 {
    devices[from_idx].paths_to_node[SEEN_NEITHER] = 1;

    for &device_index in ordering.iter() {
        visit_device(devices, device_index);
    }

    if let Some(seen_state) = seen_requirement {
        devices[to_idx].paths_to_node[seen_state]
    } else {
        devices[to_idx].paths_to_node.iter().sum()
    }
}