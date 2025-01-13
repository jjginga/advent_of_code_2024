use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::collections::VecDeque;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: u16,
    pub y: u16,
}

impl Pos {
    pub fn new(x: u16, y: u16) -> Self {
        Pos { x, y }
    }

    pub fn move_by(&self, dx: i16, dy: i16) -> Option<Pos> {
        let new_x = (self.x as i16 + dx).try_into().ok()?;
        let new_y = (self.y as i16 + dy).try_into().ok()?;
        Some(Pos::new(new_x, new_y))
    }

    pub fn gps_coordinate(&self) -> u32 {
        (self.y as u32) * 100 + (self.x as u32)
    }

}

#[derive(Debug,Clone)]
pub enum EntityType {
    Robot(Robot),
    StorageBox(Pos),
}

#[derive(Debug, Clone)]
pub struct Robot {
    pub position: Pos,
}

impl Robot {
    pub fn new(position: Pos) -> Self {
        Robot { position }
    }

    pub fn attempt_move(
        &mut self,
        direction: (i16, i16),
        grid: &Vec<Vec<char>>,
        entities: &mut HashMap<Pos, EntityType>,
    ) -> bool {
        let Some(new_robot_pos) = self.position.move_by(direction.0, direction.1) else {
            //out of bounds
            return false;
        };
    
        //wall
        if grid.get(new_robot_pos.y as usize)
               .and_then(|row| row.get(new_robot_pos.x as usize))
               == Some(&'#') 
        {
            return false;
        }
    
        if let Some(EntityType::StorageBox(_)) = entities.get(&new_robot_pos) {
            //a box is in the way
            let mut box_chain = Vec::new();
            let mut current_pos = new_robot_pos;
    
            //chain push the boxe
            loop {
                box_chain.push(current_pos);
    
                
                let Some(next_pos) = current_pos.move_by(direction.0, direction.1) else {
                    return false;
                };
    
                //if wall 
                if grid.get(next_pos.y as usize)
                       .and_then(|row| row.get(next_pos.x as usize))
                       == Some(&'#')
                {
                    return false;
                }
    
                //if box
                if let Some(EntityType::StorageBox(_)) = entities.get(&next_pos) {
                    current_pos = next_pos;
                } else {
                    for &from in box_chain.iter().rev() {
                        let push_to = from.move_by(direction.0, direction.1).unwrap();
                        entities.remove(&from);
                        entities.insert(push_to, EntityType::StorageBox(push_to));
                    }
    
                    self.position = new_robot_pos;
                    return true;
                }
            }
        } else {
            self.position = new_robot_pos;
            return true;
        }
    }

    pub fn attempt_move_2(
        &mut self,
        direction: (i16, i16),
        grid: &Vec<Vec<char>>,
        entities: &mut HashMap<Pos, EntityType>,
    ) -> bool {

        //outside the grid
        let Some(new_robot_pos) = self.position.move_by(direction.0, direction.1) else {
            return false;
        };
    
        //wall
        if grid
            .get(new_robot_pos.y as usize)
            .and_then(|row| row.get(new_robot_pos.x as usize))
            == Some(&'#')
        {
            return false;
        }

        //if the robot is pushing the box secondary position
        let secondary_pos = Pos::new(new_robot_pos.x.saturating_sub(1), new_robot_pos.y);
    
        //box in the way
        if let Some(EntityType::StorageBox(_primary_pos)) = entities.get(&new_robot_pos)
            .or_else(|| entities.get(&secondary_pos)) {
                
            let mut box_chain = Vec::new();
            let mut queue = VecDeque::new();

            if let Some(EntityType::StorageBox(primary_pos)) = entities.get(&new_robot_pos).or_else(|| entities.get(&secondary_pos)){
                queue.push_back(*primary_pos);
            } else {
                return false;
            };

            while let Some(current_primary) = queue.pop_front() {
                let Some(next_pos) = current_primary.move_by(direction.0, direction.1) else {
                    return false;
                };
            
                let primary_target_free = grid
                    .get(next_pos.y as usize)
                    .and_then(|row| row.get(next_pos.x as usize))
                    != Some(&'#');
                let secondary_target_free = grid
                    .get(next_pos.y as usize)
                    .and_then(|row| row.get(next_pos.x as usize + 1))
                    != Some(&'#');
            
                if !primary_target_free || !secondary_target_free {
                    return false;
                }
            
                box_chain.push(current_primary);
            
                if direction != (1, 0) {
                    if let Some(EntityType::StorageBox(primary_pos)) = entities.get(&next_pos) {
                        if !box_chain.contains(primary_pos) {
                            queue.push_back(*primary_pos);
                            box_chain.push(*primary_pos);
                        }
                    } else if let Some(EntityType::StorageBox(primary_pos)) = entities.get(&Pos::new(next_pos.x.saturating_sub(1), next_pos.y)) {
                        if !box_chain.contains(primary_pos) {
                            queue.push_back(*primary_pos);
                            box_chain.push(*primary_pos);
                        }
                    }
                }
                if direction != (-1, 0) {
                    if let Some(EntityType::StorageBox(primary_pos)) = entities.get(&Pos::new(next_pos.x + 1, next_pos.y)) {
                        if !box_chain.contains(primary_pos) {
                            queue.push_back(*primary_pos);
                            box_chain.push(*primary_pos);
                        }
                    } else if let Some(EntityType::StorageBox(primary_pos)) = entities.get(&next_pos) {
                        if !box_chain.contains(primary_pos) {
                            queue.push_back(*primary_pos);
                            box_chain.push(*primary_pos);
                        }
                    }
                }
            }
    
            for &from in box_chain.iter().rev() {
                let new_box_pos = from.move_by(direction.0, direction.1).unwrap();
                entities.remove(&from);
                entities.insert(new_box_pos, EntityType::StorageBox(new_box_pos));
            }
    
            self.position = new_robot_pos;
            return true;
        }
    
        self.position = new_robot_pos;
        true
    }
}

