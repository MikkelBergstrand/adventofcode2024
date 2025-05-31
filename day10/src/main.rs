use std::{collections::HashSet, fs, io::BufRead, usize};

#[derive(Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

fn read_tile(map: &Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    return map[y][x];
}

fn get_neighbors(x: usize, y: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = Vec::new();
    if x > 0 {
        ret.push((x - 1, y));
    }
    if x < w - 1 {
        ret.push((x + 1, y));
    }
    if y > 0 {
        ret.push((x, y - 1));
    }
    if y < h - 1 {
        ret.push((x, y + 1));
    }
    return ret;
}
fn explore(
    peaks: &mut HashSet<Pos>,
    map: &Vec<Vec<usize>>,
    x: usize,
    y: usize,
    val: usize,
    w: usize,
    h: usize,
) -> usize {
    if val == 9 {
        peaks.insert(Pos { x, y });
        return 1;
    }
    let mut ret = 0;
    for (nx, ny) in get_neighbors(x, y, w, h) {
        let map_val = read_tile(map, nx, ny);
        if map_val == val + 1 {
            ret += explore(peaks, map, nx, ny, val + 1, w, h)
        }
    }
    return ret;
}
fn main() {
    let map: Vec<Vec<usize>> = fs::read("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let width = map[0].len();
    let height = map.len();

    let start_pos: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, ch)| **ch == 0)
                .map(move |(x, _)| (x, y))
        })
        .collect();

    let mut total = 0;
    let mut rating = 0;
    for (x, y) in start_pos {
        let mut peaks: HashSet<Pos> = HashSet::new();
        rating += explore(&mut peaks, &map, x, y, 0, width, height);
        total += peaks.len();
    }
    dbg!(total);
    dbg!(rating);
}
