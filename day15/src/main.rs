use std::{collections::HashMap};


#[derive(Clone, Debug, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

fn print_map(map: &[Vec<char>], pos: &Pos) {
    for (y, row) in map.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if pos.x == x as i32 && pos.y == y as i32 {
                print!("{}", "@");
            } else {
                print!("{}",*ch);
            }
        }

        println!();
    }
}

fn count(map: &[Vec<char>]) -> usize {
      map.iter().enumerate().flat_map(|(y, row)|
        row.iter().enumerate().filter(|(_, &ch)| ch == '[').map(move |(x, _)| 100*y+x)).sum()
}

fn read_map(map: &[Vec<char>], pos: &Pos) -> char {
    return map[pos.y as usize][pos.x as usize];
}


fn can_push_vert(map: &mut [Vec<char>], prev_pos: &Pos, pos: &Pos, y_offset: i32, alter_map: bool) -> bool {
    let offset = Pos {
        x: pos.x,
        y: pos.y + y_offset
    };

    let map_coord = read_map(map, &offset);
    let org_map_coord = read_map(map, &pos);
        
    let ret_val = match map_coord {
        '[' => {
            if map_coord != org_map_coord {
                can_push_vert(map, &pos, &Pos {
                    x: pos.x,
                    y: pos.y + y_offset
                }, y_offset, alter_map) && 
                can_push_vert(map, &pos, &Pos {
                    x: pos.x + 1,
                    y: pos.y + y_offset  
                }, y_offset, alter_map) 
            } else {
                can_push_vert(map, &pos, &Pos { x: pos.x, y: pos.y + y_offset}, y_offset, alter_map)
            }
            }
        ']' => {
            if map_coord != org_map_coord {
                can_push_vert(map, &pos, &Pos {
                    x: pos.x,
                    y: pos.y + y_offset
                }, y_offset, alter_map) && 
                can_push_vert(map, &pos, &Pos {
                    x: pos.x - 1,
                    y: pos.y + y_offset  
                }, y_offset, alter_map)
            } else {
                can_push_vert(map,  &pos, &Pos { x: pos.x, y: pos.y + y_offset}, y_offset, alter_map)
            }
        },
        '#' => false,
        '.' => true,
        _ => panic!("Cannot recoginze {}", map_coord)
    };

    if alter_map && prev_pos != pos {

        map[(pos.y + y_offset) as usize][pos.x as usize] = map[pos.y as usize][pos.x as usize];
        map[pos.y as usize][pos.x as usize] = '.';
    }
    ret_val
}

fn map_swap(map: &mut [Vec<char>], a: (i32, i32), b: (i32, i32)) {
    let tmp = map[a.1 as usize][a.0 as usize];
    map[a.1 as usize][a.0 as usize] = map[b.1 as usize][b.0 as usize];
    map[b.1 as usize][b.0 as usize] = tmp;
}
fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let (map_str, move_str) = file.split_once("\n\n").unwrap();

    let map: Vec<Vec<char>> = map_str.lines().map(|line| line.chars().collect()).collect();

    // For task b) blow up the map in the x-dimension.
    let mut map: Vec<Vec<char>> = map.iter().map(|row| row.iter().flat_map(|&ch| {
        match ch  {
         '@' => ['@', '.'],
         'O' => ['[', ']'],
         _ => [ch, ch],
        }             
    }).collect()).collect();




    let moves: Vec<char> = move_str.chars().filter(|&ch| ch != '\n').collect();

    let mut start_pos: Option<Pos> = None;
    for (y,row) in map.iter().enumerate(){
        if let Some(x) = row.iter().position(|&ch| ch == '@') {
            start_pos = Some(Pos {
                x: x as i32,
                y: y as i32
            });
            break;
        }
    }



    let mut pos = start_pos.expect("Start pos not found!");

    // Make start pos available space.
    map[pos.y as usize][pos.x as usize] = '.';
    

    let move_to_offset: HashMap<char, (i32, i32)> = HashMap::from([
        ('<', (-1, 0)),
        ('>', (1, 0)),
        ('^', (0, -1)),
        ('v', (0, 1)),
    ]);

    for mov in moves.iter() {
        let dir = move_to_offset.get(mov).unwrap();

        let new_pos = Pos {
            x: pos.x + dir.0,
            y: pos.y + dir.1
        };
        let map_coord = read_map(&map, &new_pos);

        if map_coord == '#' {
            // Cannot move, is wall.
            continue;
        }
        else if map_coord == '.' {
            // Move into empty slot.
            pos = new_pos;
        }
        else  { // is wall '[' or ']'
            if dir.0 != 0 { // Horizontal movement: push blocks like in task a.
                // Save where the wall starts.
                let initial_wall_pos = new_pos;
                let mut final_wall_pos = initial_wall_pos.clone();
                while read_map(&map, &final_wall_pos) == '[' || read_map(&map, &final_wall_pos) == ']' {
                    final_wall_pos.x += dir.0;
                    final_wall_pos.y += dir.1;
                }

                // The wall can be moved if the space beyond the wall is free.
                if read_map(&map, &final_wall_pos) == '.' {

                    // Iterate from end-of-wall to start-of-wall, shifting each tile in the direction of motion.
                    while final_wall_pos != initial_wall_pos {
                        let prev_wall_pos = final_wall_pos.clone();
                        final_wall_pos.x -= dir.0;
                        final_wall_pos.y -= dir.1;

                        // Manually swap final_wall_pos and prev_wall_pos tiles.
                        map_swap(&mut map, (prev_wall_pos.x, prev_wall_pos.y), (final_wall_pos.x, final_wall_pos.y));
                    }  
                    // Make last tile available.
                    map[initial_wall_pos.y as usize][initial_wall_pos.x as usize] = '.';

                    // Lastly, move robot into place of where the wall was pushed out of.
                    pos = initial_wall_pos;
                }
            } else { // vertical movement. now it gets tricky.
                if can_push_vert(&mut map, &pos, &pos, dir.1, false) {
                    // Then, make the moves.
                    can_push_vert(&mut map, &pos, &pos, dir.1, true);
                    // .. and advance robot's position.
                    pos = new_pos; 
                }
            }

        }
    }

    print_map(&map, &pos);
    dbg!(count(&map));
}
