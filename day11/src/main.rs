use std::{collections::HashMap, fs, usize};

#[derive(Eq, Hash, PartialEq, Debug)]
struct NodeKey {
    depth: usize,
    value: usize,
}

const ITER_NUM: usize = 75;

fn count_node(
    value: usize,
    depth: usize,
    max_depth: usize,
    cache: &mut HashMap<NodeKey, usize>,
) -> usize {
    // Check if cached
    let key = NodeKey {
        value,
        depth: max_depth - depth,
    };
    if let Some(cache_ret) = cache.get(&key) {
        return *cache_ret;
    }

    // Terminate at leaf nodes of search
    if depth >= max_depth {
        return 1;
    }

    // Compute result according to rules of the game.
    let result: usize;
    if value == 0 {
        result = count_node(1, depth + 1, max_depth, cache);
    } else if value.to_string().len() % 2 == 0 {
        let str_num = value.to_string();
        let num1: usize = str_num[0..str_num.len() / 2].parse::<usize>().unwrap();
        let num2: usize = str_num[str_num.len() / 2..].parse::<usize>().unwrap();
        result = count_node(num1, depth + 1, max_depth, cache)
            + count_node(num2, depth + 1, max_depth, cache);
    } else {
        result = count_node(2024 * value, depth + 1, max_depth, cache);
    }

    // Update cache with new value
    cache.insert(key, result);
    return result;
}

fn main() {
    let mut cache: HashMap<NodeKey, usize> = HashMap::new();
    let nums: Vec<usize> = fs::read_to_string("input.txt")
        .unwrap()
        .replace("\n", "")
        .split(" ")
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let mut result = 0;
    // Use a "iterative deepening" approach. Advance depth of search by 1 each time.
    // Since the cache is maintained between iterations, speed is supringsly fast!
    for iter in 1..(ITER_NUM + 1) {
        result = 0;
        for num in nums.iter() {
            result += count_node(*num, 0, iter, &mut cache);
        }
    }

    dbg!(result);
}
