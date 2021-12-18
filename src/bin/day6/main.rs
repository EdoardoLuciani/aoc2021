use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

struct CountedSet {
    // age of fish, number of fish
    pub set: HashMap<u8, u64>,
}

impl CountedSet {
    fn new() -> Self {
        CountedSet {
            set: HashMap::new(),
        }
    }

    fn add(&mut self, key: u8, value: u64) {
        if let Some(val) = self.set.get_mut(&key) {
            *val += value;
        } else {
            self.set.insert(key, value);
        }
    }

    fn get_total_amount(&self) -> u64 {
        self.set.values().sum()
    }
}

fn read_values_from_file() -> CountedSet {
    let mut file = File::open("input.txt").unwrap();
    let mut line = String::new();
    file.read_to_string(&mut line);
    let mut out_set = CountedSet::new();
    for num in line.split(",") {
        out_set.add(num.parse::<u8>().unwrap(), 1);
    }
    out_set
}

fn simulate(days: u32, mut current_set: CountedSet) -> CountedSet {
    for day in 1..=days {
        let mut next_day_set = CountedSet::new();
        for (countdown, amount) in current_set.set.iter() {
            if *countdown == 0 {
                next_day_set.add(6, *amount);
                next_day_set.add(8, *amount);
            } else {
                next_day_set.add(countdown - 1, *amount);
            }
        }
        current_set = next_day_set;
    }
    current_set
}

fn main() {
    let nums = read_values_from_file();

    println!("{}", simulate(256, nums).get_total_amount());
}
