use advent_of_code_rust_runner::DayImplementation;

pub struct Day01;

pub struct Day01Context<'a> {
    words: Vec<&'a str>
}

impl DayImplementation for Day01 {
    type Output<'a> = &'a str;
    type Context<'a> = Day01Context<'a>;

    fn day(&self) -> u8 { 1 }
    fn example_input(&self) -> &'static str { "hello world" }
    fn example_part_1_result(&self) -> &'static str { "hello" }
    fn example_part_2_result(&self) -> &'static str { "world" }

    fn execute_part_1<'a>(&self, input: &'a str) -> (&'a str, Self::Context<'a>) {
        let words: Vec<&'a str> = input.split_whitespace().collect();
        let word1 = words[0];
        (word1, Day01Context { words })
    }

    fn execute_part_2<'a>(&self, _input: &'a str, context: Self::Context<'a>) -> &'a str {
        context.words[1]
    }
}