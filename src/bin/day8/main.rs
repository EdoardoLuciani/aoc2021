use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::collections::HashMap;

fn read_file() -> Vec::<(Vec<String>, Vec<String>)> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut ret_value = Vec::<(Vec<String>, Vec<String>)>::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if let Some((signal_patterns, output_values)) = line.split_once(" | ") {
            ret_value.push((signal_patterns.split(" ").map(|s| s.to_string()).collect(),
                            output_values.split(" ").map(|s| s.to_string()).collect()));
        }
    }
    ret_value
}

fn get_simple_digits(lines: &Vec::<(Vec<String>, Vec<String>)>) -> Vec::<u32> {
    let mut ret_value = Vec::<u32>::new();
    for (_, output_value) in lines.iter() {
        for value in output_value.iter() {
            match value.len() {
                2 => ret_value.push(1),
                3 => ret_value.push(7),
                4 => ret_value.push(4),
                7 => ret_value.push(8),
                _ => {},
            }
        }
    }
    ret_value
}


/*
       111
    2       3
    2       3
    2       3
       444
    5       6
    5       6
    5       6
       777
*/
// Function that given a number in the display format it its numeric counterpart
fn display_to_num(display : u8) -> Option<u8> {
    return match display {
        0b00010010 => Some(1),
        0b01011101 => Some(2),
        0b01011011 => Some(3),
        0b00111010 => Some(4),
        0b01101011 => Some(5),
        0b01101111 => Some(6),
        0b01010010 => Some(7),
        0b01111111 => Some(8),
        0b01111011 => Some(9),
        _ => None
    }
}

fn get_output_values(lines: &Vec::<(Vec<String>, Vec<String>)>) {
    for line in lines.iter() {
        let letter_to_bitpos = HashMap::<char, u8>::new();
        let mut combined_words : Vec::<String> = vec![line.0, line.1].into_iter().flatten().collect();
        combined_words.sort();
        for word in combined_words {
            match word.len() {
                2 => {
                    letter_to_bitpos.insert(word.chars().nth(0).unwrap(), 1 >> 4);
                    letter_to_bitpos.insert(word.chars().nth(1).unwrap(), 1 >> 7);
                }
                3 => {
                    letter_to_bitpos.insert(word.chars().nth(0).unwrap(), 3);
                    letter_to_bitpos.insert(word.chars().nth(1).unwrap(), 6);
                },
                4 => ret_value.push(4),
                7 => ret_value.push(8),
                _ => {},
            }
        }
    }
} 



fn main() {
    let lines = read_file();
    let count = get_simple_digits(&lines).len();
    println!("First part: {}", count);


}