use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_sonar_input_from_file() -> Vec::<i32> {
    let mut values = Vec::new();
    let file = File::open("input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        values.push(line.unwrap().parse::<i32>().unwrap());
    }
    values
}

fn get_3_sliding_window(values: &Vec::<i32>) -> Vec::<i32> {
    let mut result = Vec::new();
    for i in 0..values.len() {
        let mut sum = 0;
        for j in i..i+3 {
            match values.get(j) {
                Some(val) => sum += val,
                None => (),
            };
        }
        result.push(sum);
    }
    result
}

fn main() {
    let values = get_sonar_input_from_file();
    let values = get_3_sliding_window(&values);

    let mut old_value: Option<i32> = None::<i32>;
    let mut increases: i32 = 0;
    for value in values.iter() {
        match old_value {
            Some(old_value) => {
                if *value > old_value {
                    println!("{} increasing", value);
                    increases += 1;
                }
                else if *value <= old_value {
                    println!("{} decreasing", value);
                }
                else {
                    println!("{} no change", value);
                }
            },
            None => {
                println!("{} no prev measurements", value);
            },
        };
        old_value = Some(*value);
    }
    println!("{} increases", increases);
}
