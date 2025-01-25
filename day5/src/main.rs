use regex::Regex;
use std::{collections::HashMap, fs, vec};

fn main() {
    let empty_vec: Vec<u32> = Vec::new();
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let contents = fs::read_to_string("input.txt").unwrap();
    let re1 = Regex::new(r"([0-9]+)\|([0-9]+)\n").unwrap();

    let parsed: Vec<_> = contents.split("\n\n").collect();
    let (rules_str, pages_str) = (parsed[0], parsed[1]);

    for (_, [before_page, after_page]) in re1.captures_iter(rules_str).map(|c| c.extract()) {
        let vec = &mut rules
            .entry(before_page.parse::<u32>().unwrap())
            .or_insert(vec![]);
        vec.push(after_page.parse::<u32>().unwrap());
    }

    let mut sum = 0;
    for page_numbers in pages_str.split_whitespace() {
        let mut page_numbers: Vec<u32> = page_numbers
            .split(',')
            .map(|c| c.parse::<u32>().unwrap())
            .collect();

        let mut iterations = 0;
        let mut verified = false;

        while !verified {
            verified = true;
            iterations += 1;

            for idx in 0..page_numbers.len() {
                // Get set of pages that cannot be before this number,
                // compare them with the observed numbers.
                // If a match, the ordering is invalid.
                let rules_pages: &Vec<u32> = rules.get(&page_numbers[idx]).unwrap_or(&empty_vec);
                for i in 0..idx {
                    if rules_pages.contains(&page_numbers[i]) {
                        // Swap values to order them correctly.
                        verified = false;
                        page_numbers.swap(idx, i);
                    }
                }
            }
        }

        // check that the series was faulty, meaning more iterations had to be done
        if iterations > 1 {
            sum += page_numbers[page_numbers.len() / 2];
        }
    }

    dbg!(sum);
}
