use regex::Regex;

fn sol1() {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let input = std::fs::read_to_string("input.txt").unwrap();

    let sum = re
        .captures_iter(&input)
        .map(|caps| {
            let (_, [x, y]) = caps.extract();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .fold(0, |acc, e: (i32, i32)| acc + (e.0 * e.1));

    dbg!(sum);
}

fn sol2() {
    let re = Regex::new(r"mul\((?<x>[0-9]+),(?<y>[0-9]+)\)|do\(\)|don't\(\)").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut active = true;
    let sum: i32 = re
        .captures_iter(&input)
        .map(|caps| -> i32 {
            match &caps[0] {
                "do()" => active = true,
                "don't()" => active = false,
                _ => {
                    if active {
                        return caps["x"].parse::<i32>().unwrap()
                            * caps["y"].parse::<i32>().unwrap();
                    }
                }
            }
            0
        })
        .fold(0, |acc, e: i32| acc + e);

    dbg!(sum);
}

fn main() {
    sol2();
}
