use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|section| section.to_string())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[String]) -> i32 {

    let mut sections = input.iter();

    //tuples
    let tuples: Vec<(i32, i32)> = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    //lists
    let lists: Vec<Vec<i32>> = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line
                .split(',')
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    lists
        .into_iter()
        .filter(|list| {
            tuples.iter().all(|(a, b)| {
                let pos_a = list.iter().position(|&x| x == *a);
                let pos_b = list.iter().position(|&x| x == *b);

                match (pos_a, pos_b) {
                    (Some(pos_a), Some(pos_b)) => pos_a < pos_b,
                    _ => true,
                }
            })
        })
        .filter_map(|list| list.iter().nth(list.len() / 2).cloned())
        .sum::<i32>()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[String]) -> i32 {
    let mut sections = input.iter();

    //tuples
    let tuples: Vec<(i32, i32)> = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    //lists
    let lists: Vec<Vec<i32>> = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line
                .split(',')
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    lists
        .into_iter()
        .filter(|list| {
            !tuples.iter().all(|(a, b)| {
                let pos_a = list.iter().position(|&x| x == *a);
                let pos_b = list.iter().position(|&x| x == *b);

                match (pos_a, pos_b) {
                    (Some(pos_a), Some(pos_b)) => pos_a < pos_b,
                    _ => true,
                }
            })
        })
        .map(|list| {
            let mut sorted = list.clone();
            sorted.sort_by(|a, b| {
                if tuples.iter().any(|&(x, y)| x == *a && y == *b) {
                    std::cmp::Ordering::Less
                } else if tuples.iter().any(|&(x, y)| x == *b && y == *a) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            sorted
        })
        .filter_map(|list| list.iter().nth(list.len() / 2).cloned())
        .sum::<i32>()

}