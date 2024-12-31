use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

type Position = (i16, i16);

const DEBUG: bool = false;

#[derive(Eq, PartialEq)]
struct Node {
    position: Position,
    cost: i32, 
    priority: i32, //cost + heuristic
    direction: Position,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//manhatan distance
fn heuristic(a: Position, b: Position) -> i32 {
    i32::from(a.0 - b.0).abs() + i32::from(a.1 - b.1).abs()
}

//generate directions for movement
fn directions() -> [(i16, i16); 4] {
    [(-1, 0), (1, 0), (0, -1), (0, 1)] //north, south, west, east
}

//initialize the open set
fn initialize_open_set(start: Position, goal: Position) -> (BinaryHeap<Node>, HashMap<Position, i32>) {
    let mut open_set = BinaryHeap::new();
    let mut g_score = HashMap::new();
    g_score.insert(start, 0);
    open_set.push(Node {
        position: start,
        cost: 0,
        priority: heuristic(start, goal),
        direction: (1, 0),
    });
    (open_set, g_score)
}

#[aoc_generator(day16)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
    let grid = parse_warehouse(input);

    //find the start and end positions
    let (start, goal) = find_positions(&grid);


    let (mut open_set, mut g_score) = initialize_open_set(start, goal);
    let mut came_from = HashMap::new();

    //A* algorithm
    while let Some(current) = open_set.pop() {
        //check if we reached the goal
        if current.position == goal {
            if DEBUG {
                let mut path = Vec::new();
                let mut current_pos = current.position;
                path.push(current_pos);

                while let Some(&prev_pos) = came_from.get(&current_pos) {
                    path.push(prev_pos);
                    current_pos = prev_pos;
                }

                path.reverse();
                visualize_path(input, &path);
            }
            return current.cost;
        }

        for &direction in directions().iter() {
            let neighbor = (
                current.position.0 + direction.0,
                current.position.1 + direction.1,
            );

            if !is_valid_neighbor(&grid, &neighbor) {
                continue;
            }            

            //if the direction is 90, add a cost of 1000
            let direction_change_cost = compute_direction_change_cost(current.direction, direction);

            let tentative_g_score =
                g_score.get(&current.position).unwrap_or(&i32::MAX) + 1 + direction_change_cost;

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                g_score.insert(neighbor, tentative_g_score);
                came_from.insert(neighbor, current.position);
                open_set.push(Node {
                    position: neighbor,
                    cost: tentative_g_score,
                    priority: tentative_g_score + heuristic(neighbor, goal),
                    direction,
                });
            }
        }
    }

    -1
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &[String]) -> i32 {
    let optimal_cost = solve_part1(input);
    find_all_paths_with_same_cost(input, optimal_cost)
}

//parse the warehouse
pub fn parse_warehouse(input: &[String]) -> HashMap<Position, char> {
    input.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as i16, y as i16), ch))
        })
        .collect()
}

//visualize the path
fn visualize_path(input: &[String], path: &[Position]) {
    let mut labyrinth: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    for &(x, y) in path.iter() {
        if labyrinth[y as usize][x as usize] != 'S' && labyrinth[y as usize][x as usize] != 'E' {
            labyrinth[y as usize][x as usize] = '*';
        }
    }
    for line in labyrinth {
        println!("{}", line.iter().collect::<String>());
    }
}

