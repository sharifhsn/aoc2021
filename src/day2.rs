use color_eyre::Report;

use crate::common;

pub fn p1(path: &str) -> Result<i32, Report> {
    let data = common::read_str_file(path)?;
    let r = 0..data.len() - 1;
    let mut depth = 0;
    let mut horiz = 0;
    for i in r.into_iter().step_by(2) {
        // info!("{} {}", data[i], data[i + 1]);
        let n: i32 = data[i + 1].parse()?;
        match data[i].as_str() {
            "forward" => horiz += n,
            "up" => depth -= n,
            "down" => depth += n,
            _ => {}
        }
    }
    Ok(depth * horiz)
}

pub fn p2(path: &str) -> Result<i32, Report> {
    let data = common::read_str_file(path)?;
    let r = 0..data.len() - 1;
    let mut depth = 0;
    let mut horiz = 0;
    let mut aim = 0;
    for i in r.into_iter().step_by(2) {
        // info!("{} {}", data[i], data[i + 1]);
        let n: i32 = data[i + 1].parse()?;
        match data[i].as_str() {
            "forward" => {
                horiz += n;
                depth += n * aim;
            }
            "up" => aim -= n,
            "down" => aim += n,
            _ => {}
        }
    }
    Ok(depth * horiz)
}