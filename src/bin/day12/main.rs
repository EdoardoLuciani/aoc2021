use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

enum CaveType {
    Start,
    End,
    Big,
    Small,
}

struct Cave {
    cave_type: CaveType,
    name: String,
    connected_caves: Vec<Weak<RefCell<Cave>>>,
}

impl Cave {
    fn new(name: &str) -> Self {
        Cave {
            cave_type: {
                match name {
                    "start" => CaveType::Start,
                    "end" => CaveType::End,
                    name if name.chars().nth(0).unwrap().is_uppercase() => CaveType::Big,
                    _ => CaveType::Small,
                }
            },
            name: name.to_string(),
            connected_caves: Vec::new(),
        }
    }

    fn add_neighbour(&mut self, neighbour: Weak<RefCell<Cave>>) {
        self.connected_caves.push(neighbour);
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.name.as_str())
    }
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ->", self)?;
        for n_cave in self.connected_caves.iter() {
            write!(f, " {}", RefCell::borrow(&n_cave.upgrade().unwrap()))?;
        }
        f.write_str("\n")
    }
}

fn read_input_from_file() -> Vec<Rc<RefCell<Cave>>> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut caves: HashMap<String, Rc<RefCell<Cave>>> = HashMap::new();
    let mut cave_to_cave = Vec::<(String, String)>::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let caves_names = line.split_once('-').unwrap();

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

    for (cave_in_name, cave_out_name) in cave_to_cave {
        let cave_in = &caves[&cave_in_name];
        let cave_out = &caves[&cave_out_name];
        RefCell::borrow_mut(cave_in).add_neighbour(Rc::downgrade(cave_out));
        RefCell::borrow_mut(cave_out).add_neighbour(Rc::downgrade(cave_in));
    }

    caves.values().cloned().collect()
}

fn paths_from_start_to_end_part1(
    start_cave: &Rc<RefCell<Cave>>,
    caves: &Vec<Rc<RefCell<Cave>>>,
    mut visited: HashSet<String>,
    mut path_str: String,
    total_paths: &mut u64,
) {
    path_str.push_str(format!("{} -> ", RefCell::borrow(start_cave)).as_str());
    if matches!(RefCell::borrow(start_cave).cave_type, CaveType::Small) {
        visited.insert(RefCell::borrow(start_cave).name.clone());
    }

    for neighbour in RefCell::borrow(start_cave).connected_caves.iter() {
        if matches!(
            RefCell::borrow(&neighbour.upgrade().unwrap()).cave_type,
            CaveType::End
        ) {
            path_str.push_str("end");
            println!("{}", path_str);
            *total_paths += 1;
        } else if !visited.contains(&RefCell::borrow(&neighbour.upgrade().unwrap()).name) {
            paths_from_start_to_end_part1(
                &neighbour.upgrade().unwrap(),
                caves,
                visited.clone(),
                path_str.clone(),
                total_paths,
            );
        }
    }
}

#[derive(Clone)]
struct VisitedSmallCaves {
    visited: HashSet<String>,
    small_cave_visited_twice: bool,
}

impl VisitedSmallCaves {
    fn new() -> Self {
        VisitedSmallCaves {
            visited: HashSet::new(),
            small_cave_visited_twice: false,
        }
    }

    fn visit(&mut self, cave_name: String) {
        if !self.small_cave_visited_twice && self.visited.contains(&cave_name) {
            self.small_cave_visited_twice = true;
        } else {
            self.visited.insert(cave_name);
        }
    }

    fn can_visit(&mut self, cave_name: &str) -> bool {
        if (self.small_cave_visited_twice && self.visited.contains(cave_name))
            || cave_name == "start"
        {
            return false;
        }
        true
    }
}

fn paths_from_start_to_end_part2(
    start_cave: &Rc<RefCell<Cave>>,
    caves: &Vec<Rc<RefCell<Cave>>>,
    mut visited: VisitedSmallCaves,
    mut path_str: String,
    total_paths: &mut u64,
) {
    path_str.push_str(format!("{} -> ", RefCell::borrow(start_cave)).as_str());
    if matches!(RefCell::borrow(start_cave).cave_type, CaveType::Small) {
        visited.visit(RefCell::borrow(start_cave).name.clone());
    }

    for neighbour in RefCell::borrow(start_cave).connected_caves.iter() {
        if matches!(
            RefCell::borrow(&neighbour.upgrade().unwrap()).cave_type,
            CaveType::End
        ) {
            path_str.push_str("end");
            println!("{}", path_str);
            *total_paths += 1;
        } else if visited.can_visit(RefCell::borrow(&neighbour.upgrade().unwrap()).name.as_str()) {
            paths_from_start_to_end_part2(
                &neighbour.upgrade().unwrap(),
                caves,
                visited.clone(),
                path_str.clone(),
                total_paths,
            );
        }
    }
}

fn main() {
    let caves = read_input_from_file();
    let start_cave = caves
        .iter()
        .find(|c| matches!(RefCell::borrow(c).cave_type, CaveType::Start))
        .expect("No start cave found");

    let mut total_paths: u64 = 0;
    let mut visited = HashSet::new();
    visited.insert("start".to_string());
    paths_from_start_to_end_part1(start_cave, &caves, visited, String::new(), &mut total_paths);
    println!("Total paths {}", total_paths);

    total_paths = 0;
    let mut visited_part2 = VisitedSmallCaves::new();
    visited_part2.visit("start".to_string());
    paths_from_start_to_end_part2(
        start_cave,
        &caves,
        visited_part2,
        String::new(),
        &mut total_paths,
    );
    println!("Total paths {}", total_paths);
}
