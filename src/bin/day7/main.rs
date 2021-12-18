use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

fn read_values_from_file() -> Vec<i32> {
    let mut file = File::open("input.txt").unwrap();
    let mut line = String::new();
    file.read_to_string(&mut line);
    let mut out_vec = Vec::<i32>::new();
    for num in line.split(",") {
        out_vec.push(num.parse::<i32>().unwrap());
    }
    out_vec
}

fn compute_cost_for_all_points(
    nums: &Vec<i32>,
    cost_function: impl Fn(i32) -> u32,
) -> BTreeMap<u32, u32> {
    // First value is gonna be the cost and the second is the x coordinate
    let mut out_map = BTreeMap::new();
    let max_coordinate = *nums.iter().max().unwrap();
    for x in 0..=max_coordinate {
        let mut cost = 0;
        for num in nums.iter() {
            cost += cost_function((num - x).abs());
        }
        out_map.insert(cost, x as u32);
    }
    out_map
}

fn main() {
    let nums = read_values_from_file();

    let res1 = compute_cost_for_all_points(&nums, |distance| distance as u32);
    let res2 =
        compute_cost_for_all_points(&nums, |distance| ((distance) * (distance + 1) / 2) as u32);

    println!("{:?} {:?}", res1.iter().min(), res2.iter().min());
}
