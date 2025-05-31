use std::{cmp::max, fs, usize};

#[derive(Debug, Clone)]
struct File {
    id: usize,
    size: usize,
}

#[derive(Debug, Clone)]
enum Block {
    File(File),
    FreeSpace(usize),
}

fn find_slot_for(list: &Vec<Block>, file: &File) -> Option<(usize, usize)> {
    for i in 0..list.len() {
        if let Block::FreeSpace(free_space) = list[i] {
            if free_space >= file.size {
                return Some((i, free_space));
            }
        }
    }
    return None;
}

fn taskb(list: &mut Vec<Block>, max_id: usize) {
    for id in (0..max_id + 1).rev() {
        // Get the position of the file in the list.
        let idx = list.iter().position(|block| -> bool {
            if let Block::File(file) = block {
                return file.id == id;
            } else {
                return false;
            }
        });

        let mut file_idx;
        match idx {
            Some(i) => {
                file_idx = i;
            }
            _ => {
                panic!("Could not locate file in list");
            }
        }

        if let Block::File(file) = &list[file_idx].clone() {
            if let Some((free_idx, free_space)) = find_slot_for(list, &file) {
                // Dont move further right.
                if free_idx >= file_idx {
                    continue;
                }

                if free_space > file.size {
                    // Check if next slot is free space, if so, transfer the existing extra white space
                    // to the next. If next slot is not free space, a new must be created.
                    let mut inserted = false;
                    if free_idx < list.len() - 1 {
                        if let Block::FreeSpace(next_free) = list[free_idx + 1] {
                            inserted = true;
                            list[free_idx + 1] =
                                Block::FreeSpace(next_free + free_space - file.size);
                        }
                    }
                    if !inserted {
                        list.insert(free_idx + 1, Block::FreeSpace(free_space - file.size));
                        file_idx += 1; //Offset due to insert.
                    }
                }
                list[free_idx] = Block::File(file.clone());

                let mut new_freespace = file.size;
                //Check if the file's removal results in free space combination
                //Check free space to the right
                if file_idx < list.len() - 1 {
                    if let Block::FreeSpace(fsp) = list[file_idx + 1] {
                        new_freespace += fsp;
                        list.remove(file_idx + 1);
                    }
                }
                //Check free space on the left
                if file_idx > 0 {
                    if let Block::FreeSpace(fsp) = list[file_idx - 1] {
                        new_freespace += fsp;
                        list.remove(file_idx - 1);
                        file_idx -= 1; // Must offset the delete.
                    }
                }
                list[file_idx] = Block::FreeSpace(new_freespace);
            }
        } else {
            panic!("Could not resolve file object.")
        }
    }

    let mut checksum: u64 = 0;
    let mut idx = 0;
    for block in list {
        match block {
            Block::File(file) => {
                for _ in 0..file.size {
                    checksum += (idx * file.id) as u64;
                    idx += 1;
                }
            }
            Block::FreeSpace(free) => {
                idx += *free;
            }
        }
    }

    dbg!(&checksum);
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut list: Vec<Block> = Vec::new();

    let mut id: usize = 0;
    let mut max_id: usize = 0;
    for (i, ch) in input.chars().enumerate() {
        let is_file: bool = i % 2 == 0;
        if ch == '\n' {
            continue;
        }
        let size = ch.to_digit(10).unwrap() as usize;
        if size == 0 {
            continue;
        }
        if is_file {
            list.push(Block::File(File { id, size }));
            max_id = max(id, max_id);
            id += 1;
        } else {
            list.push(Block::FreeSpace(size));
        }
    }

    taskb(&mut list, max_id);
}
