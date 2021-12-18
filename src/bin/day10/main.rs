use crate::LineResult::{Corrupted, Incomplete};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_input_from_file() -> Vec<String> {
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.unwrap()).collect()
}

#[derive(Clone)]
enum LineResult {
    Correct,
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn get_line_status(lines: Vec<String>) -> Vec<LineResult> {
    let mut ret_value = Vec::<LineResult>::new();
    ret_value.resize(lines.len(), LineResult::Correct);

    for (i, line) in lines.iter().enumerate() {
        let mut brackets_keeper = VecDeque::<char>::new();
        for ch in line.chars() {
            let bracket_to_remove = match ch {
                '(' | '[' | '{' | '<' => {
                    brackets_keeper.push_back(ch);
                    None
                }
                ')' => Some('('),
                ']' => Some('['),
                '}' => Some('{'),
                '>' => Some('<'),
                _ => panic!("Unrecognized character in line"),
            };
            if bracket_to_remove.is_some() && brackets_keeper.pop_back() != bracket_to_remove {
                ret_value[i] = Corrupted(ch);
                break;
            }
        }
        if !brackets_keeper.is_empty() && matches!(ret_value[i], LineResult::Correct) {
            ret_value[i] = Incomplete(brackets_keeper.iter().map(|c| *c).collect());
        }
    }
    ret_value
}

fn part1_score(line_results: &Vec<LineResult>) -> u64 {
    line_results.iter().fold(0, |score, ch| {
        return score
            + match ch {
                Corrupted(')') => 3,
                Corrupted(']') => 57,
                Corrupted('}') => 1197,
                Corrupted('>') => 25137,
                _ => 0,
            };
    })
}

fn part2_score(line_results: &Vec<LineResult>) -> u64 {
    let mut scores = Vec::<u64>::new();
    for line in line_results.iter() {
        if let Incomplete(v) = line {
            let mut score: u64 = 0;
            for ch in v.iter().rev() {
                score = 5 * score
                    + match ch {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    };
            }
            scores.push(score);
        }
    }
    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let input = read_input_from_file();
    let res = get_line_status(input);
    println!("{}", part1_score(&res));
    println!("{:?}", part2_score(&res));
}
