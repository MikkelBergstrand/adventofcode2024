use priority_queue::PriorityQueue;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::hash::Hash;

const SIZE: usize = 70;
const N_BYTES: usize = 1024;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

// Does not check for obstruction on map, but does not return out of bounds.
fn get_neighbors(pos: &Pos) -> Vec<Pos> {
    let mut ret: Vec<Pos> = Vec::new();
    if pos.x > 0 {
        ret.push(Pos {
            x: pos.x - 1,
            y: pos.y,
        });
    }
    if pos.x < SIZE {
        ret.push(Pos {
            x: pos.x + 1,
            y: pos.y,
        });
    }
    if pos.y > 0 {
        ret.push(Pos {
            x: pos.x,
            y: pos.y - 1,
        });
    }
    if pos.y < SIZE {
        ret.push(Pos {
            x: pos.x,
            y: pos.y + 1,
        });
    }
    ret
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let re = Regex::new(r"([0-9]+),([0-9]+)").unwrap();
    let byte_positions: Vec<Pos> = re
        .captures_iter(&input)
        .map(|c| {
            let (_, [x, y]) = c.extract();
            Pos {
                x: x.parse::<usize>().unwrap(),
                y: y.parse::<usize>().unwrap(),
            }
        })
        .collect();
    let max_bytes = byte_positions.len();

    for n_bytes in N_BYTES..max_bytes {
        let mut solvable = false;
        let last = byte_positions[n_bytes - 1].clone();
        let byte_positions: HashSet<Pos> =
            HashSet::from_iter(byte_positions[0..n_bytes].iter().cloned());
        dbg!(byte_positions.len());

        let mut frontier = PriorityQueue::new();
        let mut visited: HashSet<Pos> = HashSet::new();
        let goal = Pos { x: SIZE, y: SIZE };
        frontier.push(Pos { x: 0, y: 0 }, Reverse(0));
        while let Some((pos, cost)) = frontier.pop() {
            let cost = cost.0 as i32;

            if pos == goal {
                solvable = true;
                break;
            }

            visited.insert(pos.clone());
            // Neighbors are not out of bounds and are not occupied by a wall.
            let neighbors: Vec<Pos> = get_neighbors(&pos)
                .into_iter()
                .filter(|pos| !byte_positions.contains(&pos))
                .collect();
            for neigh in neighbors {
                if !visited.contains(&neigh) {
                    frontier.push_decrease(neigh, Reverse(cost + 1));
                }
            }
        }

        if !solvable {
            dbg!(&last);
            break;
        }
    }
}
