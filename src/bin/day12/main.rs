use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet};
use std::fs::{read, File};
use std::io::{prelude::*, BufReader};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

enum CaveType {
    Start,
    End,
    Big,
    Small,
}

struct Cave {
    pub cave_type: CaveType,
    pub name: String,
    pub connected_caves: Vec<Rc<Cave>>,
}

impl Cave {
    fn new(name: &str) -> Self {
        Cave {
            cave_type: CaveType::Big,
            name: name.to_string(),
            connected_caves: Vec::new(),
        }
    }

    fn add_neighbour(&mut self, neighbour: Rc<Cave>) {
        self.connected_caves.push(neighbour);
    }
}

fn read_input_from_file() -> HashMap<String, Rc<RefCell<Cave>>> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut caves: HashMap<String, Rc<RefCell<Cave>>> = HashMap::new();
    let mut cave_to_cave = Vec::<(String, String)>::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut caves_names = line.split_once("-").unwrap();

        cave_to_cave.push((caves_names.0.to_string(), caves_names.1.to_string()));
        caves.insert(
            caves_names.0.to_string(),
            Rc::new(RefCell::new(Cave::new(caves_names.0))),
        );
        caves.insert(
            caves_names.1.to_string(),
            Rc::new(RefCell::new(Cave::new(caves_names.1))),
        );
    }

    let cave_in = caves.get_mut("start").unwrap().get_mut();
    cave_in.name.push_str("hello");

    caves
}

fn main() {
    read_input_from_file();

    let v = Vec::<u32>::new();
    let v : Vec<u32> = Vec::new();
}
