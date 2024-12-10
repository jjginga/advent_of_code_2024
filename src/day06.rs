use std::collections::HashMap;
use itertools::Itertools;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
    let map = create_map(input);
    let initial_pos = map.iter()
                         .find(|&(_, &value)| value != '.' && value != '#')
                         .map(|(&position, _)| position);

    //print map, line bt line
    let mut last_x = 0;

    for (&(x, _), &value) in map.iter().sorted_by_key(|(&(x, y), _)| (x, y)) {
        if x != last_x {
            println!();
            last_x = x;
        }
        print!("{} ", value);
    }
    println!();
    0
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[String]) -> i32 {
    0
}

fn create_map(input: &[String]) -> HashMap<(i16, i16), char> {
    let mut map = HashMap::new();
    for (y, line) in input.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            map.insert((x as i16, y as i16), ch);
        }
    }
    map
}

fn get_direction(value: &char) -> (i16, i16) {
    if value == '>'
        (1,0)
    else if value == '<'
        (-1,0)
    else if value == 'v'
        (0,1)
    else if value == '^'
        (0,-1)
    else (0,0)
}