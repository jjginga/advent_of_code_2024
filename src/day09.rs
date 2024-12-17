use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[String]) -> i64 {
    //represent disk map has diskmap layout
    let mut disk_layout = extract_disk_layout(&input[0]);
    
    disk_layout = organize_disk(disk_layout);
    calculate_checksum(disk_layout)
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[String]) -> i64 {
    let mut disk_layout = extract_disk_layout(&input[0]);
    disk_layout = organize_disk_by_file(disk_layout);
    calculate_checksum(disk_layout)
}

fn organize_disk(mut disk_layout: Vec<String>) -> Vec<String> {

    //while we have a '.' between the numbers - that is a . with a number on its right
    while disk_layout.windows(2).any(|pair| pair[0] == "." && pair[1].parse::<u32>().is_ok()) {

        //iterate over the disk_layout finding files from right to left
        for i in (0..disk_layout.len()).rev() {
            if let Ok(_) = disk_layout[i].parse::<u32>() {
                //find the first available space from left to right
                if let Some(j) = disk_layout.iter().position(|ch| ch == ".") {
                    //move the file to the first available space
                    disk_layout.swap(i, j);
                    break;
                }
            }
        }
    }

    disk_layout
}

fn organize_disk_by_file(mut disk_layout: Vec<String>) -> Vec<String> {
    //1 - get files (ID, size, starting position), sorted by file ID descending
    let mut files: Vec<(u32, usize, usize)> = Vec::new();
    let mut current_id = None;
    let mut current_size = 0;
    let mut start_pos = 0;

    for (i, block) in disk_layout.iter().enumerate() {
        if let Ok(file_id) = block.parse::<u32>() {
            if current_id == Some(file_id) {
                current_size += 1;
            } else {
                if let Some(id) = current_id {
                    files.push((id, current_size, start_pos));
                }
                current_id = Some(file_id);
                current_size = 1;
                start_pos = i;
            }
        } else {
            if let Some(id) = current_id {
                files.push((id, current_size, start_pos));
            }
            current_id = None;
            current_size = 0;
        }
    }
    if let Some(id) = current_id {
        files.push((id, current_size, start_pos));
    }
    files.sort_by(|a, b| b.0.cmp(&a.0)); //sort by file ID descending

    //2 - get free spaces (size, starting position), sorted by position
    let mut spaces: Vec<(usize, usize)> = Vec::new();
    let mut space_size = 0;
    let mut space_start = 0;

    for (i, block) in disk_layout.iter().enumerate() {
        if block == "." {
            if space_size == 0 {
                space_start = i;
            }
            space_size += 1;
        } else {
            if space_size > 0 {
                spaces.push((space_size, space_start));
                space_size = 0;
            }
        }
    }
    if space_size > 0 {
        spaces.push((space_size, space_start));
    }

    //3 - move files to the leftmost valid space
    for (file_id, file_size, file_pos) in files {
        if let Some((space_index, &(_, space_pos))) = spaces
            .iter()
            .enumerate()
            .find(|&(_, &(space_size, space_pos))| space_size >= file_size && space_pos < file_pos)
        {
            //move the file to the new space
            for i in 0..file_size {
                disk_layout[space_pos + i] = file_id.to_string();
            }
            //clear the original file position
            for i in 0..file_size {
                disk_layout[file_pos + i] = ".".to_string();
            }
            //update the free space list
            let remaining_space = spaces[space_index].0 - file_size;
            if remaining_space > 0 {
                spaces[space_index] = (remaining_space, space_pos + file_size);
            } else {
                spaces.remove(space_index);
            }
        }
    }

    disk_layout
}


fn extract_disk_layout(input: &str) -> Vec<String> {
    let mut prev_num = 0;
    input
        .char_indices()
        .flat_map(|(pos, ch)| {
            ch.to_digit(10).map(|num| {
                if pos % 2 == 0 {
                    //if even it is a file
                    let result = (0..num).map(|_| prev_num.to_string()).collect::<Vec<String>>();
                    prev_num += 1; //increment after adding the file blocks
                    result
                } else {
                    //if odd it is free space
                    vec![".".to_string(); num as usize]
                }
            }).unwrap_or_else(Vec::new)
        })
        .collect()
}

fn calculate_checksum(disk_layout: Vec<String>) -> i64 {
    disk_layout
        .into_iter()
        .enumerate()
        .filter(|(_, c)| *c != ".")
        .map(|(i, c)| i as i64 * c.parse::<u64>().unwrap() as i64)
        .sum()
}
