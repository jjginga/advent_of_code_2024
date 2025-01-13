use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;
use itertools::Either;

type Position = (usize, usize);

#[derive(Debug, Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub obstacles: HashSet<Position>,
    pub start: Position,
    pub end: Position,
}

fn directions() -> [(isize, isize); 4] {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: Position,
    cost: usize,
    priority: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

fn heuristic(a: Position, b: Position) -> usize{
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day20)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    let grid = generate_grid(input);
    let min_path = a_star(grid.start, grid.end, grid.width, grid.height, &grid.obstacles, true)
                .and_then(|result| if let Either::Left(path) = result { Some(path) } else { None })
                .unwrap();
    let max_psicoseconds = min_path.len().saturating_sub(1);
    let mut count = 0;
    let mut difference_counts: HashMap<usize, usize> = HashMap::new();

    let obstacles_set: HashSet<_> = grid.obstacles.iter().copied().collect();

    let obstacles_set_filtered: HashSet<Position> = obstacles_set
        .iter()
        .cloned()
        .filter(|&obstacle| {
            let adjacent_points = min_path.iter().filter(|&&path_position| {
                directions().iter().any(|&(dx, dy)| {
                    (path_position.0 as isize + dx, path_position.1 as isize + dy) == (obstacle.0 as isize, obstacle.1 as isize)
                })
            }).count();
            adjacent_points >= 2 
        })
        .collect();
    
    
    for &obstacle in &obstacles_set_filtered {
        
        let mut reduced_obstacles = grid.obstacles.clone();
        reduced_obstacles.remove(&obstacle);

        if let Some(Either::Right(path_length)) = a_star(
            grid.start,
            grid.end,
            grid.width,
            grid.height,
            &reduced_obstacles,
            false,
        ) {
            let save = max_psicoseconds.saturating_sub(path_length);

            if save >= 2 && save % 2 == 0{
                *difference_counts.entry(max_psicoseconds - path_length).or_insert(0) += 1;
                if max_psicoseconds - path_length >= 100 {
                    count += 1;
                }
            }
        }
    }
        
    count
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &[String]) -> usize {
    let grid = generate_grid(input);
    let min_path = a_star(grid.start, grid.end, grid.width, grid.height, &grid.obstacles, true)
        .and_then(|result| if let Either::Left(path) = result { Some(path) } else { None })
        .unwrap();

    let original_cost = min_path.len().saturating_sub(1);
    
    //precompute cost from start and from end for each point in min_pat
    let mut cost_from_start = HashMap::new();
    let mut cost_from_end = HashMap::new();

    for (i, &position) in min_path.iter().enumerate() {
        cost_from_start.insert(position, i);
        cost_from_end.insert(position, min_path.len() - i - 1);
    }

    //for each pair of points (A, B) in min_path, 
    //whose Manhattan distance is <= 20,
    //compute how much time we'dd save by cheating from A to B
    let mut count = 0;
    let mut difference_counts: HashMap<usize, usize> = HashMap::new();
    let max_cheat_distance = 20;

    for (_i, &pt_a) in min_path.iter().enumerate() {
        for (_j, &pt_b) in min_path.iter().enumerate() {
            //manhattan distance
            let dist = manhattan_distance(pt_a, pt_b);
            if dist <= max_cheat_distance {
                //time = cost_from_start[A] + (cheat) + cost_from_end[B]
                //cheat length = dist
                let new_cost = cost_from_start[&pt_a] + dist + cost_from_end[&pt_b];
                let save = original_cost.saturating_sub(new_cost);

                //only consider if it saves at least 50 picoseconds and is multiple of 2, etc.
                if save >= 50 && save % 2 == 0 {
                    *difference_counts.entry(save).or_insert(0) += 1;
                    if save >= 100 {
                        count += 1;
                    }
                }
            }
        }
    }

    /*let mut sorted: Vec<(&usize, &usize)> = difference_counts.iter().collect();
    sorted.sort_by_key(|&(save, _)| *save);

    for (save, cnt) in sorted {
        println!("There are {} cheats that save {} picoseconds.", cnt, save);
    }*/

    count
}

