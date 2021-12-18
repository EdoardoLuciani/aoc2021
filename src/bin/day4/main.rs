use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Board {
    // Storing number as key and position as values
    pub board_num: HashMap<u32, (u32, u32)>,
    pub current_row_num: u32,
    pub has_won: bool,
}

impl Board {
    fn new() -> Self {
        Board {
            board_num: HashMap::new(),
            current_row_num: 0,
            has_won: false,
        }
    }
    fn fill_line(&mut self, values: &Vec<u32>) {
        if values.len() != 5 {
            panic!("Numbers must be 5");
        }
        for (i, value) in values.iter().enumerate() {
            self.board_num
                .insert(*value, (self.current_row_num, i as u32));
        }
        self.current_row_num += 1;
    }
    fn is_full(&self) -> bool {
        self.current_row_num == 5 && self.board_num.len() == 25
    }
    fn call_num_and_check_bingo(&mut self, num: u32) -> Option<u32> {
        if self.has_won == false {
            if let Some(_pos) = self.board_num.remove(&num) {
                let mut cols_num = HashSet::<u32>::new();
                let mut rows_num = HashSet::<u32>::new();
                for (_, pos) in self.board_num.iter() {
                    rows_num.insert(pos.0);
                    cols_num.insert(pos.1);
                }
                if rows_num.len() != 5 || cols_num.len() != 5 {
                    self.has_won = true;
                    return Some(self.compute_score(num));
                }
            }
        }
        None
    }
    fn compute_score(&self, num: u32) -> u32 {
        let mut sum: u32 = 0;
        for (val, _) in self.board_num.iter() {
            sum += val;
        }
        return sum * num;
    }
}

fn fill_boards() -> (Vec<u32>, Vec<Board>) {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut called_numbers_str = String::new();
    reader.read_line(&mut called_numbers_str).unwrap();
    let mut out_called_numbers = Vec::<u32>::new();
    for num_str in called_numbers_str.split(",") {
        if let Ok(num) = num_str.parse::<u32>() {
            out_called_numbers.push(num);
        }
    }

    let mut out_boards = Vec::<Board>::new();
    let mut board_line_nums = Vec::<u32>::new();

    for line in reader.lines() {
        if out_boards.is_empty() || out_boards.last_mut().unwrap().is_full() {
            out_boards.push(Board::new());
        }
        board_line_nums.clear();
        for num_str in line.unwrap().split(" ") {
            if let Ok(num) = num_str.parse::<u32>() {
                board_line_nums.push(num);
            }
        }
        if board_line_nums.len() == 5 {
            out_boards.last_mut().unwrap().fill_line(&board_line_nums);
        }
    }
    (out_called_numbers, out_boards)
}

fn main() {
    let (calls, mut boards) = fill_boards();
    let boards_len = boards.len();
    let mut winner_count: u32 = 0;
    for num in calls {
        for board in boards.iter_mut() {
            if let Some(score) = board.call_num_and_check_bingo(num) {
                if winner_count == 0 {
                    println!("First board has won with score {}", score);
                }
                winner_count += 1;
                if winner_count == boards.len() as u32 {
                    println!("Last board has won with score {}", score);
                }
            }
        }
    }
}
