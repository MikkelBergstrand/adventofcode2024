use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::{char, usize};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone, Ord, PartialOrd)]
enum Dir {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Dir {
    fn from_usize(value: usize) -> Dir {
        let value = value % 4;
        match value {
            0 => Dir::North,
            1 => Dir::East,
            2 => Dir::South,
            3 => Dir::West,
            _ => panic!("Bad usize"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct State {
    pos: Pos,
    dir: Dir,
}

struct NeighInfo {
    prev: Vec<State>,
    cost: usize,
}

fn map_read(map: &Vec<Vec<char>>, pos: &Pos) -> char {
    return map[pos.y][pos.x];
}

fn map_set(map: &mut Vec<Vec<char>>, pos: &Pos, value: char) {
    map[pos.y][pos.x] = value;
}

fn advance_pos(pos: &Pos, dir: &Dir) -> Pos {
    match dir {
        Dir::West => Pos {
            x: pos.x - 1,
            y: pos.y,
        },
        Dir::East => Pos {
            x: pos.x + 1,
            y: pos.y,
        },
        Dir::North => Pos {
            x: pos.x,
            y: pos.y - 1,
        },
        Dir::South => Pos {
            x: pos.x,
            y: pos.y + 1,
        },
    }
}

fn find(map: &Vec<Vec<char>>, ch: char) -> Option<Pos> {
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|c| *c == ch) {
            return Some(Pos { x, y });
        }
    }
    None
}

fn neighbors(map: &Vec<Vec<char>>, state: &State) -> Vec<(State, usize)> {
    let mut ret: Vec<(State, usize)> = Vec::new();

    let dir: usize = state.dir as usize;

    // Check if we can move forward.
    let advanced_pos = advance_pos(&state.pos, &state.dir);
    if map_read(&map, &advanced_pos) == '.' {
        let state = State {
            pos: advanced_pos,
            dir: state.dir,
        };

        ret.push((state, 1));
    }

    ret.push((
        State {
            pos: state.pos.clone(),
            dir: Dir::from_usize(dir.wrapping_add(1)),
        },
        1000,
    ));
    ret.push((
        State {
            pos: state.pos.clone(),
            dir: Dir::from_usize(dir.wrapping_sub(1)),
        },
        1000,
    ));
    ret
}

fn main() {
    let mut map: Vec<Vec<char>> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start_pos = find(&map, 'S').unwrap();
    let end_pos = find(&map, 'E').unwrap();

    map_set(&mut map, &start_pos, '.');
    map_set(&mut map, &end_pos, '.');

    let mut frontier = PriorityQueue::new();

    let initial_state = State {
        pos: start_pos,
        dir: Dir::East,
    };

    frontier.push(initial_state.clone(), Reverse(0));
    let mut visited: HashMap<State, NeighInfo> = HashMap::new();
    let mut best_cost = usize::MAX;

    visited.insert(
        initial_state,
        NeighInfo {
            prev: Vec::new(),
            cost: 0,
        },
    );

    let mut end_of_best_paths_search: Vec<State> = Vec::new();

    while let Some((state, cost)) = frontier.pop() {
        if state.pos == end_pos {
            if best_cost == cost.0 {
                end_of_best_paths_search.push(state.clone());
            } else if cost.0 < best_cost {
                end_of_best_paths_search = Vec::from([state.clone()]);
                best_cost = cost.0;
            }
        }

        for (neighbor_state, neighbor_cost) in neighbors(&map, &state) {
            let total_cost = cost.0 + neighbor_cost;
            let neigh_cost = match visited.get(&neighbor_state) {
                Some(neigh) => neigh.cost,
                None => 0,
            };

            if !visited.contains_key(&neighbor_state) || total_cost <= neigh_cost {
                if total_cost == neigh_cost {
                    visited
                        .get_mut(&neighbor_state)
                        .unwrap()
                        .prev
                        .push(state.clone());
                } else {
                    visited.insert(
                        neighbor_state.clone(),
                        NeighInfo {
                            prev: Vec::from([state.clone()]),
                            cost: total_cost,
                        },
                    );
                }
                frontier.push_decrease(neighbor_state, Reverse(total_cost));
            }
        }
    }

    let mut part_of_best: HashSet<Pos> = HashSet::new();
    while let Some(state) = end_of_best_paths_search.pop() {
        part_of_best.insert(state.pos.clone());
        if let Some(neigh_state) = visited.get(&state) {
            neigh_state
                .prev
                .iter()
                .for_each(|x| end_of_best_paths_search.push(x.clone()));
        }
    }
    dbg!(part_of_best.len());
}
