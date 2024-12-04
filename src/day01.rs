use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn generate_input(input: &str) -> (Vec<i32>, Vec<i32>) {
        input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip()
}

//part 1
#[aoc(day1, part1)]
pub fn solve_part1(input:&(Vec<i32>, Vec<i32>)) -> i32 {
    let mut col1 = input.0.clone(); 
    let mut col2 = input.1.clone(); 

    col1.sort();
    col2.sort();

    col1.iter()
        .zip(col2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>()
}

//part 2
#[aoc(day1, part2)]
pub fn solve_part2(input:&(Vec<i32>, Vec<i32>)) -> i32 {
    let col1 = input.0.clone();
    let col2 = input.1.clone();

    col1.iter().map(|n|  n*{col2.iter().filter(|&x| x==n).count() as i32}).sum::<i32>()
}