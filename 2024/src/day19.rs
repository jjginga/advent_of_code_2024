use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap};

#[aoc_generator(day19)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    let (patterns, designs) = process_input(input);
    designs.iter().filter(|design| can_construct_design(&patterns, design)).count()
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &[String]) -> usize {
    let (patterns, designs) = process_input(input);
    designs.iter().map(|design| count_ways_to_construct(&patterns, design)).sum()
}

fn count_ways_to_construct(patterns: &[String], design: &str) -> usize {
    fn backtrack(
        patterns: &[String],
        remaining: &str,
        memo: &mut HashMap<String, usize>,
    ) -> usize {
        if remaining.is_empty() {
            return 1;
        }

        if let Some(&cached) = memo.get(remaining) {
            return cached;
        }

        let mut total_ways = 0;
        for pattern in patterns {
            if remaining.starts_with(pattern) {
                total_ways += backtrack(patterns, &remaining[pattern.len()..], memo);
            }
        }

        memo.insert(remaining.to_string(), total_ways);
        total_ways
    }

    let mut memo = HashMap::new();
    backtrack(patterns, design, &mut memo)
}


fn can_construct_design(patterns: &[String], design: &str) -> bool {
    fn backtrack(patterns: &[String], remaining: &str, memo: &mut HashMap<String, bool>) -> bool {

        if remaining.is_empty() {
            return true;
        }

        if let Some(&cached) = memo.get(remaining) {
            return cached;
        }

        for pattern in patterns {
            if remaining.starts_with(pattern) {
                if backtrack(patterns, &remaining[pattern.len()..], memo) {
                    memo.insert(remaining.to_string(), true);
                    return true;
                }
            }
        }
        memo.insert(remaining.to_string(), false);
        false
    }

    let mut memo = HashMap::new();
    backtrack(patterns, design, &mut memo)
}


fn process_input(input: &[String]) -> (Vec<String>, Vec<String>) {
    let mut first_vector = Vec::new();
    let mut second_vector = Vec::new();

    let mut iter = input.iter();
    
    if let Some(first_line) = iter.next() {
        first_vector = first_line.split(',').map(|s| s.trim().to_string()).collect();
    }
    
    iter.next();

    for line in iter {
        second_vector.push(line.to_string());
    }

    (first_vector, second_vector)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
fn test_day19_part1() {
    let example_input = concat!(
        "r, wr, b, g, bwu, rb, gb, br\n",
        "\n",
        "brwrr\n",
        "bggr\n",
        "gbbr\n",
        "rrbgbr\n",
        "ubwu\n",
        "bwurrg\n",
        "brgr\n",
        "bbrgwb\n"
    );
    let parsed_input = generate_input(example_input);
    assert_eq!(solve_part1(&parsed_input), 6);
}


    #[test]
    fn test_day19_part2() {
        let example_input = concat!(
            "r, wr, b, g, bwu, rb, gb, br\n",
            "\n",
            "brwrr\n",
            "bggr\n",
            "gbbr\n",
            "rrbgbr\n",
            "ubwu\n",
            "bwurrg\n",
            "brgr\n",
            "bbrgwb\n"
        );
        let parsed_input = generate_input(example_input);
        assert_eq!(solve_part2(&parsed_input), 16);
    }
}