fn manhattan_distance(a: Position, b: Position) -> usize {
    (a.0 as isize - b.0 as isize).abs() as usize
        + (a.1 as isize - b.1 as isize).abs() as usize
}


pub fn generate_grid(input: &[String]) -> Grid {
    let mut obstacles = HashSet::new();
    let mut start = None;
    let mut end = None;
    let width = input.get(0).map(|line| line.len()).unwrap();
    let height = input.len();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {obstacles.insert((x, y));},
                'S' => {start = Some((x, y));},
                'E' => {end = Some((x, y));},
                _ => {}
            }
        }
    }

    Grid {
        width,
        height,
        obstacles,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}


pub fn a_star(
    start: Position, 
    end: Position, 
    width: usize, 
    height: usize,
    obstacles: &HashSet<Position>,
    return_path: bool,
) -> Option<Either<Vec<Position>,usize>>{
    let mut open_set = BinaryHeap::new();
    open_set.push(Node {
        position: start,
        cost: 0,
        priority: heuristic(start, end),
    });

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);
    let mut came_from = HashMap::new();

    while let Some(Node {position, cost, .. }) = open_set.pop() {
        if position == end {
            if return_path {
                let mut path = Vec::new();
                let mut current = end;
                path.push(current);

                while let Some(&prev) = came_from.get(&current) {
                    current = prev;
                    path.push(current);
                }
                path.reverse();
                return Some(Either::Left(path));
            } else {
                return Some(Either::Right(cost));
            }
        }

        for (dx, dy) in directions().iter() {
            let new_position: (usize, usize) = (
                (position.0 as isize + dx).try_into().unwrap_or(usize::MAX),
                (position.1 as isize + dy).try_into().unwrap_or(usize::MAX),
            );

            if new_position.0 > width || new_position.1 > height-1 {
                continue;
            }

            let new_position = (new_position.0, new_position.1);

            if obstacles.contains(&new_position) {
                continue;
            }

            let new_cost = g_score.get(&position).unwrap_or(&usize::MAX) + 1;

            if new_cost < *g_score.get(&new_position).unwrap_or(&usize::MAX) {
                g_score.insert(new_position, new_cost);
                open_set.push(Node {
                    position: new_position,
                    cost: new_cost,
                    priority: new_cost + heuristic(new_position, end),
                });
                came_from.insert(new_position, position);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day20_part1() {
        let example_input = concat!(
            "###############\n",
            "#...#...#.....#\n",
            "#.#.#.#.#.###.#\n",
            "#S#...#.#.#...#\n",
            "#######.#.#.###\n",
            "#######.#.#...#\n",
            "#######.#.###.#\n",
            "###..E#...#...#\n",
            "###.#######.###\n",
            "#...###...#...#\n",
            "#.#####.#.###.#\n",
            "#.#...#.#.#...#\n",
            "#.#.#.#.#.#.###\n",
            "#...#...#...###\n",
            "###############\n"
        );
        let input = generate_input(example_input);
        let result = solve_part1(&input);
        assert_eq!(result, 0); 
    }

     #[test]
    fn test_day20_part2() {
        let example_input = concat!(
            "###############\n",
            "#...#...#.....#\n",
            "#.#.#.#.#.###.#\n",
            "#S#...#.#.#...#\n",
            "#######.#.#.###\n",
            "#######.#.#...#\n",
            "#######.#.###.#\n",
            "###..E#...#...#\n",
            "###.#######.###\n",
            "#...###...#...#\n",
            "#.#####.#.###.#\n",
            "#.#...#.#.#...#\n",
            "#.#.#.#.#.#.###\n",
            "#...#...#...###\n",
            "###############\n"
        );
        let input = generate_input(example_input);
        let result = solve_part2(&input);
        assert_eq!(result, 0); 
    }
}