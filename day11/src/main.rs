use std::fs;

struct Cached {
    value: i32,
    result: i32,
}

const ITER_NUM: i32 = 75;

fn main() {
    let mut nums: Vec<usize> = fs::read_to_string("input.txt")
        .unwrap()
        .replace("\n", "")
        .split(" ")
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    for iter in 0..6 {
        let mut i = 0;
        while i < nums.len() {
            if nums[i] == 0 {
                nums[i] = 1;
                i += 1;
            } else if nums[i].to_string().len() % 2 == 0 {
                let str_num = nums[i].to_string();
                nums[i] = str_num[0..str_num.len() / 2].parse::<usize>().unwrap();
                nums.insert(
                    i + 1,
                    str_num[str_num.len() / 2..].parse::<usize>().unwrap(),
                );
                i += 2;
            } else {
                nums[i] *= 2024;
                i += 1;
            }
        }
        dbg!((iter, &nums));
    }

    dbg!(nums.len());
}
