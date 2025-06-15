use std::{char, collections::HashMap, fs, io::BufRead};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Size {
    width: usize,
    height: usize,
}

fn in_bounds(pos: &Pos, size: &Size) -> bool {
    return pos.x >= 0 && pos.x < size.width as i32 && pos.y >= 0 && pos.y < size.height as i32;
}

fn process_beacons(positions: &[Pos], size: &Size) -> HashMap<Pos, bool> {
    let mut ret: HashMap<Pos, bool> = HashMap::new();
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let diff = Pos {
                x: positions[j].x - positions[i].x,
                y: positions[j].y - positions[i].y,
            };

            let mut node = Pos {
                x: positions[i].x,
                y: positions[i].y,
            };
            while in_bounds(&node, &size) {
                ret.insert(node.clone(), true);
                node.x += diff.x;
                node.y += diff.y;
            }
            node = Pos {
                x: positions[j].x - diff.x,
                y: positions[j].y - diff.y,
            };

            while in_bounds(&node, &size) {
                ret.insert(node.clone(), true);
                node.x -= diff.x;
                node.y -= diff.y;
            }
        }
    }
    return ret;
}

fn main() {
    let mut beacons: HashMap<char, Vec<Pos>> = HashMap::new();

    let file = fs::read("input.txt").unwrap();
    let lines: Vec<_> = file.lines().collect();

    let size = Size {
        width: lines[0].as_ref().unwrap().len(),
        height: lines.len(),
    };

    for (y, line) in lines.into_iter().enumerate() {
        for (x, ch) in line.unwrap().chars().enumerate() {
            if ch != '.' {
                beacons.entry(ch).or_insert(Vec::new()).push(Pos {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    let mut all_antinodes: HashMap<Pos, bool> = HashMap::new();

    for freq in beacons.keys() {
        let antinodes = process_beacons(beacons.get(freq).unwrap(), &size);
        all_antinodes.extend(antinodes);
    }
    dbg!(all_antinodes.len());
}
