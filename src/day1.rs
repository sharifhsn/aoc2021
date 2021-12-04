use crate::common;
use color_eyre::Report;
//use tracing::info;

pub fn p1(path: &str) -> Result<i32, Report> {
    let numbers = common::read_num_file(path)?;

    let mut i = 0;

    for [a, b] in numbers.array_windows() {
        if b > a {
            i += 1;
        }
    }

    Ok(i)
}

pub fn p2(path: &str) -> Result<i32, Report> {
    let numbers = common::read_num_file(path)?;
    let mut v: Vec<i32> = Vec::new();
    let mut i = 0;
    for [a, b, c] in numbers.array_windows() {
        v.push(a + b + c);
    }
    for [a, b] in v.array_windows() {
        if b > a {
            i += 1;
        }
    }
    Ok(i)
}
