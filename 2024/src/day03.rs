use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day3)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
    //regex patern 
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    input.into_iter()
         .flat_map(|line| {
            re.captures_iter(line).filter_map(|cap| {
                let num1 = cap[1].parse::<i32>().ok()?;
                let num2 = cap[2].parse::<i32>().ok()?;
                Some(num1*num2)
            })
         })
         .sum()
}



//part 2
#[aoc(day3, part2)]
pub fn solve_part2(input: &[String]) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut skip = false;
    
    re.captures_iter(&input.join(""))
      .filter_map(|cap| {
          let token = &cap[0];
          if token == "don't()" {
              skip = true; 
              None
          } else if token == "do()" {
              skip = false;
              None
          } else if !skip {
              let num1: i32 = cap[1].parse().unwrap(); 
              let num2: i32 = cap[2].parse().unwrap(); 
              Some(num1 * num2) 
          } else {
              None 
          }
      })
      .sum()
}
