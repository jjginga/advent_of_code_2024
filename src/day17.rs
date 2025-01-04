use aoc_runner_derive::{aoc, aoc_generator};
use std::sync::Arc;
use rayon::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};


#[derive(Clone, Debug)]
pub enum SymExpr{
    A0, //initial value of A
    Lit(u64), //literal int

    //composed expressions
    Shr(Arc<SymExpr>, Arc<SymExpr>), //shift right expr >> n
    Xor(Arc<SymExpr>, Arc<SymExpr>), //xor expr1 ^ expr2
    And(Arc<SymExpr>, Arc<SymExpr>), //and expr1 & expr2
}

impl SymExpr {
    pub fn shr(self, bits: SymExpr) -> Self {
        SymExpr::Shr(Arc::new(self), Arc::new(bits))
    }

    pub fn xor(self, rhs: SymExpr) -> Self {
        SymExpr::Xor(Arc::new(self), Arc::new(rhs))
    }

    pub fn and(self, mask: SymExpr) -> Self {
        SymExpr::And(Arc::new(self), Arc::new(mask))
    }

    fn structural_hash<H: Hasher>(&self, state: &mut H) {
        match self {
            SymExpr::A0 => {
                0u8.hash(state);
            }
            SymExpr::Lit(n) => {
                1u8.hash(state);
                n.hash(state);
            }
            SymExpr::Shr(a, b) => {
                2u8.hash(state);
                a.structural_hash(state);
                b.structural_hash(state);
            }
            SymExpr::Xor(a, b) => {
                4u8.hash(state);
                a.structural_hash(state);
                b.structural_hash(state);
            }
            SymExpr::And(a, b) => {
                5u8.hash(state);
                a.structural_hash(state);
                b.structural_hash(state);
            }
        }
    }
}

impl Hash for SymExpr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.structural_hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct SymState {
    a: SymExpr,
    b: SymExpr,
    c: SymExpr,
}

impl PartialEq for SymState {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for SymState {}

impl PartialOrd for SymState {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl Ord for SymState {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl Hash for SymState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.b.hash(state);
        self.c.hash(state);
    }
}

#[derive(Debug)]
pub struct Constraint {
    pub expr: Arc<SymExpr>, //expression from the output
    pub expected: u64, //expected value (0..7, mod 8)
    //in the end we want a list of Constraints expr_i == expected_i
}

fn heuristic(program: &Vec<u8>, state: &SymState, pointer: usize, out_count: usize) -> usize {
    let expected_output = program[out_count];
    let current_output = match &state.a {
        SymExpr::Lit(value) => *value & 7, 
        _ => 8,
    };

    let output_distance = (current_output as isize - expected_output as isize).abs() as usize;

    (out_count << 16) + output_distance.saturating_add(pointer)
}

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
    
    let (_a_reg, b_reg, c_reg, program) = parse_input(input);

    let constraints = symbolic_execute(&program, b_reg, c_reg);

    if let Some(a0) = solve_constraints(&constraints, &program) {
        return a0;
    }

    0
}

fn decode_operand(state: &SymState, operand: u8) -> SymExpr {
    match operand {
        0..=3 => SymExpr::Lit(operand as u64),
        4 => state.a.clone(),
        5 => state.b.clone(),
        6 => state.c.clone(),
        _ => panic!("Invalid operand"),
    }
}

