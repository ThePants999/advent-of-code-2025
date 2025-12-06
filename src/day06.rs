use advent_of_code_rust_runner::{DayImplementation, Result};

pub struct Day06;

pub struct Day06Context<'a> {
    input_lines: Vec<&'a str>
}

enum Operator {
    Add,
    Multiply
}

impl std::str::FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => Err(())
        }
    }
}

impl From<u8> for Operator {
    fn from(c: u8) -> Self {
        match c as char {
            '+' => Operator::Add,
            '*' => Operator::Multiply,
            _ => panic!("Invalid operator character")
        }
    }
}

struct Equation {
    operands: Vec<u64>,
    operator: Operator,
    num_operands: usize
}

impl Equation {
    fn new(operator: Operator, capacity: usize) -> Self {
        Equation {
            operands: Vec::with_capacity(capacity),
            operator,
            num_operands: capacity
        }
    }

    fn from_input(input: &[&str], index: usize) -> Self {
        let operator_line = input.last().unwrap().as_bytes();
        let operand_lines = &input[0..input.len()-1];
        let operator: Operator = Operator::from(operator_line[index]);
        let mut num_operands = 1usize;

        for c in operator_line[index+1..].iter() {
            if !c.is_ascii_whitespace() {
                num_operands -= 1;
                break;
            }
            num_operands += 1;
        }

        let operands = (0..num_operands)
            .map(|operand_ix| {
                operand_lines
                    .iter()
                    .map(|line| line.as_bytes()[index + operand_ix] as char)
                    .collect::<String>()
                    .trim()
                    .parse::<u64>()
                    .unwrap()
            })
            .collect::<Vec<u64>>();

        Equation {
            operands,
            operator,
            num_operands
        }
    }

    fn add_operand(&mut self, operand: u64) {
        self.operands.push(operand);
    }

    fn evaluate(&self) -> u64 {
        match self.operator {
            Operator::Add => self.operands.iter().sum(),
            Operator::Multiply => self.operands.iter().product()
        }
    }
}

impl DayImplementation for Day06 {
    type Output<'a> = u64;
    type Context<'a> = Day06Context<'a>;

    fn day(&self) -> u8 { 6 }
    fn example_input(&self) -> Option<&'static str> { Some("123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ") }
    fn example_part_1_result(&self) -> Option<Self::Output<'static>> { Some(4277556) }
    fn example_part_2_result(&self) -> Option<Self::Output<'static>> { Some(3263827) }

    fn execute_part_1<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, Self::Context<'a>)> {
        let lines = input.lines().collect::<Vec<&str>>();
        let num_operands = lines.len() - 1;
        let mut equations = lines[num_operands]
            .split_ascii_whitespace()
            .map(|op_str| Equation::new(op_str.parse().unwrap(), num_operands))
            .collect::<Vec<Equation>>();
        for line in &lines[0..num_operands] {
            for (i, num_str) in line.split_ascii_whitespace().enumerate() {
                let operand = num_str.parse::<u64>().expect("Failed to parse operand");
                equations[i].add_operand(operand);
            }
        }
        let result = equations.iter().map(|eq| eq.evaluate()).sum();
        Ok((result, Day06Context { input_lines: lines }))
    }

    fn execute_part_2<'a>(&self, _input: &'a str, ctx: Self::Context<'a>) -> Result<Self::Output<'a>> {
        let mut index = 0usize;
        let operator_line = ctx.input_lines.last().unwrap();
        let mut equations: Vec<Equation> = Vec::new();
        while index < operator_line.len() {
            let equation = Equation::from_input(&ctx.input_lines, index);
            index += equation.num_operands + 1;
            equations.push(equation);
        }
        Ok(equations.iter().map(|eq| eq.evaluate()).sum())
    }
}
