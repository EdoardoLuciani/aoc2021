use std::collections::{BinaryHeap, HashSet};
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Matrix<T> {
    mat_nums: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Default + Clone + PartialEq + std::fmt::Debug> Matrix<T> {
    fn new(rows: usize, cols: usize) -> Self {
        let mut mat_nums = Vec::<T>::new();
        mat_nums.resize(rows * cols, T::default());
        Matrix {
            mat_nums,
            rows,
            cols,
        }
    }

    fn set(&mut self, row: usize, col: usize, val: T) {
        self.mat_nums[row * self.cols + col] = val;
    }

    fn get(&self, row: usize, col: usize) -> T {
        self.mat_nums[row * self.cols + col].clone()
    }

    fn get_cross_neighbours(&self, row: usize, col: usize) -> Vec<((usize, usize), T)> {
        let positions: [(i64, i64); 4] = [
            (row as i64 - 1, col as i64),
            (row as i64 + 1, col as i64),
            (row as i64, col as i64 - 1),
            (row as i64, col as i64 + 1),
        ];

        let ret: Vec<((usize, usize), T)> = positions
            .iter()
            .filter(|(r, c)| *r > -1 && *c > -1 && *r < self.rows as i64 && *c < self.cols as i64)
            .map(|(r, c)| {
                (
                    (*r as usize, *c as usize),
                    self.get(*r as usize, *c as usize),
                )
            })
            .collect();
        ret
    }
}

impl<T: fmt::Debug> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for elem in self.mat_nums.iter().as_slice().chunks(self.cols) {
            writeln!(f, "{:?}", elem)?;
        }
        Ok(())
    }
}

fn read_input_from_file() -> Matrix<u8> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();

    let mut mat = Matrix::<u8>::new(lines.len(), lines[0].len());

    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            let num = ch.to_digit(10).unwrap() as u8;
            mat.set(i, j, num);
        }
    }
    mat
}

fn read_input_from_file_part2() -> Matrix<u8> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();

    let mut mat = Matrix::<u8>::new(lines.len() * 5, lines[0].len() * 5);

    for q1 in 0..5_usize {
        for q2 in 0..5_usize {
            for (i, line) in lines.iter().enumerate() {
                for (j, ch) in line.chars().enumerate() {
                    let num = ch.to_digit(10).unwrap() as u8;
                    mat.set(
                        i + q2 * lines.len(),
                        j + q1 * line.len(),
                        ((num - 1 + q1 as u8 + q2 as u8) % 9) + 1,
                    );
                }
            }
        }
    }
    mat
}

fn dijkstra_shortest_path(mat: &Matrix<u8>) -> Matrix<u64> {
    let starting_node: (usize, usize) = (0, 0);

    // location, distance
    let mut distances = Matrix::<u64>::new(mat.rows, mat.cols);
    distances.mat_nums.iter_mut().for_each(|s| *s = u64::MAX);
    distances.set(starting_node.0, starting_node.1, 0);

    // negated distance to get a min-heap BECAUSE rust does not have ANY WAY to fucking specify order in BinaryHeap, location
    let mut shortest_dis_queue = BinaryHeap::<(i64, (usize, usize))>::new();
    shortest_dis_queue.push((0, (0, 0)));

    // location
    let mut visited = HashSet::<(usize, usize)>::new();

    while let Some((_, node)) = shortest_dis_queue.pop() {
        visited.insert(node);
        for (neighbour_coord, neighbour_val) in mat.get_cross_neighbours(node.0, node.1) {
            if !visited.contains(&neighbour_coord) {
                let new_distance = distances.get(node.0, node.1) + neighbour_val as u64;
                if new_distance < distances.get(neighbour_coord.0, neighbour_coord.1) {
                    distances.set(neighbour_coord.0, neighbour_coord.1, new_distance);
                    shortest_dis_queue.push((-(new_distance as i64), neighbour_coord));
                }
            }
        }
    }
    distances
}

fn main() {
    let mat = read_input_from_file_part2();
    let distances = dijkstra_shortest_path(&mat);
    println!("{}", *distances.mat_nums.last().unwrap());
}
