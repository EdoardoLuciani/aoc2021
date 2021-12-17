use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, prelude::*};

extern crate num;
use num::signum;


struct Line {
    pub start: (i32, i32),
    pub end: (i32, i32),
    curr: (i32, i32),
    pub dir: (i32, i32),
}

impl Line {
    fn new(nums: [i32; 4]) -> Self {
        Line{start: (nums[0], nums[1]), 
            end: (nums[2], nums[3]),
            curr: (nums[0], nums[1]),
            dir: (signum(nums[2] - nums[0]), signum(nums[3] - nums[1])),
        }
    }
}

impl Iterator for Line {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr != (self.end.0 + self.dir.0, self.end.1 + self.dir.1) {
            let ret = Some(self.curr);
            self.curr.0 += self.dir.0;
            self.curr.1 += self.dir.1;
            return ret;
        }
        None
    }
}

struct Diagram {
    diagram_map: HashMap<(i32, i32), u32>,
}

impl Diagram {
    fn new() -> Self {
        Diagram{diagram_map: HashMap::new()}
    }
    fn add_line(&mut self, line: Line) {
        for (x,y) in line {
            match self.diagram_map.get(&(x,y)) {
                Some(& val) => self.diagram_map.insert((x,y), val+1),
                None => self.diagram_map.insert((x,y), 1),
            };
        }
    }

    fn get_n_obstacles(&self) -> usize {
        let mut diagram_map_copy = self.diagram_map.clone();
        diagram_map_copy.retain(|&k, v| v > &mut 1);
        diagram_map_copy.len()as usize
    }
}

fn main() {
    const NO_DIAGONAL: bool = false;

    let mut diagram = Diagram::new();
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut temp_line_coords: [i32; 4] = [0 ; 4];
    for line in reader.lines() {
        let mut arr_indx: usize = 0;
        for pair in line.unwrap().split(" -> ") {
            for num in pair.split(",") {
                if let Ok(parsed) = num.parse::<i32>() {
                    temp_line_coords[arr_indx] = parsed;
                    arr_indx += 1;
                }
            }
        }
        // Check if line is horizontal or vertical and if so add it to the diagram
        if NO_DIAGONAL {
            if (temp_line_coords[0] == temp_line_coords[2] || temp_line_coords[1] == temp_line_coords[3]) {
                let line = Line::new(temp_line_coords);
                diagram.add_line(line);
            }
        }
        else {
            let line = Line::new(temp_line_coords);
            diagram.add_line(line);
        }
    }
    println!("{}", diagram.get_n_obstacles());
}