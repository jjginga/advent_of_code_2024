use aoc_runner_derive::{aoc, aoc_generator};
use crate::utils::create_map;
use std::collections::VecDeque;
use std::collections::HashSet;


#[aoc_generator(day10)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
    let map = create_map(input);
    
    //find all trailheads
    let trailheads = map.iter()
                        .filter(|&(_, &value)| value == '0')
                        .map(|(&position, _)| position)
                        .collect::<Vec<(i16, i16)>>();

    //can only move horizontally or vertically
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    //use BFS to explore all possible paths from trailhead
    let mut total = 0;

    for &trailhead in &trailheads{
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((trailhead, 0));

        let mut reachable_peaks = 0;

        while let Some((current, height)) = queue.pop_front(){
            if visited.contains(&current){
                continue;
            }

            visited.insert(current);

            //check if current position is a peak
            if let Some(&value) = map.get(&current){
                if value == '9'{
                    reachable_peaks += 1;
                    continue; //no need to explore further
                }
            }

            //explore all possible directions
            for &(dx, dy) in &directions{
                let next = (current.0 + dx, current.1 + dy);
                if let Some(&next_value) = map.get(&next){
                    if next_value.to_digit(10) == Some(height +1) {
                        queue.push_back((next, height + 1));
                    }
                }
            }
        }
        total += reachable_peaks;
    }    
    total
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[String]) -> i32 {
    let map = create_map(input);

    //find the trailheads
    let trailheads = map
        .iter()
        .filter(|&(_, &value)| value == '0')
        .map(|(&position, _)| position)
        .collect::<Vec<(i16, i16)>>();

    //movements
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut total_rating = 0;

    //BFS to explore all possible paths from trailhead
    for &trailhead in &trailheads {
        let mut queue = VecDeque::new();
        queue.push_back((trailhead, 0, vec![trailhead])); 

        let mut trails = HashSet::new();

        while let Some((current, height, path)) = queue.pop_front() {
            
            if let Some(&value) = map.get(&current) {
                if value == '9' {
                    trails.insert(path.clone()); 
                    continue;
                }

                
                for &(dx, dy) in &directions {
                    let next = (current.0 + dx, current.1 + dy);
                    if let Some(&next_value) = map.get(&next) {
                        if next_value.to_digit(10) == Some(height + 1) {
                            let mut new_path = path.clone();
                            new_path.push(next); 
                            queue.push_back((next, height + 1, new_path));
                        }
                    }
                }
            }
        }

        
        total_rating += trails.len();
    }

    total_rating as i32
}
