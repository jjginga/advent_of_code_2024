use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;


struct ClawMachine {
    button_a: (i64, i64),
    price_a: i64,
    button_b: (i64, i64),
    price_b: i64,
    prize: (i64, i64),
}

impl ClawMachine {
    pub fn new(button_a: (i64, i64), price_a: i64, button_b: (i64, i64), price_b: i64, prize: (i64, i64)) -> Self {
        ClawMachine {
            button_a,
            price_a,
            button_b,
            price_b,
            prize,
        }
    }

    pub fn calculate_price(&self) -> Option<i64> {
        let (ax, ay) = self.button_a;
        let (bx, by) = self.button_b;
        let (px, py) = self.prize;

        let determinant = (ax * by) - (ay * bx);

        let di = (px * by) - (py * bx);
        let dj = (py * ax) - (px * ay);

        if di % determinant != 0 || dj % determinant != 0 {
            return None;
        }

        let cost_a = di / determinant;
        let cost_b = dj / determinant;

        //println!("Cost A: {}, Cost B: {}", cost_a, cost_b);
        Some((cost_a * self.price_a) + (cost_b * self.price_b))
       
    }
}



#[aoc_generator(day13)]
pub fn generate_input(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|line| line.trim())
                .collect::<Vec<_>>()
                .join("\n") 
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[String]) -> i64 {
    const A_COST: i64 = 3;
    const B_COST: i64 = 1;

    parse_machines(input, A_COST, B_COST)
        .iter()
        .filter_map(|machine| machine.calculate_price())
        .sum()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[String]) -> i64 {
    const A_COST: i64 = 3;
    const B_COST: i64 = 1;
    const INCREMENT: i64 = 10000000000000;

    parse_machines(input, A_COST, B_COST)
        .iter()
        .map(|machine| ClawMachine::new(
            machine.button_a,
            machine.price_a,
            machine.button_b,
            machine.price_b,
            (machine.prize.0 + INCREMENT, machine.prize.1 + INCREMENT),
        ))
        .filter_map(|machine| machine.calculate_price())
        .sum::<i64>()
}

fn parse_machines(input: &[String], a_cost: i64, b_cost: i64) -> Vec<ClawMachine> {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();


    input
        .iter()
        .filter_map(|chunk| {
            let captures = re.captures(chunk)?;
            Some(ClawMachine::new(
                (
                    captures[1].parse().unwrap(),
                    captures[2].parse().unwrap(),
                ),
                a_cost,
                (
                    captures[3].parse().unwrap(),
                    captures[4].parse().unwrap(),
                ),
                b_cost, 
                (
                    captures[5].parse().unwrap(),
                    captures[6].parse().unwrap(),
                ),
            ))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
        fn test_day13_part1() {
            let puzzle = "Button A: X+94, Y+34
                          Button B: X+22, Y+67
                          Prize: X=8400, Y=5400

                          Button A: X+26, Y+66
                          Button B: X+67, Y+21
                          Prize: X=12748, Y=12176

                          Button A: X+17, Y+86
                          Button B: X+84, Y+37
                          Prize: X=7870, Y=6450

                          Button A: X+69, Y+23
                          Button B: X+27, Y+71
                          Prize: X=18641, Y=10279";
            let input = generate_input(puzzle);
            assert_eq!(solve_part1(&input), 480);
        }
      
    }