use aoc_runner_derive::{aoc, aoc_generator};


#[aoc_generator(day4)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
    
    let count_horizontal = |line: &str| {
        line.match_indices("XMAS")
            .chain(line.match_indices("SAMX"))
            .count()
    };

    let transpose = |matrix: &[Vec<char>]| -> Vec<String> {
        let rows = matrix.len();
        let cols = matrix[0].len();
        (0..cols)
            .map(|col| (0..rows).map(|row| matrix[row][col]).collect::<String>())
            .collect()
    };
    
    let diagonals = |matrix: &[Vec<char>]| -> Vec<String> {
        let rows = matrix.len();
        let cols = matrix[0].len();
    
        let collect_diag = |start_row: usize, start_col: usize, row_inc: isize, col_inc: isize| {
            (0..)
                .scan((start_row as isize, start_col as isize), |(row, col), _| {
                    if *row >= 0 && *col >= 0 && *row < rows as isize && *col < cols as isize {
                        let ch = matrix[*row as usize][*col as usize];
                        *row += row_inc;
                        *col += col_inc;
                        Some(ch)
                    } else {
                        None
                    }
                })
                .collect::<String>()
        };
    
        let mut result: Vec<String> = (0..rows)
            .map(|start| collect_diag(start, 0, 1, 1))
            .chain((1..cols).map(|start| collect_diag(0, start, 1, 1)))
            .collect();
    
        result.extend(
            (0..rows)
                .map(|start| collect_diag(start, cols - 1, 1, -1))
                .chain((1..cols).map(|start| collect_diag(0, cols - 1 - start, 1, -1))),
        );
    
        result
    };

    let matrix: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    
    matrix.iter()
         .map(|line| count_horizontal(&line.iter().collect::<String>()))
         .chain(transpose(&matrix).iter().map(|line| count_horizontal(line)))
         .chain(diagonals(&matrix).iter().map(|line| count_horizontal(line)))
         .map(|count| count as i32)
         .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[String]) -> i32 {
    let matrix: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    let is_x_mas = |matrix: &[Vec<char>], i: usize, j: usize| -> bool {

        let diag1 = [matrix[i - 1][j - 1], matrix[i][j], matrix[i + 1][j + 1]];
        let diag2 = [matrix[i + 1][j - 1], matrix[i][j], matrix[i - 1][j + 1]];

        (diag1 == ['M', 'A', 'S'] || diag1 == ['S', 'A', 'M'])
            && (diag2 == ['M', 'A', 'S'] || diag2 == ['S', 'A', 'M'])

    };

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut count = 0;

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if is_x_mas(&matrix, i, j) {
                count += 1;
            }
        }
    }

    count
}