fn symbolic_execute(program: &Vec<u8>, initial_b: u64, initial_c: u64) -> Vec<Constraint> {
    let initial_state = SymState {
        a: SymExpr::A0,
        b: SymExpr::Lit(initial_b),
        c: SymExpr::Lit(initial_c),
    };

    let mut heap = BinaryHeap::new();
    let score = heuristic(&program, &initial_state, 0, 0);
    heap.push(Reverse((score, 0, 0, initial_state)));
    let mut visited = std::collections::HashSet::new();
    let mut constraints = Vec::new();


    while let Some(Reverse((_, mut pointer, mut out_count, mut state))) = heap.pop() {


        if !visited.insert((pointer, out_count, state.clone())) {
            continue;
        }
        //println!("Processing state: {:?}, Pointer: {}, Out Count: {}", state.a, pointer, out_count);

        if pointer >= program.len() {
            continue;
        }

        //instruction
        let opcode = program[pointer];
        if pointer + 1 >= program.len() {
            break;
        }

        let operand_u8 = program[pointer + 1];

        match opcode {
            0 => { //adv: A = A >> operand
                //if operand is 0..=3 is literal 
                state.a = match operand_u8 {
                    0..=3 => state.a.clone().shr(SymExpr::Lit(operand_u8 as u64)),
                    4 => {
                        // A >>= A
                        state.a.clone().shr(state.a.clone()) 
                    },
                    5 => {
                        // A >>= B
                        state.a.clone().shr(state.b.clone()) 
                    },
                    6 => {
                        // A >>= C
                        state.a.clone().shr(state.c.clone())
                    },
                    _ => panic!("Invalid operand for adv"),
                };
                pointer += 2;
                let new_state = state.clone();
                heap.push(Reverse((heuristic(&program,&new_state, pointer, out_count), pointer, out_count, new_state)));
            },
            1 => { // bxl: B = B ^ literal
                state.b = state.b.clone().xor(SymExpr::Lit(operand_u8 as u64));
                pointer += 2;
                let new_state = state.clone();
                heap.push(Reverse((heuristic(&program,&new_state, pointer, out_count), pointer, out_count, new_state)));
            },
            2 => { // bst: B = operand & 7
                let mask = SymExpr::Lit(7);
                state.b = decode_operand(&state, operand_u8).and(mask);
                pointer += 2;
                let new_state = state.clone();
                heap.push(Reverse((heuristic(&program,&new_state, pointer, out_count), pointer, out_count, new_state)));
            },
            3 => { // jnz: if A != 0, jumps to operand
                match &state.a {
                    SymExpr::Lit(val) => {
                        if *val != 0 {
                            pointer = operand_u8 as usize;
                        } else {
                            pointer += 2;
                        }

                        heap.push(Reverse((heuristic(&program,&state, pointer, out_count), pointer, out_count, state)));
                    },
                    _ => {
                        let state_branch_1 = state.clone();
                        let state_branch_2 = state.clone();

                        //branch 1 assume A!=0
                        heap.push(Reverse((heuristic(&program,&state_branch_1, pointer, out_count), pointer, out_count, state_branch_1)));

                        //branch 2 assume A==0
                        heap.push(Reverse((heuristic(&program,&state_branch_2, pointer, out_count), pointer, out_count, state_branch_2)));
                    },
                }
            },
            4 => { //bxc: B = B ^ C
                state.b = state.b.clone().xor(state.c.clone());
                pointer += 2;
                let new_state = state.clone();
                heap.push(Reverse((heuristic(&program,&new_state, pointer, out_count), pointer, out_count, new_state)));
            },
            5 => { //out: output = (operand_expr & 7)
                let out_expr = decode_operand(&state, operand_u8).and(SymExpr::Lit(7));

                if out_count < program.len() {
                    let expected_val = program[out_count] as u64;
                    constraints.push(Constraint {
                        expr: Arc::new(out_expr),
                        expected: expected_val,
                    });
                    out_count += 1;
                }

                pointer += 2;
                let new_state = state.clone();
                heap.push(Reverse((heuristic(&program,&new_state, pointer, out_count), pointer, out_count, new_state)));
            },
            6 => { // bdv: B = A >> operand
                state.b = match operand_u8 {
                    0..=3 => state.a.clone().shr(SymExpr::Lit(operand_u8 as u64)),
                    4 => state.a.clone().shr(state.a.clone()), // Placeholder
                    5 => state.a.clone().shr(state.b.clone()), // Placeholder
                    6 => state.a.clone().shr(state.c.clone()), // Placeholder
                    _ => panic!("Invalid operand for bdv"),
                };
                pointer += 2;
                let new_state = state.clone();
                heap.push(Reverse((heuristic(&program,&new_state, pointer, out_count), pointer, out_count, new_state)));
            },
            7 => { // cdv: C = A >> operand
                state.c = match operand_u8 {
                    0..=3 => state.a.clone().shr(SymExpr::Lit(operand_u8 as u64)),
                    4 => state.a.clone().shr(state.a.clone()), // Placeholder
                    5 => state.a.clone().shr(state.b.clone()), // Placeholder
                    6 => state.a.clone().shr(state.c.clone()), // Placeholder
                    _ => panic!("Invalid operand for cdv"),
                };
                pointer += 2;
                let new_state = state.clone();
                heap.push(Reverse((heuristic(&program,&new_state, pointer, out_count), pointer, out_count, new_state)));
            },
            _ => { // Halt 
                break;
            },
        }
    }

    println!("Constraints: {:?}", constraints);
    constraints
}

fn eval_expr(expr: &SymExpr, a0: u64) -> u64 {
    match expr {
        SymExpr::A0 => a0,
        SymExpr::Lit(n) => *n,
        SymExpr::Shr(e, bits) => eval_expr(e, a0) >> eval_expr(bits, a0),
        SymExpr::Xor(e1, e2) => eval_expr(e1, a0) ^ eval_expr(e2, a0),
        SymExpr::And(e1, e2) => eval_expr(e1, a0) & eval_expr(e2, a0),
    }
}


fn solve_constraints(constraints: &[Constraint], program: &Vec<u8>) -> Option<u64> {
    let attempt_max = u64::MAX; // 16,777,216
    let (instruction, operand) = parse_instruction(&program, 0); //parse the first instruction and operand
    let program_len = program.len();
    

    //(u32::MAX as u64..=attempt_max).into_par_iter()
    (1..=attempt_max).into_par_iter()
        .find_any(|&candidate_a0| {
            let is_valid_constraints = constraints.iter().try_fold(true, |_, constraint| {
                if eval_expr(&constraint.expr, candidate_a0) != constraint.expected {
                    None 
                } else {
                    Some(true)
                }
            }).is_some();
            
            
            
            if is_valid_constraints {
                let mut output_vec = Vec::new();
                compute(instruction,operand,program,candidate_a0,0,0,0,&mut output_vec);
                
                return output_vec.len() == program_len
                    && output_vec.iter().zip(program).all(|(a, b)| a == b) 
            }
            false
        })
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


    println!("input: {:?}", input);

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
