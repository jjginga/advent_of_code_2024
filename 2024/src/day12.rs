use aoc_runner_derive::{aoc, aoc_generator};
use crate::utils::create_map;
use std::collections::{HashMap, HashSet, VecDeque};

const DIRS: [(i16, i16); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

struct Region {
    area: usize,
    perimeter: usize,
    sides: usize,
}

impl Region {
    fn new(plant_type: char, points: Vec<(i16, i16)>, map: &HashMap<(i16, i16), char>) -> Self {
        let plant_type = plant_type;
        let area = points.len();
        let perimeter = points.iter().map(|&(x, y)| {
            let neighbors = vec![
                (x - 1, y),
                (x + 1, y),
                (x, y - 1),
                (x, y + 1),
            ];

            neighbors.iter().filter(|&&neighbor| {
                match map.get(&neighbor) {
                    Some(&neighbor_type) => neighbor_type != plant_type,
                    None => true, 
                }
            }).count()
        }).sum();
        
        let sides = count_corners(&points, plant_type, map);

        Self {
            area,
            perimeter,
            sides,
        }
    }

    fn price(&self) -> usize {
        self.area * self.perimeter
    }

    fn price_with_sides(&self) -> usize {
        self.area * self.sides
    }
}


#[aoc_generator(day12)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    let map = create_map(input);
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for (&position, &plant_type) in &map {
        //if not visited
        if !visited.contains(&position) {
            let points = bfs_region(position, plant_type, &map, &mut visited);
            //create new region and calculate price
            let region = Region::new(plant_type, points, &map);
            total_price += region.price();
        }
    }

    total_price
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[String]) -> usize {
    let map = create_map(input);
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for (&position, &plant_type) in &map {
        if !visited.contains(&position) {
            let points = bfs_region(position, plant_type, &map, &mut visited);
            let region = Region::new(plant_type, points, &map);
            total_price += region.price_with_sides();
        }
    }

    total_price
}

fn bfs_region(
    start: (i16, i16),
    plant_type: char,
    map: &HashMap<(i16, i16), char>,
    visited: &mut HashSet<(i16, i16)>,
) -> Vec<(i16, i16)> {

    let mut queue = VecDeque::new();
    let mut points = Vec::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some((x, y)) = queue.pop_front() {
        points.push((x, y));

        for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let neighbor = (x + dx, y + dy);
            if !visited.contains(&neighbor) {
                if let Some(&neighbor_type) = map.get(&neighbor) {
                    if neighbor_type == plant_type {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    points
}

fn count_corners(
    vec_region: &Vec<(i16, i16)>,
    _plant_type: char,
    _map: &HashMap<(i16, i16), char>,
) -> usize {
    let region: HashSet<(i16, i16)> = vec_region.iter().cloned().collect();
    let mut sides = 0;

    for &missing_dir in DIRS.iter() {
        let mut found: HashSet<(i16, i16)> = HashSet::new();

        for &cell in &region {
            if found.contains(&cell) {
                continue;
            }

            let check_dir = (cell.0 + missing_dir.0, cell.1 + missing_dir.1);

            if region.contains(&check_dir) {
                continue;
            }

            //this cell has a missing neighbor in the current direction, indicating a side
            found.insert(cell);
            sides += 1;

            //determine the perpendicular directions to explore adjacent sides
            let left = (missing_dir.1, missing_dir.0);
            let right = (-missing_dir.1, -missing_dir.0);

            for &lr_dir in &[left, right] {
                let mut current = cell;

                loop {
                    current = (current.0 + lr_dir.0, current.1 + lr_dir.1);
                    let neighbor = (current.0 + missing_dir.0, current.1 + missing_dir.1);

                    if region.contains(&current) && !region.contains(&neighbor) {
                        found.insert(current);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    //println!("Plant Type: {}  Sides: {}", _plant_type, sides);
    sides
}