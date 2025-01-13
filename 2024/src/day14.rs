use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

struct Robot{
    position: (i16, i16),
    speed: (i16, i16),
}

impl Robot{
    pub fn new(position: (i16, i16), speed: (i16, i16)) -> Self {
        Robot {
            position,
            speed,
        }
    }

    pub fn move_position(&mut self, width: i16, height: i16) {
        let (x, y) = self.position;
        let (dx, dy) = self.speed;
        self.position = (
            (x + dx).rem_euclid(width),
            (y + dy).rem_euclid(height),
        );
    }
}

#[aoc_generator(day14)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
    solve_part1_with_dimensions(input, 101, 103)
}

fn solve_part1_with_dimensions(input: &[String], width: i16, height: i16) -> i32 {
    let mut robots = parse_robots(input);

    robots.iter_mut().for_each(|robot| {
        for _ in 0..100 {
            robot.move_position(width, height);
        }
    });
    
    let mut quadrant_counts = [0; 4];    

    robots.iter().for_each(|robot| {
        let (x, y) = robot.position;

        if x == width / 2 || y == height / 2 {
            return;
        }

        if x < width / 2 && y < height / 2 {
            quadrant_counts[0] += 1;
        } else if x >= width / 2 && y < height / 2 {
            quadrant_counts[1] += 1;
        } else if x < width / 2 && y >= height / 2 {
            quadrant_counts[2] += 1;
        } else {
            quadrant_counts[3] += 1;
        }
    });

    println!("{:?}", quadrant_counts);

    quadrant_counts.iter().product()    
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &[String]) -> i32 {
    const WIDTH: i16 = 101;
    const HEIGHT: i16 = 103;
    const WIDTH_REQUIRED: usize = 20;
    let debug = false;

    let mut robots = parse_robots(input);
    let mut moves = 0;
    let cache = Mutex::new(HashMap::new());

    let matrix = Mutex::new(vec![vec![0; WIDTH as usize]; HEIGHT as usize]);

    while !is_christmas_tree(&matrix.lock().unwrap(), WIDTH_REQUIRED) {
        //reset
        matrix.lock().unwrap().iter_mut().for_each(|row| row.iter_mut().for_each(|cell| *cell = 0));

        //update position with cache
        robots.par_iter_mut().for_each(|robot| {
            let key = (robot.position, robot.speed);

            //reuse cache if possible
            let new_position = *cache.lock().unwrap().entry(key).or_insert_with(|| {
                let (x, y) = robot.position;
                let (dx, dy) = robot.speed;
                (
                    (x + dx).rem_euclid(WIDTH),
                    (y + dy).rem_euclid(HEIGHT),
                )
            });
            

            robot.position = new_position;

            //update
            let (x, y) = robot.position;
            matrix.lock().unwrap()[y as usize][x as usize] += 1;
        });

        moves += 1;
        if moves % 10000 == 0 {
            println!("Moves: {}", moves);
        }
    }

    //print matrix
    if debug {
        for row in matrix.lock().unwrap().iter() {
            for cell in row.iter() {
                if *cell > 0 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    moves
}


fn parse_robots(input: &[String]) -> Vec<Robot> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    input
        .iter()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Robot::new(
                (
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                ),
                (
                    caps[3].parse().unwrap(),
                    caps[4].parse().unwrap(),
                ),
            )
        })
        .collect()
}

fn is_christmas_tree(matrix: &[Vec<i32>], width_required: usize) -> bool {
    //we search for the line that forms over the tree
    let mut full_width_count = 0;

    for row in matrix.iter().filter(|row| row.iter().sum::<i32>() >= width_required as i32) {
        let continuous_count = row.windows(width_required).any(|window| {
            window.iter().all(|&cell| cell > 0)
        });

        if continuous_count {
            full_width_count += 1;
            if full_width_count >= 2 {
                return true;
            }
        }
    }

    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14_part1() {
        let example_input = "p=0,4 v=3,-3
                             p=6,3 v=-1,-3
                             p=10,3 v=-1,2
                             p=2,0 v=2,-1
                             p=0,0 v=1,3
                             p=3,0 v=-2,-2
                             p=7,6 v=-1,-3
                             p=3,0 v=-1,-2
                             p=9,3 v=2,3
                             p=7,3 v=-1,2
                             p=2,4 v=2,-3
                             p=9,5 v=-3,-3";
        let parsed_input = generate_input(example_input);
        assert_eq!(solve_part1_with_dimensions(&parsed_input,11,7), 12); 
    }

}
