use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_and_convert() -> Vec::<u32> {
    let mut values = Vec::<u32>::new();
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        values.push(u32::from_str_radix(&line.unwrap(), 2).unwrap());
    }
    values
}

fn get_gamma_and_epsilon(values: &Vec::<u32>) -> (u32, u32) {
    let (mut gamma, mut epsilon): (u32, u32) = (0,0);
    for bit in (0..32).rev() {
        let (mut set_1, mut set_0): (u32, u32) = (0,0);
        for value in values.iter() {
            if value & (1 << bit) != 0 {
                set_1 += 1
            }
            else {
                set_0 += 1;
            }
        }
        if set_0 != values.len() as u32 {
            if set_1 == std::cmp::max(set_1, set_0) {
                gamma += 1 << bit;
            }
            else {
                epsilon += 1 << bit;
            }
        }
    }
    (gamma, epsilon)
}

fn get_val(mut values: Vec::<u32>, choosing_fun: impl Fn(u32, u32) -> bool) -> u32 {
    for bit in (0..32).rev() {
        let (mut set_1, mut set_0): (u32, u32) = (0,0);
        for value in values.iter() {
            if value & (1 << bit) != 0 {
                set_1 += 1
            }
            else {
                set_0 += 1;
            }
        }
        if set_0 != values.len() as u32 {
            let mut new_oxy_values = Vec::<u32>::new();
            if choosing_fun(set_1, set_0) {
                for value in values.iter() {
                    if value & (1 << bit) != 0 {
                        new_oxy_values.push(*value);
                    }
                }
            }
            else {
                for value in values.iter() {
                    if value & (1 << bit) == 0 {
                        new_oxy_values.push(*value);
                    }
                }
            }
            if new_oxy_values.len() == 1 {
                return new_oxy_values[0];
            }
            else {
                values = new_oxy_values;
            }
        }
    }
    0
}

fn get_oxygen_and_co2(values: &Vec::<u32>) -> (u32, u32) {
    let find_oxy = |set_1, set_0| set_1 == std::cmp::max(set_1, set_0) || set_1 == set_0;
    let find_co2 = |set_1, set_0| set_1 == std::cmp::min(set_1, set_0) && set_1 != set_0;
    (get_val(values.clone(), find_oxy), get_val(values.clone(), find_co2))
}


fn main() {
    let values = read_and_convert();
    let (gamma, epsilon) = get_gamma_and_epsilon(&values);
    let (oxy, co2) = get_oxygen_and_co2(&values);

    println!("Power consumption is {}", gamma * epsilon);
    println!("Life rating is {}", oxy*co2);
}