pub fn find_all_paths_with_same_cost(input: &[String], optimal_cost: i32) -> i32 {
    let grid: HashMap<(i16, i16), char> = parse_warehouse(input);

    let (start, goal) = find_positions(&grid);

    //g_score and came_from are indexed by (position, direction)
    let mut g_score: HashMap<(Position, (i16, i16)), i32> = HashMap::new();
    let mut came_from: HashMap<(Position, (i16, i16)), Vec<(Position, (i16, i16))>> = HashMap::new();

    let mut open_set = BinaryHeap::new();
    let mut tiles_on_paths = HashSet::new();

    //initial state: at the start position, facing East (1,0)
    g_score.insert((start, (1, 0)), 0);
    open_set.push(Node {
        position: start,
        cost: 0,
        priority: 0,
        direction: (1, 0), //east
    });

    //dijkstra's algorithm
    while let Some(current) = open_set.pop() {
        //skip nodes exceeding the optimal cost
        if current.cost > optimal_cost {
            continue;
        }

        //current state (position + direction) used as key
        let current_state = (current.position, current.direction);

        //check if the current cost is still the best one recorded
        if let Some(&best_cost) = g_score.get(&current_state) {
            if current.cost > best_cost {
                continue;
            }
        }

        //try to expand to each possible direction
        for &dir in directions().iter() {
            let neighbor_pos = (
                current.position.0 + dir.0,
                current.position.1 + dir.1,
            );

            //check if it's a wall or out of bounds
            if !is_valid_neighbor(&grid, &neighbor_pos) {
                continue;
            }

            //calculate the cost of turning (if the direction is perpendicular)
            let direction_change_cost = compute_direction_change_cost(current.direction, dir);

            let tentative_g_score = current.cost + 1 + direction_change_cost;

            //define the neighbor state as (position, direction)
            let neighbor_state = (neighbor_pos, dir);

            //if the cost is within the optimal limit, process it
            if tentative_g_score <= optimal_cost {
                let old_score = *g_score.get(&neighbor_state).unwrap_or(&i32::MAX);
                if tentative_g_score < old_score {
                    //found a better path to neighbor_state
                    g_score.insert(neighbor_state, tentative_g_score);
                    came_from.insert(neighbor_state, vec![current_state]);
                    open_set.push(Node {
                        position: neighbor_pos,
                        cost: tentative_g_score,
                        priority: tentative_g_score,
                        direction: dir,
                    });
                } else if tentative_g_score == old_score {
                    //tie in cost => add another predecessor
                    came_from
                        .entry(neighbor_state)
                        .or_insert_with(Vec::new)
                        .push(current_state);
                }
            }
        }
    }

    //collect all states (goal, dir) that have this minimum cost
    let goal_states: Vec<_> = g_score
        .iter()
        .filter_map(|(&(pos, dir), &cost)| {
            if pos == goal && cost == optimal_cost {
                Some((pos, dir))
            } else {
                None
            }
        })
        .collect();

    //reconstruct all paths: for each optimal (goal, dir), backtrack to (start, (1,0))
    let mut stack = vec![];
    for gs in goal_states {
        stack.push((gs, vec![gs.0])); 
    }

    while let Some((current, path)) = stack.pop() {
        if current.0 == start {
            tiles_on_paths.extend(path);
            continue;
        }

        //if predecessors exist
        if let Some(preds) = came_from.get(&current) {
            for &prev in preds {
                let mut new_path = path.clone();
                new_path.push(prev.0); //add only the position of the predecessor
                stack.push((prev, new_path));
            }
        }
    }

    tiles_on_paths.len() as i32
}

fn is_valid_neighbor(grid: &HashMap<Position, char>, neighbor: &Position) -> bool {
    if let Some(&ch) = grid.get(neighbor) {
        ch != '#' //valid if it's not a wall
    } else {
        false //out of bounds
    }
}

fn compute_direction_change_cost(current_direction: Position, new_direction: Position) -> i32 {
    if current_direction != (0, 0)
        && (current_direction.0 * new_direction.0 + current_direction.1 * new_direction.1 == 0)
    {
        1000
    } else {
        0
    }
}

fn find_positions(grid: &HashMap<Position, char>) -> (Position, Position) {
    let start = grid
        .iter()
        .find(|(_, &ch)| ch == 'S')
        .map(|(pos, _)| *pos)
        .unwrap();

    let goal = grid
        .iter()
        .find(|(_, &ch)| ch == 'E')
        .map(|(pos, _)| *pos)
        .unwrap();

    (start, goal)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day16_part1() {
        let example_input = "\
            ###############
            #.......#....E#
            #.#.###.#.###.#
            #.....#.#...#.#
            #.###.#####.#.#
            #.#.#.......#.#
            #.#.#####.###.#
            #...........#.#
            ###.#.#####.#.#
            #...#.....#.#.#
            #.#.#.###.#.#.#
            #.....#...#.#.#
            #.###.#.#.#.#.#
            #S..#.....#...#
            ###############";
        let parsed_input = generate_input(example_input);
        assert_eq!(solve_part1(&parsed_input), 7036);
    }

    #[test]
    fn test_day16_part2() {
        let example_input = "\
            ###############
            #.......#....E#
            #.#.###.#.###.#
            #.....#.#...#.#
            #.###.#####.#.#
            #.#.#.......#.#
            #.#.#####.###.#
            #...........#.#
            ###.#.#####.#.#
            #...#.....#.#.#
            #.#.#.###.#.#.#
            #.....#...#.#.#
            #.###.#.#.#.#.#
            #S..#.....#...#
            ###############";
        let parsed_input = generate_input(example_input);
        assert_eq!(solve_part2(&parsed_input), 45);
    }
}
