use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // position, depth and aim
    let mut starting_pos: (i32, i32, i32) = (0,0,0); 

    let file = File::open("input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let (command, value) = line.rsplit_once(" ").unwrap();
        let value = value.parse::<i32>().unwrap();
        match command {
            "down" => { starting_pos.2 += value; },
            "up" => { starting_pos.2 -= value; },
            "forward" => { 
                starting_pos.0 += value;
                starting_pos.1 += starting_pos.2 * value;
            },
            _ => (),
        };
    }
    println!("Final value {}", starting_pos.0 * starting_pos.1)
}
