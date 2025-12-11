use advent_of_code_rust_runner::{DayImplementation, Result};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day11;

pub struct Day11Context {
    devices: Vec<Device>
}

type Device = Option<Vec<usize>>;

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

impl DayImplementation for Day11 {
    type Output<'a> = u32;
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
        let devices = parse_input(input);
        let number_of_paths = count_paths(
            &devices,
            YOU_DEVICE_INDEX,
            OUT_DEVICE_INDEX,
            None);
        Ok((number_of_paths, Day11Context { devices }))
    }

    fn execute_part_2<'a>(&self, input: &'a str, ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        // Annoyingly, my runner doesn't support different inputs for parts
        // 1 and 2, so we have to detect the example input here and replace it.
        let devices = if input.len() < 200 {
            parse_input("svr: aaa bbb
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
hhh: out")
        } else {
            ctx.devices
        };

        let number_of_paths = count_paths(
            &devices,
            SVR_DEVICE_INDEX,
            OUT_DEVICE_INDEX,
            Some(&[DAC_DEVICE_INDEX, FFT_DEVICE_INDEX]));
        Ok(number_of_paths)
    }
}

fn parse_input(input: &str) -> Vec<Device> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut devices: Vec<Device> = Vec::with_capacity(lines.len());
    let mut devices_dict: HashMap<&str, usize> = HashMap::with_capacity(lines.len());

    for (i, &device_name) in FIXED_DEVICES.iter().enumerate() {
        devices_dict.insert(device_name, i);
        devices.push(None);
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
                    devices.push(None);
                    new_index
                })
                .to_owned())
            .collect::<Vec<usize>>());
        if let Some(device_index) = devices_dict.get(device_name) {
            devices[*device_index] = outputs;
        } else {
            let new_index = devices.len();
            devices_dict.insert(device_name, new_index);
            devices.push(outputs);
        }
    }
    devices
}

fn count_paths(
    devices: &[Device],
    from_idx: usize,
    to_idx: usize,
    via_idxs: Option<&[usize]>
) -> u32 {
    let mut visited: HashSet<Vec<usize>> = HashSet::new();
    let mut queue: VecDeque<Vec<usize>> = VecDeque::new();
    let mut number_of_paths = 0u32;
    queue.push_back(vec![from_idx]);

    while let Some(path) = queue.pop_back() {
        if visited.contains(&path) {
            continue;
        }
        visited.insert(path.clone());

        let current_device_index = *path.last().unwrap();
        if current_device_index == to_idx {
            if let Some(via_idxs) = via_idxs {
                if via_idxs.iter().all(|&via_idx| path.contains(&via_idx)) {
                    number_of_paths += 1;
                }
            } else {
                // No via requirement
                number_of_paths += 1;
            }
            continue;
        }

        let outputs = &devices[current_device_index].as_ref().expect("Device should have outputs here");
        for &next_device_index in outputs.iter() {
            let mut new_path = path.clone();
            new_path.push(next_device_index);
            if !visited.contains(&new_path) {
                queue.push_back(new_path);
            }
        }
    }
    number_of_paths
}