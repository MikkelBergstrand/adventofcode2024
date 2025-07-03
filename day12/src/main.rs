use std::{collections::HashSet, io::BufRead, usize};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

struct Size {
    width: i32,
    height: i32,
}

// Offsets in a clockwise direction.
const OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn in_bounds(pos: &Pos, size: &Size) -> bool {
    return pos.x >= 0 && pos.x < size.width && pos.y >= 0 && pos.y < size.height;
}

fn map_get(map: &Vec<Vec<char>>, pos: &Pos, size: &Size) -> char {
    if !in_bounds(pos, size) {
        return '\0';
    }
    return map[pos.y as usize][pos.x as usize];
}

fn explore_area(
    map: &Vec<Vec<char>>,
    initial: &Pos,
    size: &Size,
) -> (HashSet<Pos>, usize, HashSet<Pos>) {
    let mut fringe_positions: Vec<Pos> = Vec::new(); // Which are to be visited
    let mut area: HashSet<Pos> = HashSet::new(); // Which has been visited
    let mut border: HashSet<Pos> = HashSet::new(); // Not visited, but adjacent to the area, and

    let mut edges = 0;
    let current_char = map[initial.y as usize][initial.x as usize];

    fringe_positions.push(initial.clone());
    area.insert(initial.clone());

    while let Some(pos) = fringe_positions.pop() {
        // If adjacent to out of bounds, update perimeter count.

        for (ox, oy) in OFFSETS {
            let neigh = Pos {
                x: pos.x + ox,
                y: pos.y + oy,
            };

            let neigh_val = map_get(map, &neigh, &size);
            // Update search
            if neigh_val == current_char && !area.contains(&neigh) {
                fringe_positions.push(neigh.clone());
                area.insert(neigh.clone());
            } else if neigh_val != current_char && neigh_val != '\0' {
                border.insert(neigh.clone());
            }
        }

        for i in 0..4 {
            // Look out two orthogonal tiles, as well as the diagonal tile connecting them.
            let offset1 = Pos {
                x: pos.x + OFFSETS[i].0,
                y: pos.y + OFFSETS[i].1,
            };
            let offset2 = Pos {
                x: pos.x + OFFSETS[(i + 1) % 4].0,
                y: pos.y + OFFSETS[(i + 1) % 4].1,
            };
            let diag = Pos {
                x: pos.x + OFFSETS[i].0 + OFFSETS[(i + 1) % 4].0,
                y: pos.y + OFFSETS[i].1 + OFFSETS[(i + 1) % 4].1,
            };

            // Two orthogonal tiles not connected to area: corner
            if map_get(map, &offset1, size) != current_char
                && map_get(map, &offset2, size) != current_char
            {
                edges += 1;
            }

            // Two orthogonal tiles connected, but the tile between them (i.e. diagonally to the
            // tile) is not: corner
            if map_get(map, &offset1, size) == current_char
                && map_get(map, &offset2, size) == current_char
                && map_get(map, &diag, size) != current_char
            {
                edges += 1;
            }
        }
    }

    (area, edges, border)
}

fn main() {
    let map: Vec<Vec<char>> = std::fs::read("input.txt")
        .unwrap()
        .lines()
        .into_iter()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let size = Size {
        width: map[0].len() as i32,
        height: map.len() as i32,
    };

    let mut unexplored: HashSet<Pos> = HashSet::from([Pos { x: 0, y: 0 }]);
    let mut explored: HashSet<Pos> = HashSet::new();

    let mut result = 0;

    while !unexplored.is_empty() {
        let initial = unexplored.iter().next().unwrap().clone();
        unexplored.remove(&initial);

        let (area, edges, border) = explore_area(&map, &initial, &size);

        result += area.len() * edges;

        // Add to explored area.
        explored.extend(area.iter().cloned());

        // Unexplored = border of last searched area - already explored nodes.
        unexplored.extend(border.iter().cloned());
        unexplored.retain(|pos| !explored.contains(&pos));
    }
    dbg!(result);
}
