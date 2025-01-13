use aoc_runner_derive::{aoc, aoc_generator};
use phf::phf_map;

const NUMERIC_KEYPAD: phf::Map<char, (u32, u32)> = phf::phf_map! {
    '7' => (0, 0), '8' => (1, 0), '9' => (2, 0),
    '4' => (0, 1), '5' => (1, 1), '6' => (2, 1),
    '1' => (0, 2), '2' => (1, 2), '3' => (2, 2),
                   '0' => (1, 3), 'A' => (2, 3),
};

const ROBOT_KEYPAD: phf::Map<char, (u32, u32)> = phf::phf_map! {
                   '^' => (1, 0), 'A' => (2, 0),
    '<' => (0, 1), 'v' => (1, 1), '>' => (2, 1),
};


#[aoc_generator(day21)]
fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day21, part1)]
fn solve_part1(_input: &[String]) -> u32 {
    let numeric_keypad_start = (2, 3); 
    let robot1_keypad_start = (1, 0);
    let robot2_keypad_start = (1, 0); 

    let keypads = [
        (&NUMERIC_KEYPAD, numeric_keypad_start),
        (&ROBOT_KEYPAD, robot1_keypad_start),
        (&ROBOT_KEYPAD, robot2_keypad_start),
    ];

    let result =compute_full_sequence("029A", &keypads);    
    println!("The full sequence is: {}", result);
    0 
}

#[aoc(day21, part2)]
fn solve_part2(_input: &[String]) -> u32 {
    0
}

fn compute_full_sequence(
    input: &str,
    keypads: &[(&phf::Map<char, (u32, u32)>, (u32, u32))],
) -> String {
    let mut pos = keypads.iter().map(|k| k.1).collect::<Vec<_>>();
    let mut ans = String::new();
    for ch in input.chars() {
        if let Some(seq) = recurse(&mut pos, ch, keypads, 0) {
            ans.push_str(&seq);
        }
    }
    ans
}

fn recurse(
    positions: &mut [(u32, u32)],
    target: char,
    keypads: &[(&phf::Map<char, (u32, u32)>, (u32, u32))],
    level: usize,
) -> Option<String> {
    if level >= keypads.len() {
        return None;
    }
    let (map, _) = keypads[level];
    if let Some(&goal) = map.get(&target) {
        if level == keypads.len() - 1 {
            let route = safe_move(positions[level], goal, map)?;
            positions[level] = goal;
            let mut out = route;
            out.push('A');
            Some(out)
        } else {
            let path = safe_move(positions[level], goal, map)?;
            let mut big = String::new();
            for c in path.chars() {
                if let Some(sub) = recurse(positions, c, keypads, level + 1) {
                    big.push_str(&sub);
                } else {
                    return None;
                }
            }
            if let Some(sub) = recurse(positions, 'A', keypads, level + 1) {
                big.push_str(&sub);
            } else {
                return None;
            }
            positions[level] = goal;
            Some(big)
        }
    } else {
        recurse(positions, target, keypads, level + 1)
    }
}

fn safe_move(
    start: (u32, u32),
    end: (u32, u32),
    map: &phf::Map<char, (u32, u32)>,
) -> Option<String> {
    let mut best = None;
    for &(dy_first, dx_first) in &[(true, false), (false, true)] {
        if let Some(r) = route_2phase(start, end, dy_first, dx_first, map) {
            if best.as_ref().map_or(true, |b: &String| r.len() < b.len()) {
                best = Some(r);
            }
        }
    }
    best
}

fn route_2phase(
    start: (u32, u32),
    end: (u32, u32),
    vertical_first: bool,
    horizontal_first: bool,
    map: &phf::Map<char, (u32, u32)>,
) -> Option<String> {
    let (sx, sy) = (start.0 as i32, start.1 as i32);
    let (ex, ey) = (end.0 as i32, end.1 as i32);
    let (mut x, mut y) = (sx, sy);
    let mut r = String::new();

    let mut moves = Vec::new();
    if vertical_first {
        while y < ey { moves.push('v'); y += 1; }
        while y > ey { moves.push('^'); y -= 1; }
    }
    if horizontal_first {
        while x < ex { moves.push('>'); x += 1; }
        while x > ex { moves.push('<'); x -= 1; }
    }
    if !vertical_first {
        while y < ey { moves.push('v'); y += 1; }
        while y > ey { moves.push('^'); y -= 1; }
    }
    if !horizontal_first {
        while x < ex { moves.push('>'); x += 1; }
        while x > ex { moves.push('<'); x -= 1; }
    }

    for c in moves {
        let mut nx = x;
        let mut ny = y;
        match c {
            '^' => ny -= 1,
            'v' => ny += 1,
            '<' => nx -= 1,
            '>' => nx += 1,
            _ => {}
        }
        if !is_valid(map, nx, ny) {
            return None;
        }
        r.push(c);
        x = nx;
        y = ny;
    }
    Some(r)
}

fn is_valid(
    map: &phf::Map<char, (u32, u32)>,
    xx: i32,
    yy: i32,
) -> bool {
    for (&ch, &(cx, cy)) in map.entries() {
        if cx as i32 == xx && cy as i32 == yy && ch != ' ' {
            return true;
        }
    }
    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day21_part1() {
        let example_input = concat!(
            "029A\n",
            "980A\n",
            "179A\n",
            "456A\n",
            "379A\n"
        );
        let input = generate_input(example_input);
        let result = solve_part1(&input);
        assert_eq!(result, 126384);
    }

    #[test]
    fn test_day21_part2() {
        let example_input = "";
        let input = generate_input(example_input);
        assert_eq!(solve_part2(&input), 0); 
    }
}
