use color_eyre::Report;
use crate::common;
use tracing::info;

fn privec(v: &Vec<Vec<u32>>) {
    info!("vec is:");
    for r in v {
        info!("{:?}", r);
    }
}

pub fn p1(path: &str) -> Result<i32, Report> {
    let file = common::read_file(path)?;
    let mut data: Vec<Vec<u32>> = Vec::new();
    let n = 2;
    info!("{}", file);
    for r in file.split('\n') {
        let mut v: Vec<u32> = Vec::new();
        for c in r.chars().map(|s| s.to_digit(10).unwrap()) {
            v.push(c);
        }
        data.push(v);
    }
    info!("{:?}", data);
    let mut flashes = 0;
    for x in 0..n {
        for i in 0..data.len() {
            for j in 0..data.len() {
                data[i][j] += 1;
                if data[i][j] > 9 {
                    flashes += flash(&mut data, (i, j));
                    //flashes += 1;
                }
            }
        }
    }

    Ok(flashes)
}
fn flash(data: &mut Vec<Vec<u32>>, pos: (usize, usize)) -> i32 {
    privec(&data);
    let mut flashes = 1;
    let (i, j) = pos;
    data[i][j] = 0;
    if let Some(r) = data.get(i - 1) {
        if let Some(c) = data.get(j - 1) {
            data[i - 1][j - 1] += 1;
            if data[i - 1][j - 1] > 9 {
                flashes += flash(data, (i - 1, j - 1));
            }
        }
    }
    if let Some(r) = data.get(i - 1) {
        if let Some(c) = data.get(j) {
            data[i - 1][j] += 1;
            if data[i - 1][j] > 9 {
                flashes += flash(data, (i - 1, j));
            }
        }
    }
    if let Some(r) = data.get(i - 1) {
        if let Some(c) = data.get(j + 1) {
            data[i - 1][j + 1] += 1;
            if data[i - 1][j + 1] > 9 {
                flashes += flash(data, (i - 1, j + 1));
            }
        }
    }
    if let Some(r) = data.get(i) {
        if let Some(c) = data.get(j - 1) {
            data[i][j - 1] += 1;
            if data[i][j - 1] > 9 {
                flashes += flash(data, (i, j - 1));
            }
        }
    }
    if let Some(r) = data.get(i) {
        if let Some(c) = data.get(j + 1) {
            data[i][j + 1] += 1;
            if data[i][j + 1] > 9 {
                flashes += flash(data, (i, j + 1));
            }
        }
    }
    if let Some(r) = data.get(i + 1) {
        if let Some(c) = data.get(j - 1) {
            data[i + 1][j - 1] += 1;
            if data[i + 1][j - 1] > 9 {
                flashes += flash(data, (i + 1, j - 1));
            }
        }
    }
    if let Some(r) = data.get(i + 1) {
        if let Some(c) = data.get(j) {
            data[i + 1][j] += 1;
            if data[i + 1][j] > 9 {
                flashes += flash(data, (i + 1, j));
            }
        }
    }
    if let Some(r) = data.get(i + 1) {
        if let Some(c) = data.get(j + 1) {
            data[i + 1][j + 1] += 1;
            if data[i + 1][j + 1] > 9 {
                flashes += flash(data, (i + 1, j + 1));
            }
        }
    }
    flashes
}
pub fn p2(path: &str) -> Result<i32, Report> {
    Ok(0)
}
