use std::fs;

use regex::Regex;

fn solve_sub_eq(equation: &[i64], i: usize, value: i64, goal: i64) -> bool {
    if value == goal {
        // We got it!
        return true;
    }
    if value > goal {
        // Too large, give up.
        return false;
    }

    if i >= equation.len() {
        return false;
    }

    // Concat two numbers to a larger number
    let concat = (value.to_string() + &equation[i].to_string())
        .parse::<i64>()
        .unwrap();

    return solve_sub_eq(equation, i + 1, value + equation[i], goal)
        || solve_sub_eq(equation, i + 1, value * equation[i], goal)
        || solve_sub_eq(equation, i + 1, concat, goal);
}
fn is_solvable(equation: &[i64]) -> bool {
    return solve_sub_eq(equation, 2, equation[1], equation[0]);
}

fn main() {
    let re = Regex::new(r"([0-9]+): (.*)").unwrap();
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let mut equations: Vec<Vec<i64>> = Vec::new();

    for (_, [first, rest]) in re.captures_iter(&contents).map(|c| c.extract()) {
        let mut equation: Vec<i64> = Vec::new();
        equation.push(first.parse::<i64>().unwrap());
        for num in rest.split(" ") {
            equation.push(num.parse::<i64>().unwrap());
        }
        equations.push(equation);
    }

    let mut solvable = 0;
    let mut sum: i64 = 0;
    for equation in equations {
        if is_solvable(&equation) {
            solvable += 1;
            sum += equation[0];
        }
    }

    dbg!(&solvable);
    dbg!(&sum);
}
