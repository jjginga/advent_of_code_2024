use std::collections::HashMap;
use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
    let map = create_map(input);
    let mut positions: HashSet<(i16, i16)> = HashSet::new();
    let (mut position, ch) = map.iter()
                            .find(|&(_, &value)| value != '.' && value != '#')
                            .map(|(&position, &value)| (position, value))
                            .unwrap();
    let mut direction = get_direction(&ch);
    positions.insert(position);
    
    loop {
        let next_position = (position.0 + direction.0, position.1 + direction.1);
    
        if !map.contains_key(&next_position) {
            break; 
        }
    
        if let Some(&next_value) = map.get(&next_position) {
            if next_value == '#' {
                direction = rotate_90(direction); 
            } else {
                position = next_position;
                positions.insert(position);
            }
        }
    }
    
   

    positions.len() as i32
}

#[aoc(day6, part2)]
pub fn solve_part2(_input: &[String]) -> i32 {
    0
}

fn create_map(input: &[String]) -> HashMap<(i16, i16), char> {
    let mut map = HashMap::new();
    for (row, line) in input.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            map.insert(((row + 1) as i16, (col + 1) as i16), ch);
        }
    }
    map
}

fn get_direction(value: &char) -> (i16, i16) {
    if *value == '>' {
        (0,1)
    } else if *value == '<' {
        (0,-1)
    } else if *value == 'v' {
        (1,0)
    } else if *value == '^' {
        (-1,0)
    } else {
        (0,0) 
    }
}
fn rotate_90(direction: (i16, i16)) -> (i16, i16) {
    match direction {
        (0, 1) => (1, 0),  
        (1, 0) => (0, -1), 
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1), 
        _ => (0, 0),       
    }
}
