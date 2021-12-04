use color_eyre::Report;

use std::{fs::File, io::Read};

struct Board([[(i32, bool); 5]; 5]);
impl Board {
    fn is_bingo(&self) -> bool {
        for i in 0..5 {
            if (self.0[i][0].1
                && self.0[i][1].1
                && self.0[i][2].1
                && self.0[i][3].1
                && self.0[i][4].1)
                || (self.0[0][i].1
                    && self.0[1][i].1
                    && self.0[2][i].1
                    && self.0[3][i].1
                    && self.0[4][i].1)
            {
                return true;
            }
        }
        false
    }

    fn set_board(&mut self, n: i32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.0[i][j].0 == n {
                    self.0[i][j] = (self.0[i][j].0, true);
                }
            }
        }
    }

    fn unmarked_sum(&self) -> i32 {
        let mut n = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.0[i][j].1 {
                    n += self.0[i][j].0;
                }
            }
        }
        n
    }
}
struct Info3 {
    nums: Vec<i32>,
    boards: Vec<Board>,
}

pub fn p1(path: &str) -> Result<i32, Report> {
    let mut info = read_bingo(path)?;
    for n in &info.nums {
        for board in &mut info.boards {
            board.set_board(*n);
            if board.is_bingo() {
                return Ok(board.unmarked_sum() * n);
            }
        }
    }
    Ok(0)
}
pub fn p2(path: &str) -> Result<i32, Report> {
    let mut info = read_bingo(path)?;
    let sz = info.boards.len();
    let mut count = 0;
    for n in &info.nums {
        for board in &mut info.boards {
            if board.is_bingo() {
                continue;
            }
            board.set_board(*n);
            if board.is_bingo() {
                count += 1;
            }
            if count == sz {
                return Ok(board.unmarked_sum() * n);
            }
            //info!("count! {}", count);
        }
    }
    Ok(0)
    // if let Some((board, n)) = finished_boards.pop() {
    //     return Ok(board.unmarked_sum() * *n);
    // } else {
    //     Ok(0)
    // }
}

fn read_bingo(path: &str) -> Result<Info3, Report> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut bs = contents.split("\n\n");
    let nums: Vec<i32> = bs
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    //info!("{:?}", nums);
    let mut boards: Vec<Board> = Vec::new();
    for b in bs {
        let mut board = [[(0, false); 5]; 5];
        for (i, line) in b.split('\n').enumerate() {
            for (j, c) in line.split_whitespace().enumerate() {
                //info!("{}", c);
                board[i][j] = (c.parse::<i32>()?, false);
            }
        }
        boards.push(Board(board));
    }
    //info!("{:?}", boards);
    let info = Info3 { nums, boards };
    Ok(info)
}