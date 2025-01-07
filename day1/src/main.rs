use std::fs;
fn parse_lines() -> (Vec<usize>, Vec<usize>) {
    let mut left_numbers: Vec<usize> = Vec::new(); 
    let mut right_numbers: Vec<usize> = Vec::new(); 

    for line in fs::read_to_string("input.txt").unwrap().lines() {

        let mut words = line.split_whitespace();

        let x = words.next().unwrap().parse().expect("Not a number");
        let y = words.next().unwrap().parse().expect("Not a number");
        left_numbers.push(x);
        right_numbers.push(y);
    }

    return (left_numbers, right_numbers);

}
fn sol1() {
    let (mut left_numbers, mut right_numbers) = parse_lines();
    left_numbers.sort();
    right_numbers.sort();

    dbg!(&left_numbers);
    dbg!(&right_numbers);

    let ans: i32 = left_numbers.iter()
        .zip(right_numbers.iter())
        .map(|(&x, &y)| (x as i32 - y as i32).abs())
        .sum();

    dbg!(ans);
}

fn sol2() {
    let (left_numbers, right_numbers) = parse_lines();
    
    let mut sum: usize = 0;
    for left_no in left_numbers {
        sum += left_no as usize * right_numbers.iter()
            .filter(|&x| *x == left_no)
            .count();
    }
    dbg!(sum);
    
}

fn main() {
    sol1();
    sol2();
}

