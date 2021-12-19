use std::collections::{HashMap, LinkedList};
use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_from_file() -> (String, HashMap<String, String>) {
    let file = File::open("input.txt").unwrap();
    let lines: Vec<String> = BufReader::new(&file).lines().map(|s| s.unwrap()).collect();

    let polymer = lines.iter().next().unwrap().to_string();

    let rules: HashMap<String, String> = lines
        .iter()
        .filter_map(|s| s.split_once(" -> "))
        .map(|p| {
            (
                p.0.to_string(),
                format!(
                    "{}{}",
                    p.1.chars().nth(0).unwrap(),
                    p.0.chars().nth(1).unwrap()
                ),
            )
        })
        .collect();

    (polymer, rules)
}

fn compute_steps(mut polymer: String, rules: &HashMap<String, String>, steps: u64) -> u64 {
    for step in 0..steps {
        println!("Step {}", step);
        let mut ret_val = String::new();
        ret_val.write_char(polymer.chars().nth(0).unwrap());

        for i in 0..polymer.len() {
            if let Some(substring) = polymer.get(i..=i + 1) {
                match rules.get(substring) {
                    Some(substitution) => ret_val.push_str(substitution),
                    None => ret_val.push_str(substring),
                };
            }
        }
        polymer = ret_val;
    }

    // calculate the score
    let mut counter = HashMap::<char, u64>::new();
    for ch in polymer.chars() {
        *counter.entry(ch).or_default() += 1;
    }
    counter.values().max().unwrap() - counter.values().min().unwrap()
}

fn compute_steps_optimized(
    mut polymer: String,
    rules: &HashMap<String, String>,
    steps: u64,
) -> f64 {
    let mut pairs = HashMap::<String, u64>::new();
    for i in 0..polymer.len() {
        if let Some(substring) = polymer.get(i..=i + 1) {
            *pairs.entry(substring.to_string()).or_default() += 1;
        }
    }

    for step in 0..steps {
        let mut new_pairs = HashMap::<String, u64>::new();
        for (pair, count) in pairs.iter_mut() {
            match rules.get(pair) {
                Some(substitution) => {
                    *new_pairs
                        .entry(format!(
                            "{}{}",
                            pair.chars().nth(0).unwrap(),
                            substitution.chars().nth(0).unwrap()
                        ))
                        .or_default() += *count;
                    *new_pairs.entry(substitution.clone()).or_default() += *count;
                }
                None => *new_pairs.entry(pair.clone()).or_default() += *count,
            }
        }
        pairs = new_pairs;
    }

    let mut counter = HashMap::<char, f64>::new();
    for (pair, count) in pairs.iter() {
        *counter
            .entry(pair.chars().nth(0).unwrap())
            .or_insert(0.0f64) += 0.5f64 * (*count) as f64;
        *counter
            .entry(pair.chars().nth(1).unwrap())
            .or_insert(0.0f64) += 0.5f64 * (*count) as f64;
    }
    let max = counter.values().fold(f64::NAN, |a, b| a.max(*b));
    let min = counter
        .values()
        .max_by(|a, b| b.partial_cmp(a).unwrap())
        .unwrap();
    max - min
}

fn main() {
    let (polymer, rules) = read_input_from_file();
    //println!("{}", compute_steps(polymer.clone(), &rules, 10));
    println!("{}", compute_steps_optimized(polymer.clone(), &rules, 40));
}
