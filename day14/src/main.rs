use regex::Regex;
use std::io;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    pos: Pos,
    vel: Pos,
}

#[derive(Debug)]
struct Size {
    width: i32,
    height: i32,
}

// Non-negative modulo
fn modulo(a: i32, n: i32) -> i32 {
    ((a % n) + n) % n
}

const SIZE: Size = Size {
    width: 101,
    height: 103,
};

fn draw(robots: &Vec<Robot>) {
    let mut map = [['.'; SIZE.width as usize]; SIZE.height as usize];

    for robot in robots.iter() {
        map[robot.pos.y as usize][robot.pos.x as usize] = '#';
    }

    for y in 0..SIZE.height {
        for x in 0..SIZE.width {
            print!("{}", map[y as usize][x as usize]);
        }
        println!();
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let re = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    let mut robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            let (_, [px, py, vx, vy]) = re.captures(line).unwrap().extract();
            Robot {
                pos: Pos {
                    x: px.parse().unwrap(),
                    y: py.parse().unwrap(),
                },
                vel: Pos {
                    x: vx.parse().unwrap(),
                    y: vy.parse().unwrap(),
                },
            }
        })
        .collect();

    // Simulate
    for seconds in 0..100000 {
        robots.iter_mut().for_each(|robot| {
            robot.pos.x = modulo(robot.pos.x + robot.vel.x, SIZE.width);
            robot.pos.y = modulo(robot.pos.y + robot.vel.y, SIZE.height);
        });

        if seconds >= 1000 && (((seconds - 1081) % 101) == 0) {
            println!("After {} seconds:", seconds + 1);
            draw(&robots);

            // Pause stdout
            let mut buf: String = String::new();
            io::stdin().read_line(&mut buf).unwrap();
        }
    }

    // Classify quadrant
    //
    //
    let mut quadrant_count: [usize; 4] = [0; 4];
    for robot in robots.iter() {
        // Define quadrant as such (here using compass notation): 0 = NW, 1 = NE, 2 = SW, 3 = SE
        let mut quadrant_id = 0;
        // Ignore if on quadrant borders
        if robot.pos.x == SIZE.width / 2 || robot.pos.y == SIZE.height / 2 {
            continue;
        }
        if robot.pos.x > SIZE.width / 2 {
            quadrant_id += 1;
        }
        if robot.pos.y > SIZE.height / 2 {
            quadrant_id += 2;
        }

        quadrant_count[quadrant_id] += 1;
    }
    let product = quadrant_count.into_iter().reduce(|a, b| a * b).unwrap();
    dbg!(product);
    assert!(robots.len() == 500);
}
