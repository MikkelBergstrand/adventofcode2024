use std::{char::EscapeDebug, fs, usize};

use regex::Regex;

#[derive(Debug)]
struct Equation {
    a: usize,
    b: usize,
    target: usize,
}

// Two equations must add up.
#[derive(Debug)]
struct Rules {
    x: Equation,
    y: Equation,
}

fn main() {
    let re = Regex::new(
        r"Button A: X\+([0-9]+), Y\+([0-9]+)
Button B: X\+([0-9]+), Y\+([0-9]+)
Prize: X=([0-9]+), Y=([0-9]+)",
    )
    .unwrap();
    let hay = fs::read_to_string("input.txt").unwrap();
    let mut rules: Vec<Rules> = vec![];

    for (_, [a_x, a_y, b_x, b_y, target_x, target_y]) in re.captures_iter(&hay).map(|c| c.extract())
    {
        rules.push(Rules {
            x: Equation {
                a: a_x.parse().unwrap(),
                b: b_x.parse().unwrap(),
                target: target_x.parse::<usize>().unwrap() + 10000000000000,
            },
            y: Equation {
                a: a_y.parse().unwrap(),
                b: b_y.parse().unwrap(),
                target: target_y.parse::<usize>().unwrap() + 10000000000000,
            },
        });
    }

    let mut result: usize = 0;
    for rule in rules.iter() {
        //dbg!(&rule);
        let nom_a = rule.y.target * rule.x.a;
        let nom_b = rule.x.target * rule.y.a;

        let denom_a = rule.x.a * rule.y.b;
        let denom_b = rule.y.a * rule.x.b;

        let nom = nom_a.abs_diff(nom_b);
        let denom = denom_a.abs_diff(denom_b);

        //dbg!((nom, denom));

        if nom % denom == 0 {
            // Solution is possible: y is integer.
            let sol_y: usize = nom / denom;

            // Now, also check that x is integer.
            let sol_x_nom: usize = rule.y.target - rule.y.b * sol_y;
            if sol_x_nom % rule.y.a == 0 {
                let sol_x: usize = sol_x_nom / rule.y.a;

                let tokens: usize = 3 * sol_x + sol_y;
                result += tokens;
                dbg!((sol_x, sol_y, tokens));
            }
        }
    }
    dbg!(result);
}
