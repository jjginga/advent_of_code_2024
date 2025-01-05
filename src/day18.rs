use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, BinaryHeap, HashMap};
use std::cmp::Ordering;

type Position = (u8, u8);

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: Position,
    cost: u16,
    priority: u16,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//manhatan distance
fn heuristic(a: Position, b: Position) -> u16 {
    ((a.0 as i16 - b.0 as i16).abs() + (a.1 as i16 - b.1 as i16).abs()) as u16
}

//generate directions for movement
fn directions() -> [(i8, i8); 4] {
    [(-1, 0), (1, 0), (0, -1), (0, 1)] //up, down, left, right
}

#[aoc_generator(day18)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[String]) -> u16 {
    let obstacles = process_map(input, 1024);
    let dimension = 70;
    let start = (0, 0);
    let goal = (dimension, dimension);
    a_star(start, goal, dimension, &obstacles)    
}

#[aoc(day18, part2)]
pub fn solve_part2(_input: &[String]) -> u16 {
    let position = find_blocking_obstacle(_input, 70).unwrap();
    println!("Position: {:?}", position);
    position.0 as u16 + position.1 as u16
}

fn find_blocking_obstacle(input: &[String], dimension: u8) -> Option<Position> {
    let all_obstacles: Vec<Position> = input.iter().map(|line| parse_coordinates(line)).collect();

    let mut current_obstacles: HashSet<Position> = all_obstacles.iter().take(1024).cloned().collect();

    for obstacle in all_obstacles.iter().skip(1024) {
        current_obstacles.insert(*obstacle);

        if a_star((0, 0), (dimension, dimension), dimension, &current_obstacles) == 0 {
            return Some(*obstacle);
        }
    }

    None
}


fn a_star(start: Position, goal: Position, dimension: u8, obstacles: &HashSet<Position>) -> u16 {

    let mut open_set = BinaryHeap::new();
    open_set.push(Node { position: start, cost: 0, priority: heuristic(start, goal) });

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    while let Some(Node { position, cost, priority: _ }) = open_set.pop() {

        if position == goal {
            return cost as u16;
        }

        for (dx, dy) in directions().iter() {
            let new_position = (
                (position.0 as i8 + dx).try_into().unwrap_or(0),
                (position.1 as i8 + dy).try_into().unwrap_or(0),
            );

            if new_position.0 > dimension || new_position.1 > dimension {
                continue;
            }

            if obstacles.contains(&new_position) {
                continue;
            }

            let tentative_g_score = g_score.get(&position).unwrap_or(&u16::MAX) + 1;

            if tentative_g_score < *g_score.get(&new_position).unwrap_or(&u16::MAX) {
                g_score.insert(new_position, tentative_g_score);
                let priority = tentative_g_score + heuristic(new_position, goal);
                open_set.push(Node { position: new_position, cost: tentative_g_score, priority });
            }
        }
    }
    0
}

fn process_map(input: &[String], num_obstacles: usize) -> HashSet<Position>{
    let mut map = HashSet::new();

    input.iter().take(num_obstacles).for_each(|line| {
        map.insert(parse_coordinates(line));
    });

    map
}

fn parse_coordinates(line: &str) -> Position {
    let mut parts = line.split(",");
    let x = parts.next().unwrap().parse().unwrap();
    let y = parts.next().unwrap().parse().unwrap();

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day18_part1() {
        let example_input = concat!(
            "5,4\n",
            "4,2\n",
            "4,5\n",
            "3,0\n",
            "2,1\n",
            "6,3\n",
            "2,4\n",
            "1,5\n",
            "0,6\n",
            "3,3\n",
            "2,6\n",
            "5,1\n",
            "1,2\n",
            "5,5\n",
            "2,5\n",
            "6,5\n",
            "1,4\n",
            "0,4\n",
            "6,4\n",
            "1,1\n",
            "6,1\n",
            "1,0\n",
            "0,5\n",
            "1,6\n",
            "2,0"
        );
        let parsed_input = generate_input(example_input);
        let obstacles = process_map(&parsed_input, 12);
        let dimension = 6;
        let result = a_star((0, 0), (dimension, dimension), dimension, &obstacles);
        println!("Result: {}", result);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_day18_part2() {
        let example_input = "";
        let parsed_input = generate_input(example_input);
        assert_eq!(solve_part2(&parsed_input), 0);
    }
}
