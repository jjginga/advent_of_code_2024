use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

//part 1
#[aoc(day2, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    input.iter().map(|line| get_vec(line)).filter(|arr| is_valid(arr)).count()
}

//part 2
#[aoc(day2, part2)]
pub fn solve_part2(input: &[String]) -> usize {
    let mut valids = 0;
    let invalid: Vec<Vec<i32>> = input.iter()
                           .map(|line| get_vec(line))
                           .filter(|arr|{
                               let is_valid_seq = is_valid(arr);
                               valids += if is_valid_seq { 1 } else { 0 };
                               !is_valid_seq
                           })
                           .collect();
    valids + invalid.into_iter()
        .filter_map(|arr| {
            arr.iter()
                .enumerate()
                .find_map(|(i, _)| {
                    let mut modified = arr.clone();
                    modified.remove(i);
                    if is_valid(&modified) {
                        Some(modified)
                    } else {
                        None
                    }
                })
        }).count()
}

fn is_valid(arr: &[i32]) -> bool {
    let is_monotonic = |arr: &[i32]| -> bool {
        arr.windows(2).all(|pair| pair[0] < pair[1])
            || arr.windows(2).all(|pair| pair[0] > pair[1])
    };

    let is_valid_diff = |arr: &[i32]| -> bool {
        arr.windows(2).all(|pair| {
            let diff = (pair[0] - pair[1]).abs();
            diff == 1 || diff == 2 || diff == 3
        })
    };

    is_monotonic(arr) && is_valid_diff(arr)
}


fn get_vec(line: &str) -> Vec<i32> {
    line.split_whitespace()
       .map(|num| num.parse::<i32>().unwrap())
       .collect()
}