#[derive(Debug)]
pub struct Warehouse {
    pub grid: Vec<Vec<char>>,
    pub entities: HashMap<Pos, EntityType>,
    robot_pos: Pos,
}

impl Warehouse {
    pub fn new(grid: Vec<Vec<char>>, entities: HashMap<Pos, EntityType>) -> Self {
        let robot_pos = entities
            .iter()
            .find_map(|(pos, entity)| match entity {
                EntityType::Robot(_) => Some(*pos),
                _ => None,
            })
            .unwrap();
        Warehouse {
            grid,
            entities,
            robot_pos,
        }
    }

    pub fn simulate(&mut self, movements: Vec<(i16, i16)>) {
        for movement in movements {
            if let Some(EntityType::Robot(mut robot)) = self.entities.remove(&self.robot_pos) {
                let movement_result = robot.attempt_move(movement, &self.grid, &mut self.entities);
                if movement_result {
                    self.robot_pos = robot.position;
                }
    
                self.entities.insert(self.robot_pos, EntityType::Robot(robot));
            }
        }
    }

    pub fn simulate_2(&mut self, movements: Vec<(i16, i16)>) {
        for m in movements {
            if let Some(EntityType::Robot(mut r)) = self.entities.remove(&self.robot_pos) {
                let ok = r.attempt_move_2(m, &self.grid, &mut self.entities);
                if ok { self.robot_pos = r.position; }
                self.entities.insert(self.robot_pos, EntityType::Robot(r));
            }
            //print_warehouse_2(&self);
        }
    }
}


#[aoc_generator(day15)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.split("\n\n").map(|s| s.to_string()).collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[String]) -> u32 {
    let (mut warehouse, movements) = parse_warehouse(input);
    warehouse.simulate(movements);
    // Print warehouse
    for (y, row) in warehouse.grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            let pos = Pos::new(x as u16, y as u16);
            if let Some(entity) = warehouse.entities.get(&pos) {
                match entity {
                    EntityType::Robot(_) => print!("@"),
                    EntityType::StorageBox(_) => print!("O"),
                }
            } else {
                print!("{}", ch);
            }
        }
        println!();
    }
    warehouse
        .entities
        .values()
        .filter_map(|entity| match entity {
            EntityType::StorageBox(pos) => {
                Some(pos.gps_coordinate())
            }
            _ => None,
        })
        .sum()
}

#[aoc(day15, part2)]
pub fn solve_part2(input:&[String]) -> u32 {
    let (mut w, moves) = parse_warehouse_part2(input);
    print_warehouse_2(&w);
    w.simulate_2(moves);
    print_warehouse_2(&w);
    w.entities.values().filter_map(|e| {
        match e {
            EntityType::StorageBox(pos) => {
                let x = pos.x;
                let y = pos.y;
                Some(y as u32 * 100 + x as u32)
            }
            _ => None,
        }
    }).sum()   
}
pub fn print_warehouse_2(warehouse: &Warehouse) {
    let mut final_map = warehouse.grid.clone();
    for (p, e) in &warehouse.entities {
        match e {
            EntityType::Robot(_) => {
                if p.x as usize + 1 < final_map[p.y as usize].len() {
                    final_map[p.y as usize][p.x as usize] = '@';
                } else {
                    final_map[p.y as usize][p.x as usize] = '@';
                }
            }
            EntityType::StorageBox(_) => {
                if p.x as usize + 1 < final_map[p.y as usize].len() {
                    final_map[p.y as usize][p.x as usize] = '[';
                    final_map[p.y as usize][p.x as usize + 1] = ']';
                } else {
                    final_map[p.y as usize][p.x as usize] = 'X';
                }
            }
        }
    }
    for row in final_map {
        println!("{}", row.iter().collect::<String>());
    }
}

