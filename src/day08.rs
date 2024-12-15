use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day8)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[String]) -> usize {

    //create map
    let map = create_map(input);
    let mut antinodes: HashSet<(i16, i16)> = HashSet::new();
    //distinct chars
    let frequencies: HashSet<char> = map.values().copied().collect();
    
    let min_x = map.keys().map(|&(x, _)| x).min().unwrap();
    let max_x = map.keys().map(|&(x, _)| x).max().unwrap();
    let min_y = map.keys().map(|&(_, y)| y).min().unwrap();
    let max_y = map.keys().map(|&(_, y)| y).max().unwrap();

    //iterate each char
    for &frequency in &frequencies{

        //get all positions with this frequency
        let antennas: Vec<_> = map.iter()
                                  .filter_map(|(&pos, &ch)| if ch == frequency  && ch != '.' { Some(pos) } else { None })
                                  .collect();
        
        for i in 0..antennas.len(){
            for j in i + 1..antennas.len(){
                let a1 = antennas[i];
                let a2 = antennas[j];

                //calculate the distance between antennas
                let dx = a2.0 - a1.0;
                let dy = a2.1 - a1.1;
                    
                    
                if dx == 0 && dy == 0 {
                    continue;
                }
                //direction vector
                let gcd = gcd(dx.abs() as i16, dy.abs() as i16);
                let ux = dx / gcd;
                let uy = dy / gcd;
                
                //antinodes: one beyond a1 (oposite to a2) other beyond a2
                let antinode1 = (a1.0 - ux, a1.1 - uy);
                let antinode2 = (a2.0 + ux, a2.1 + uy);
                
                //check limit and add node
                if antinode1.0 >= min_x && antinode1.0 <= max_x && antinode1.1 >= min_y && antinode1.1 <= max_y {
                    antinodes.insert(antinode1);
                }
                if antinode2.0 >= min_x && antinode2.0 <= max_x && antinode2.1 >= min_y && antinode2.1 <= max_y {
                    antinodes.insert(antinode2);
                }
            }

        }
        
            

    }

    //print antinodes each char in it's position
    // Print the map
    for y in 0..=max_y {
        for x in 0..=max_x {
            if antinodes.contains(&(y, x)) {
                print!("#");
            } else if let Some(&ch) = map.get(&(y, x)) {
                print!("{}", ch);
            } else {
                print!(".");
            }
        }
        println!();
    }

    //count 
    antinodes.len()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[String]) -> usize {
    //create map
    let map = create_map(input);
    let mut antinodes: HashSet<(i16, i16)> = HashSet::new();
    //distinct chars
    let frequencies: HashSet<char> = map.values().copied().collect();
    
    let min_x = map.keys().map(|&(x, _)| x).min().unwrap();
    let max_x = map.keys().map(|&(x, _)| x).max().unwrap();
    let min_y = map.keys().map(|&(_, y)| y).min().unwrap();
    let max_y = map.keys().map(|&(_, y)| y).max().unwrap();

    //iterate each char
    for &frequency in &frequencies{

        //get all positions with this frequency
        let antennas: Vec<_> = map.iter()
                                  .filter_map(|(&pos, &ch)| if ch == frequency  && ch != '.' { Some(pos) } else { None })
                                  .collect();
        
        for i in 0..antennas.len(){
            for j in i + 1..antennas.len(){
                
                let a1 = antennas[i];
                let a2 = antennas[j];

                //calculate the distance between antennas
                let dx = a2.0 - a1.0;
                let dy = a2.1 - a1.1;

                //use the gds divisor to normalize the vector
                let gcd = gcd(dx.abs() as i16, dy.abs() as i16);
                let ux = dx / gcd;
                let uy = dy / gcd;

                //extend the line backwards to the edge of the map
                let mut current = (a1.0, a1.1);
                while current.0 >= min_x && current.0 <= max_x && current.1 >= min_y && current.1 <= max_y {
                    antinodes.insert(current);
                    current = (current.0 - ux, current.1 - uy);
                }

                //extend the line forward to the edge of the map
                current = (a2.0, a2.1);
                while current.0 >= min_x && current.0 <= max_x && current.1 >= min_y && current.1 <= max_y {
                    antinodes.insert(current);
                    current = (current.0 + ux, current.1 + uy);

                }
            }
        }
        
            

    }

    //print antinodes each char in it's position
    // Print the map
    for y in 0..=max_y {
        for x in 0..=max_x {
            if antinodes.contains(&(y, x)) {
                print!("#");
            } else if let Some(&ch) = map.get(&(y, x)) {
                print!("{}", ch);
            } else {
                print!(".");
            }
        }
        println!();
    }

    //count 
    antinodes.len()
}

fn create_map(input: &[String]) -> HashMap<(i16, i16), char> {
    let mut map = HashMap::new();
    for (row, line) in input.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            map.insert(((row) as i16, (col) as i16), ch);
        }
    }
    map
}

fn gcd(mut a: i16, mut b: i16) -> i16 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}