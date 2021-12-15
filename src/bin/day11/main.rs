use std::collections::HashSet;
use std::fmt;
use std::fmt::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Matrix<T> {
    mat_nums: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix{mat_nums : Vec::<T>::new(), rows, cols}
    }

    pub fn append(&mut self, val: T) -> bool {
        if self.mat_nums.len() < (self.rows * self.cols) {
            self.mat_nums.push(val);
            return true;
        }
        false
    }

    pub fn get(&mut self, row : i64, col : i64) -> Option<&mut T> {
        if row < self.rows as i64 && row > -1 && col < self.cols as i64 && col > -1 {
            return self.mat_nums.get_mut((row * self.cols as i64 + col) as usize);
        }
        None
    }
}

impl<T: std::fmt::Debug> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.mat_nums.len() != self.rows*self.cols {
            return fmt::Result::Err(Error);
        }
        for rn in 0..self.rows {
            for cn in 0..self.cols {
                write!(f, "{:?} ", self.mat_nums[rn * self.cols as usize + cn])?
            }
            f.write_str("\n")?
        }
        fmt::Result::Ok(())
    }

}

impl Matrix<u8> {
    fn compute_steps(&mut self, steps : u32) -> usize {
        let mut total_flashes : usize = 0;
        for _ in 0..steps {
            self.mat_nums.iter_mut().for_each(|v| *v += 1);
            let mut flashed = HashSet::<(i64, i64)>::new();
            for rn in 0..self.rows as i64 {
                for cn in 0..self.cols as i64 {
                    self.check_and_flash(rn, cn, &mut flashed);
                }
            }
            total_flashes += flashed.len();
        }
        total_flashes
    }

    fn check_and_flash(&mut self, row : i64, col : i64, flashed: &mut HashSet<(i64, i64)>) {
        if let Some(val) = self.get(row, col) {
            if *val > 9 {
                *val = 0;
                flashed.insert((row, col));
                let positions: [(i64, i64); 8] = [(row, col+1), (row, col-1), (row-1, col), (row+1, col),
                    (row-1, col-1), (row+1, col-1), (row-1, col+1), (row+1, col+1)];
                for (row_pos, col_pos) in positions {
                    if !flashed.contains(&(row_pos, col_pos)) {
                        if let Some(v) = self.get(row_pos, col_pos) {
                            *v += 1;
                            self.check_and_flash(row_pos, col_pos, flashed);
                        }
                    }
                }
            }
        }
    }

    fn compute_steps_until_all_equal(&mut self) -> u64 {
        let mut step : u64 = 0;
        while self.mat_nums.iter().any(|v| *v != self.mat_nums[0]) {
            self.compute_steps(1);
            step += 1;
        }
        step
    }
}

fn read_input_from_file() -> Matrix<u8> {
    let mut mat = Matrix::<u8>::new(10, 10);
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        for ch in line.unwrap().chars() {
            mat.append(ch.to_digit(10).unwrap() as u8);
        }
    }
    mat
}

fn main() {
    let mut input = read_input_from_file();
    println!("{}", input.compute_steps(100));
    //println!("{}", input.compute_steps_until_all_equal());
    //println!("{:?}", input);
}