pub fn parse_warehouse(parts: &[String]) -> (Warehouse, Vec<(i16, i16)>) {
    let mut grid = Vec::new();
    let mut entities = HashMap::new();
    let mut movements = Vec::new();
    //split the map from the movements
    let map_lines = parts[0].lines();
    
    let directions = parts[1].lines().collect::<String>();//all movements in a single string

    //parse the map
    for (y, line) in map_lines.enumerate() {
        let mut row = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => row.push('#'),
                '.' => row.push('.'),
                '@' => {
                    row.push('.');
                    let robot_pos = Pos::new(x as u16, y as u16);
                    entities.insert(robot_pos, EntityType::Robot(Robot::new(robot_pos)));
                }
                'O' => {
                    row.push('.');
                    entities.insert(Pos::new(x as u16, y as u16), EntityType::StorageBox(Pos::new(x as u16, y as u16)));
                }
                _ => (),
            }
        }
        grid.push(row);
    }


    //parse the movements 
    for ch in directions.chars() {
        let movement = match ch {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _   => (0,0),
        };
        movements.push(movement);
    }

    (Warehouse::new(grid, entities), movements)
}

pub fn parse_warehouse_part2(parts: &[String]) -> (Warehouse, Vec<(i16, i16)>) {
    let (original_w, moves) = parse_warehouse(parts);
    let mut new_grid = Vec::new();
    let mut new_entities = HashMap::new();
    for row in original_w.grid {
        let mut expanded_row = Vec::new();
        for c in row {
            if c == '#' { expanded_row.push('#'); expanded_row.push('#'); }
            else if c == '.' { expanded_row.push('.'); expanded_row.push('.'); }
            else if c == '@' { expanded_row.push('@'); expanded_row.push('.'); }
            else if c == 'O' { expanded_row.push('['); expanded_row.push(']'); }
            else { expanded_row.push(c); }
        }
        new_grid.push(expanded_row);
    }
    for (p, e) in original_w.entities {
        match e {
            EntityType::Robot(_) => {
                let nx = p.x * 2;
                let ny = p.y;
                new_entities.insert(Pos::new(nx, ny), EntityType::Robot(Robot::new(Pos::new(nx, ny))));
            }
            EntityType::StorageBox(_) => {
                let nx = p.x * 2;
                let ny = p.y;
                new_entities.insert(Pos::new(nx, ny), EntityType::StorageBox(Pos::new(nx, ny)));
            }
        }
    }
    (Warehouse::new(new_grid, new_entities), moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_part1() {
        let example_input = concat!("##########\n",
                            "#..O..O.O#\n",
                            "#......O.#\n",
                            "#.OO..O.O#\n",
                            "#..O@..O.#\n",
                            "#O#..O...#\n",
                            "#O..O..O.#\n",
                            "#.OO.O.OO#\n",
                            "#....O...#\n",
                            "##########\n",
                            "\n",
                            "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n",
                            "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n",
                            "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n",
                            "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n",
                            "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n",
                            "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n",
                            ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n",
                            "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n",
                            "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n",
                            "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^");
        let parsed_input = generate_input(example_input);
        let res = solve_part1(&parsed_input);
        println!("Result: {}", res);
        assert_eq!(res, 10092);
    }

    #[test]
    fn test_day15_part2() {
        let example_input = concat!("##########\n",
                            "#..O..O.O#\n",
                            "#......O.#\n",
                            "#.OO..O.O#\n",
                            "#..O@..O.#\n",
                            "#O#..O...#\n",
                            "#O..O..O.#\n",
                            "#.OO.O.OO#\n",
                            "#....O...#\n",
                            "##########\n",
                            "\n",
                            "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n",
                            "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n",
                            "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n",
                            "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n",
                            "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n",
                            "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n",
                            ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n",
                            "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n",
                            "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n",
                            "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^");
        let parsed_input = generate_input(example_input);
        assert_eq!(solve_part2(&parsed_input), 9021); 
    }
    #[test]
    fn test_day15_minimal() {
        let example_input = concat!(
            "#######\n",
            "#...#.#\n",
            "#.....#\n",
            "#..OO@#\n",
            "#..O..#\n",
            "#.....#\n",
            "#######\n",
            "\n",
            "<vv<<^^<<^^"
        );
        let parsed_input = generate_input(example_input);
        assert_eq!(solve_part2(&parsed_input), 105);
    }
}
