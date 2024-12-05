use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    input.len()
}


/*
//part 2
#[aoc(day3, part2)]
pub fn solve_part2(input: &[String]) -> usize {
    0 as usize
}
*/