use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Formatter, Write};
use std::fs::{read, File};
use std::io::{BufRead, BufReader};
use std::ops::Deref;

struct Matrix<T> {
    mat_nums: Vec<T>,
    cols: usize,
    rows: usize,
}

impl<T: Default + Clone + PartialEq> Matrix<T> {
    fn new(cols: usize, rows: usize) -> Self {
        let mut mat_nums = Vec::<T>::new();
        mat_nums.resize(rows * cols, T::default());
        Matrix {
            mat_nums,
            cols,
            rows,
        }
    }

    fn set(&mut self, col: usize, row: usize, val: T) {
        self.mat_nums[row * self.cols + col] = val;
    }

    fn get(&self, col: usize, row: usize) -> T {
        self.mat_nums[row * self.cols + col].clone()
    }

    fn mirror_over_rows(&mut self, y_line: usize, val: T) {
        for col in 0..self.cols {
            for row in y_line..self.rows {
                if self.get(col, row) != T::default() {
                    self.set(col, y_line - (row - y_line), val.clone());
                    self.set(col, row, T::default());
                }
            }
        }
    }

    fn mirror_over_cols(&mut self, x_line: usize, val: T) {
        for row in 0..self.rows {
            for col in x_line..self.cols {
                if self.get(col, row) != T::default() {
                    self.set(x_line - (col - x_line), row, val.clone());
                    self.set(col, row, T::default());
                }
            }
        }
    }

    fn count_equal(&self, val: T) -> usize {
        self.mat_nums
            .iter()
            .map(|e| if *e == val { 1 } else { 0 })
            .sum()
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

impl<T: fmt::Display + Default + Clone + PartialEq> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut first_coord: (usize, usize) = (0, 0);
        'outer1: for row in 0..self.rows {
            for col in 0..self.cols {
                if self.get(col, row) != T::default() {
                    first_coord = (col, row);
                    break 'outer1;
                }
            }
        }

        let mut second_coord: (usize, usize) = (0, 0);
        'outer2: for row in (0..self.rows).rev() {
            for col in (0..self.cols).rev() {
                if self.get(col, row) != T::default() {
                    second_coord = (col, row);
                    break 'outer2;
                }
            }
        }

        for row in (first_coord.1..=second_coord.1) {
            for col in (first_coord.0..=second_coord.0) {
                let c = match self.get(col, row) {
                    n if n != T::default() => '■',
                    _ => '.',
                };
                f.write_char(c)?
            }
            f.write_char('\n')?
        }
        Ok(())
    }
}

fn read_input_from_file() -> (Matrix<u8>, Vec<String>) {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(&file);

    let (folds, coordinates): (Vec<String>, Vec<String>) = reader
        .lines()
        .map(|s| s.unwrap())
        .partition(|s| s.starts_with("fold along"));

    let coordinates: Vec<(usize, usize)> = coordinates
        .iter()
        .filter_map(|s| s.split_once(','))
        .map(|s| (s.0.parse().unwrap(), s.1.parse().unwrap()))
        .collect();

    let rows = coordinates.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
    let cols = coordinates.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;
    let mut mat = Matrix::<u8>::new(rows + 1, cols + 1);

    for coordinate in coordinates.iter() {
        mat.set(coordinate.0, coordinate.1, 1);
    }

    (mat, folds)
}

fn execute_folds(mat: &mut Matrix<u8>, folds: &[String]) {
    for fold in folds.iter() {
        let (dir, coord) = fold.get(11..fold.len()).unwrap().split_once('=').unwrap();
        let coord = coord.parse::<usize>().unwrap();
        match dir {
            "y" => mat.mirror_over_rows(coord, 1),
            "x" => mat.mirror_over_cols(coord, 1),
            _ => {}
        }
    }
}

fn main() {
    let (mut mat, folds) = read_input_from_file();

    let folds_to_execute_count = folds.len();
    execute_folds(&mut mat, &folds);

    println!("{}", mat);
    println!("{}", mat.count_equal(1));
}
