use std::fs;

fn parse_lines() -> Vec<Vec<i32>> {
    fs::read_to_string("input.txt")
        .unwrap() 
        .lines()
        .map(|line| { 
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn validate_differences(differences: &[i32]) -> bool {
    (differences.iter().all(|&x| x >= 0)  || 
     differences.iter().all(|&x| x <= 0)) && 
    !differences.iter().any(|&x| x.abs() < 1 || x.abs() > 3)
}

fn is_valid_sol1(numbers: &[i32]) -> bool {
    let differences: Vec<i32> = numbers
        .windows(2)
        .map(|slice| slice[1] - slice[0])
        .collect();

    validate_differences(&differences)
}

fn is_valid_sol2(numbers: &[i32]) -> bool {
    for i in 0..(numbers.len()) {
        let mut nos: Vec<i32> = numbers.to_vec();
        nos.remove(i);

        let differences: Vec<i32> = nos
            .windows(2)
            .map(|slice| slice[1] - slice[0])
            .collect();

        if validate_differences(&differences) {
            return true
        }
    }

    false
}

fn main() {
    let lines = parse_lines();
    
    let count = lines
        .iter()
        .filter(|x| is_valid_sol2(x))
        .count();

    dbg!(count);

}
