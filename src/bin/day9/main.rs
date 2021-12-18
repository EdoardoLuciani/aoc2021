use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Matrix {
    mat_nums: Vec<u8>,
    rows: i32,
    cols: i32,
}

impl Matrix {
    fn new(rows: u32, cols: u32) -> Self {
        Matrix {
            mat_nums: Vec::new(),
            rows: rows as i32,
            cols: cols as i32,
        }
    }

    fn get_element(&self, row_n: i32, col_n: i32) -> Option<u8> {
        if row_n < self.rows && row_n > -1 && col_n < self.cols && col_n > -1 {
            return Some(
                *(self
                    .mat_nums
                    .get((row_n * self.cols + col_n) as usize)
                    .unwrap()),
            );
        }
        None
    }

    fn is_low_point(&self, row_n: i32, col_n: i32) -> Option<u8> {
        let num_center = self.get_element(row_n, col_n).unwrap();
        let positions: [(i32, i32); 4] = [
            (row_n, col_n + 1),
            (row_n, col_n - 1),
            (row_n - 1, col_n),
            (row_n + 1, col_n),
        ];
        for (row, col) in positions {
            if let Some(num) = self.get_element(row, col) {
                if num_center >= num {
                    return None;
                }
            }
        }
        Some(num_center)
    }

    fn get_basin_points(&self, row_n: i32, col_n: i32) -> HashSet<(i32, i32)> {
        let mut points = HashSet::<(i32, i32)>::new();
        points.insert((row_n, col_n));
        self.get_basin_points_rec_step(row_n, col_n, &mut points);
        points
    }

    fn get_basin_points_rec_step(
        &self,
        row_n: i32,
        col_n: i32,
        basin_points: &mut HashSet<(i32, i32)>,
    ) {
        let num_center = self.get_element(row_n, col_n).unwrap();
        let positions: [(i32, i32); 4] = [
            (row_n, col_n + 1),
            (row_n, col_n - 1),
            (row_n - 1, col_n),
            (row_n + 1, col_n),
        ];
        for (row, col) in positions {
            if let Some(num) = self.get_element(row, col) {
                if num != 9 && num_center < num {
                    basin_points.insert((row, col));
                    self.get_basin_points_rec_step(row, col, basin_points);
                }
            }
        }
    }

    fn add_row(&mut self, row: &Vec<u8>) {
        if row.len() != self.cols as usize {
            panic!("Row must fit into the matrix")
        } else {
            self.mat_nums.extend(row);
        }
    }
}

fn get_low_points_sum(mat: &Matrix) -> u32 {
    let mut risk_level_sum: u32 = 0;
    for row_idx in 0..mat.rows {
        for col_idx in 0..mat.cols {
            if let Some(val) = mat.is_low_point(row_idx, col_idx) {
                risk_level_sum += (val + 1) as u32;
            }
        }
    }
    risk_level_sum
}

fn get_basin_sizes(mat: &Matrix) -> Vec<u32> {
    let mut basin_sizes = Vec::<u32>::new();
    for row_idx in 0..mat.rows {
        for col_idx in 0..mat.cols {
            if let Some(val) = mat.is_low_point(row_idx, col_idx) {
                basin_sizes.push(mat.get_basin_points(row_idx, col_idx).len() as u32);
            }
        }
    }
    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));
    basin_sizes
}

fn read_input_from_file() -> Matrix {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut matrix = Matrix::new(lines.len() as u32, lines.get(0).unwrap().len() as u32);
    for line in lines.iter() {
        let nums: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        matrix.add_row(&nums);
    }
    matrix
}

fn main() {
    let matrix = read_input_from_file();
    println!("{}", get_low_points_sum(&matrix));
    let mut basin_sizes = get_basin_sizes(&matrix);
    println!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}
