use std::{fs, str, usize};

const OFFSETS: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, -1),
    (1, -1),
    (1, 1),
];

const SEARCH: &str = "XMAS";

fn in_bound(n: &(i32, i32), bounds: &(i32, i32)) -> bool {
    n.0 >= 0 && n.0 < bounds.0 && n.1 >= 0 && n.1 < bounds.1
}

fn find_rest(
    map: &Vec<Vec<char>>,
    start: (i32, i32),
    dir: (i32, i32),
    search: &[char],
    bounds: &(i32, i32),
) -> bool {
    if search.len() == 0 {
        return true;
    }

    let n = (start.0 + dir.0, start.1 + dir.1);
    if !in_bound(&n, bounds) || map[n.1 as usize][n.0 as usize] != search[0] {
        return false;
    }

    return find_rest(map, n, dir, &search[1..], bounds);
}

fn find_words(map: &Vec<Vec<char>>, x: i32, y: i32, bounds: &(i32, i32), search: &[char]) -> i32 {
    OFFSETS
        .iter()
        .filter(|(dx, dy)| find_rest(map, (x, y), (*dx, *dy), &search, &bounds))
        .count() as i32
}

fn sol1(map: &Vec<Vec<char>>) {
    let search: Vec<char> = SEARCH.chars().collect();
    let mut words: i32 = 0;
    let bounds = (map[0].len() as i32, map.len() as i32);

    for (y, line) in map.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch == search[0] {
                words += find_words(&map, x as i32, y as i32, &bounds, &search[1..]);
            }
        }
    }
    dbg!(words);
}

const DIAGS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

fn map_get(map: &Vec<Vec<char>>, x: i32, y: i32) -> char {
    return map[y as usize][x as usize];
}

fn sol2(map: &Vec<Vec<char>>) {
    let bounds = (map[0].len() as i32, map.len() as i32);
    let mut results = 0;

    for y in 1..(bounds.1 - 1) {
        for x in 1..(bounds.0 - 1) {
            let mut xcount = 0;
            if map_get(&map, x, y) == 'A' {
                for (dx, dy) in DIAGS {
                    if map_get(&map, x + dx, y + dy) == 'M' && map_get(&map, x - dx, y - dy) == 'S'
                    {
                        xcount += 1;
                    }
                }
            }
            if xcount == 2 {
                results += 1;
            }
        }
    }

    dbg!(results);
}

fn main() {
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().split_whitespace() {
        map.push(Vec::new());
        for ch in line.chars() {
            map.last_mut().unwrap().push(ch);
        }
    }

    sol2(&map);
}
