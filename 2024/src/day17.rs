use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day17)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &[String]) -> String {
    let (a_reg, b_reg, c_reg, program) = parse_input(input);
    let (instruction, operand) = parse_instruction(&program, 0); //parse the first instruction and operand
    let mut output_vec = Vec::with_capacity(program.len());
    compute(instruction, operand, &program, a_reg, b_reg, c_reg, 0, &mut output_vec)
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",") //join the output vec into a string
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &[String]) -> u64 {
    
    let (_a_reg, _b_reg, _c_reg, program) = parse_input(input);

    reverse_engineer(&program).unwrap_or(0) 
}

fn reverse_engineer(program: &Vec<u8>) -> Option<u64> {
    let mut possible_a = vec![0u64]; 
    let (instruction, operand) = parse_instruction(&program, 0); //parse the first instruction and operand


    //we start by iterating over the program's expected output in reverse order
    //this ensures that the reverse engineering process starts from the final
    for &expected_output in program.iter().rev() {

        //vector to store all possible candidates for register A
        let mut new_possible_a = vec![];

        //now we iterate over the currently known possible values for register A
        //from previous iterations
        for &next_a in &possible_a {
            //each k represents the lower 3 bits of the candidate
            for k in 0..8 {
                //calculate the current value of register A
                //by shifting the previous value left by 3 bits and adding k
                let current_a = (next_a * 8) + k;
             
                //we check if the current value of register A satisfies the program
                //if it does, we add it to the list of possible candidates
                let mut output_vec = Vec::new();
                compute(instruction, operand, &program.to_vec(), current_a, 0, 0, 0, &mut output_vec);

                if output_vec.get(0) == Some(&expected_output) {
                    new_possible_a.push(current_a);
                }

            }
        }

        possible_a = new_possible_a;
    }

    possible_a.into_iter().min()
}


pub fn compute<'a>(
    instruction: u8,
    mut operand: u64,
    program: &Vec<u8>,
    mut a_reg: u64,
    mut b_reg: u64,
    mut c_reg: u64,
    mut pointer: usize,
    output_vec: &'a mut Vec<u8>,
) -> &'a Vec<u8> {

    //operand
    match instruction {
        0 | 2 | 5 | 6 | 7 => {
            operand = match operand {
                0..=3 => operand,
                4 => a_reg,
                5 => b_reg,
                6 => c_reg,
                _ => panic!("Invalid operand"),
            };
        }
        _ => {}
    }
    
    //process instruction
    match instruction {
        //adv: Perform division
        0 => {
            a_reg >>= operand; 
            pointer += 2; 
        }

        // bxl: Perform bitwise XOR
        1 => {
            b_reg ^= operand; // XOR B with the operand
            pointer += 2;
        }

        // bst: Modulo 8 operation and store in B
        2 => {
            b_reg = operand & 7; //keep only the lowest 3 bits
            pointer += 2;
        }

        //jnz
        3 => {
            if a_reg != 0 {
                pointer = operand as usize;
            } else {
                pointer += 2;
            }
        }

        //bxc
        4 => {
            b_reg ^= c_reg; // XOR B with C
            pointer += 2;
        }
        //out
        5 => {
            let output = operand & 7; 
            output_vec.push(output as u8); 
            pointer += 2; 
        }
        //bdv
        6 => {
            b_reg = a_reg >> operand; //divide B by 2^operand
            pointer += 2;
        }
        //cdv
        7 => {
            c_reg = a_reg >> operand; //divide C by 2^operand
            pointer += 2;
        }
        // Halt if an unknown instruction is encountered
        _ => return output_vec,
    }

    if pointer >= program.len() {
        return output_vec;
    }

    let (new_instruction, new_operand) = parse_instruction(&program, pointer); //parse the next instruction and operand

    //continue processing with updated state
    compute(new_instruction, new_operand, program, a_reg, b_reg, c_reg, pointer, output_vec)
}

fn parse_instruction(program: &Vec<u8>, pointer: usize) -> (u8, u64) {
    let new_instruction = program[pointer];
    let operand = program[pointer + 1] as u64;

    (new_instruction, operand)
}

pub fn parse_input(input: &[String]) -> (u64, u64, u64, Vec<u8>) {
    let mut a_reg = 0;
    let mut b_reg = 0;
    let mut c_reg = 0;
    let mut program = Vec::new();

    for line in input {
        if line.starts_with("Register A:") {
            a_reg = line[11..].trim().parse().unwrap();
        } else if line.starts_with("Register B:") {
            b_reg = line[11..].trim().parse().unwrap();
        } else if line.starts_with("Register C:") {
            c_reg = line[11..].trim().parse().unwrap();
        } else if line.starts_with("Program:") {
            program = line[8..]
                .trim()
                .split(',')
                .map(|num| num.trim().parse().unwrap())
                .collect();
        }
    }

    (a_reg, b_reg, c_reg, program)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day17_part1() {
        let example_input = concat!("Register A: 729\n",
            "Register B: 0\n",
            "Register C: 0\n",
            "\n",
            "Program: 0,1,5,4,3,0");
    
        let parsed_input = generate_input(example_input);
        assert_eq!(solve_part1(&parsed_input), "4,6,3,5,6,3,5,2,1,0"); 
    }

    #[test]
    fn test_day17_part2() {
        let example_input = concat!(
            "Register A: 2024\n",
            "Register B: 0\n",
            "Register C: 0\n",
            "\n",
            "Program: 0,3,5,4,3,0"
        );
    
        let parsed_input = generate_input(example_input);
        assert_eq!(solve_part2(&parsed_input), 117440);
    }
}
