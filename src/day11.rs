use aoc_runner_derive::{aoc, aoc_generator};
use dashmap::DashMap;


#[aoc_generator(day11)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    let mut stones = input[0].split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    for _ in 0..25 {
        stones = stones.iter().flat_map(|x| 
            if *x == 0 {
                vec![process_zero(*x)]
            } else if is_even_digits(*x) {
                let (left, right) = split_stones(*x);
                vec![left, right]
            } else {
                vec![process_oder(*x)]
            })
        .collect();
    }
    stones.len()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[String]) -> usize {
    let initial_stones: Vec<u64> = input[0]
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let cache: DashMap<(u64, usize), usize> = DashMap::new();
    let total_blinks = 75;

    let total_length: usize = initial_stones
        .into_iter()
        .map(|stone| count_stones(stone, 0, total_blinks, &cache))
        .sum();

    total_length
}

fn count_stones(
    stone: u64,
    current_blink: usize,
    total_blinks: usize,
    cache: &DashMap<(u64, usize), usize>,
) -> usize {
    if current_blink >= total_blinks {
        return 1;
    }

    //check cache 
    if let Some(cached_count) = cache.get(&(stone, current_blink)) {
        return *cached_count;
    }

    let count = if stone == 0 {
        count_stones(1, current_blink + 1, total_blinks, cache)
    } else if !is_even_digits(stone) {
        count_stones(stone * 2024, current_blink + 1, total_blinks, cache)
    } else {
        let (left, right) = split_stones(stone);
        let left_count = count_stones(left, current_blink + 1, total_blinks, cache);
        let right_count = count_stones(right, current_blink + 1, total_blinks, cache);
        left_count + right_count
    };

    //update cache
    cache.insert((stone, current_blink), count);

    count
}


fn process_zero(stone: u64) -> u64 {
    stone + 1
}

fn number_digits(stone: u64) -> u64 {
    if stone != 0 {
        (stone as f64).log10().floor() as u64 + 1
    } else {
        1
    }
}
fn is_even_digits(stone: u64) -> bool {
    number_digits(stone) % 2 == 0
}

fn split_stones(stone: u64) -> (u64, u64) {
    let number_digits = number_digits(stone);
    let half = number_digits / 2;
    let divisor = 10u64.pow(half as u32);

    (stone/divisor, stone%divisor)
}

fn process_oder(stone: u64) -> u64 {
    stone * 2024
}