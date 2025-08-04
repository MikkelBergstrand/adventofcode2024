use std::{collections::HashMap, fs, usize};

fn check<'a>(pattern: &'a str, designs: &Vec<&str>, cache: &mut HashMap<&'a str, usize>) -> usize {
    if pattern.len() == 0 {
        return 1;
    }

    if cache.contains_key(pattern) {
        return *cache.get(pattern).unwrap();
    }

    let valid_continuations = designs.into_iter().filter(|&s| pattern.starts_with(s));
    let mut ret = 0;
    for cont in valid_continuations {
        ret += check(&pattern[cont.len()..], designs, cache);
    }

    cache.insert(pattern, ret);
    ret
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (designs, patterns) = input.split_once("\n\n").unwrap();

    let designs: Vec<&str> = designs.split(", ").collect();
    let patterns: Vec<&str> = patterns.lines().collect();
    let mut cache: HashMap<&str, usize> = HashMap::new();
    let count: usize = patterns
        .iter()
        .map(|pattern| check(pattern, &designs, &mut cache))
        .sum();

    dbg!(&count);
}
