use std::{char, collections::HashMap, fs, usize};

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct TurnPoint {
    x: i32,
    y: i32,
    dir: usize,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

const N_DIRS: usize = 4;
const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn in_bounds(x: &i32, y: &i32, w: &i32, h: &i32) -> bool {
    return *x >= 0 && *x < *w && *y >= 0 && *y < *h;
}

fn main() {
    let mut sx: Option<usize> = None;
    let mut sy: Option<usize> = None;
    let file_contents = fs::read_to_string("real.txt").unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    for (y, line) in file_contents.split_whitespace().enumerate() {
        let mut map_line: Vec<char> = line.chars().collect();
        if let Some(pos) = map_line.iter().position(|&ch| ch == '^') {
            sx = Some(pos);
            sy = Some(y);
            map_line[pos] = '.';
        }
        map.push(map_line);
    }

    let sx = sx.unwrap() as i32;
    let sy = sy.unwrap() as i32;
    dbg!(sx);
    dbg!(sy);

    let w = map[0].len() as i32;
    let h = map.len() as i32;
    let mut dir_index = 0;
    let mut x = sx;
    let mut y = sy;
    let mut visited: HashMap<Point, bool> = HashMap::new();
    let mut obstacles: HashMap<Point, bool> = HashMap::new();

    while in_bounds(&x, &y, &w, &h) {
        if map[y as usize][x as usize] == '#' {
            x -= DIRS[dir_index].0;
            y -= DIRS[dir_index].1;
            dir_index = (dir_index + 1) % N_DIRS;
        } else {
            visited.insert(Point { x, y }, true);
        }

        //Position of inserted obstacle: in the path of the moving thingy
        let ox = x + DIRS[dir_index].0;
        let oy = y + DIRS[dir_index].1;

        // First, check if we can place an obstacle there.
        // Skip checking those already checked.
        // We can not place at the guard's starting pos, and not where there already is a wall.
        if in_bounds(&ox, &oy, &w, &h)
            && !obstacles.contains_key(&Point { x: ox, y: oy })
            && !visited.contains_key(&Point { x: ox, y: oy })
            && map[oy as usize][ox as usize] == '.'
            && !(ox == sx && oy == sy)
        {
            //Place the obstacle, to be undone later.
            map[oy as usize][ox as usize] = '#';

            let mut nx = x;
            let mut ny = y;
            let mut ndir_index = (dir_index + 1) % N_DIRS;
            let mut turn_points: HashMap<TurnPoint, bool> = HashMap::new();

            turn_points.insert(
                TurnPoint {
                    x,
                    y,
                    dir: ndir_index,
                },
                true,
            );

            while in_bounds(&nx, &ny, &w, &h) {
                if map[ny as usize][nx as usize] == '#' {
                    nx -= DIRS[ndir_index].0;
                    ny -= DIRS[ndir_index].1;
                    ndir_index = (ndir_index + 1) % N_DIRS;

                    if turn_points.contains_key(&TurnPoint {
                        x: nx,
                        y: ny,
                        dir: ndir_index,
                    }) {
                        // A loop, we have been here before.
                        obstacles.insert(Point { x: ox, y: oy }, true);
                        break;
                    }

                    turn_points.insert(
                        TurnPoint {
                            x: nx,
                            y: ny,
                            dir: ndir_index,
                        },
                        true,
                    );
                }

                nx += DIRS[ndir_index].0;
                ny += DIRS[ndir_index].1;
            }

            // Unplace obstacle
            map[oy as usize][ox as usize] = '.';
        }
        x += DIRS[dir_index].0;
        y += DIRS[dir_index].1;
    }

    dbg!(visited.len());
    dbg!(obstacles.len());
}
