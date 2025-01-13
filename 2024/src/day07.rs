use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[String]) -> u64 {
    input.iter()
         .map(|line| {
            let (first, rest) = line.split_once(":").unwrap();
            let number = first.trim().parse::<u64>().unwrap();
            let vec = rest.split_whitespace()
                          .map(|x| x.parse::<u64>().unwrap())
                          .collect::<Vec<u64>>();
            (number, vec)
         })
         .filter_map(|(number, vec)| {
            if can_calculate(number, &vec) {
                Some(number)
            } else {
                None
            }
         })
         .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[String]) -> u64 {
    
    input.iter()
         .map(|line| {
            let (first, rest) = line.split_once(":").unwrap();
            let number = first.trim().parse::<u64>().unwrap();
            let vec = rest.split_whitespace()
                          .map(|x| x.parse::<u64>().unwrap())
                          .collect::<Vec<u64>>();
            (number, vec)
         })
         .filter_map(|(number, vec)| {
            if can_calculate(number, &vec) {
                Some(number)
            } else {
                if can_calculate_concat(number, &vec) {
                    Some(number)
                } else {
                    None
                }
            }
         })
         .sum::<u64>()
    
}


fn can_calculate(value: u64, vec: &[u64]) -> bool {
    
    if vec.len() == 1 {
       return vec[0] == value;
    } 

    let num = vec[vec.len()-1];
    let mut result = false;

    if value % num == 0 {
        result |= can_calculate(value/num, &vec[..vec.len()-1])
    } 
    
    if value > num {
        result |= can_calculate(value-num, &vec[..vec.len()-1])
    }

    result
}

fn can_calculate_concat(value: u64, vec: &[u64]) -> bool {
    if vec.len() == 1 {
        return vec[0] == value;
    }

    let num = vec[vec.len() - 1];
    let mut result = false;

    //division
    if value % num == 0 {
        result |= can_calculate_concat(value / num, &vec[..vec.len() - 1]);
    }

    if value > num {
        result |= can_calculate_concat(value - num, &vec[..vec.len() - 1]);
    }

    //concatenation
    result |= desconcatenate_and_check(value, num, &vec[..vec.len() - 1]);

    result
}

fn desconcatenate_and_check(value: u64, num: u64, vec: &[u64]) -> bool {
    let num_digits = ((num as f64).log10().floor() as u32) + 1;

    if value >= num && value % 10u64.pow(num_digits) == num {
        let value2 = value / 10u64.pow(num_digits);
        return can_calculate_concat(value2, vec);  
    }

    false
}