use crate::common;
use color_eyre::Report;
use itertools::Itertools;
use std::cmp;

pub fn p1(path: &str) -> Result<i32, Report> {
    let data = common::read_str_file(path)?;
    let mut diagram = vec![[0; 1000]; 1000];
    for d in data {
        //info!("{}", d);
        let t: (&str, &str) = d.split(" -> ").collect_tuple().unwrap();
        let (x1, y1) = t.0.split(',').collect_tuple().unwrap();
        let (x2, y2) = t.1.split(',').collect_tuple().unwrap();
        let (x1, y1, x2, y2) = (
            x1.parse::<usize>()?,
            y1.parse::<usize>()?,
            x2.parse::<usize>()?,
            y2.parse::<usize>()?,
        );
        let (dx, dy) = (usize::abs_diff(x1, x2), usize::abs_diff(y1, y2));
        if dx == 0 {
            for i in cmp::min(y1, y2)..=cmp::max(y1, y2) {
                diagram[x1][i] += 1;
            }
        } else if dy == 0 {
            for i in cmp::min(x1, x2)..=cmp::max(x1, x2) {
                diagram[i][y1] += 1;
            }
        }
        //info!("{}", dx);
        //println!("{:?}", v);
    }
    let mut n = 0;
    for r in diagram {
        for c in r {
            if c >= 2 {
                n += 1;
            }
        }
    }
    //println!("{:?}", diagram);
    Ok(n)
}
pub fn p2(path: &str) -> Result<i32, Report> {
    let data = common::read_str_file(path)?;
    let mut diagram = vec![[0; 1000]; 1000];
    for d in data {
        //info!("{}", d);
        let t: (&str, &str) = d.split(" -> ").collect_tuple().unwrap();
        //info!("{}", v[0]);
        let (x1, y1) = t.0.split(',').collect_tuple().unwrap();
        let (x2, y2) = t.1.split(',').collect_tuple().unwrap();
        let (x1, y1, x2, y2) = (
            x1.parse::<usize>()?,
            y1.parse::<usize>()?,
            x2.parse::<usize>()?,
            y2.parse::<usize>()?,
        );
        let (dx, dy) = (usize::abs_diff(x1, x2), usize::abs_diff(y1, y2));
        if dx == 0 {
            for i in cmp::min(y1, y2)..=cmp::max(y1, y2) {
                diagram[x1][i] += 1;
            }
        } else if dy == 0 {
            for i in cmp::min(x1, x2)..=cmp::max(x1, x2) {
                diagram[i][y1] += 1;
            }
        } else if dx == dy {
            if x1 < x2 && y1 < y2 {
                for i in 0..=dx {
                    diagram[x1 + i][y1 + i] += 1;
                }
            } else if x1 < x2 && y1 > y2 {
                for i in 0..=dx {
                    diagram[x1 + i][y1 - i] += 1;
                }
            } else if x1 > x2 && y1 < y2 {
                for i in 0..=dx {
                    diagram[x1 - i][y1 + i] += 1;
                }
            } else if x1 > x2 && y1 > y2 {
                for i in 0..=dx {
                    diagram[x1 - i][y1 - i] += 1;
                }
            }
        }
        //info!("{}", dx);
        //println!("{:?}", v);
    }
    let mut n = 0;
    for r in diagram {
        for c in r {
            if c >= 2 {
                n += 1;
            }
        }
    }
    //println!("{:?}", diagram);
    Ok(n)
}
