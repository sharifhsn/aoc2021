use color_eyre::Report;
//use tracing::info;

use crate::common;

pub fn p1(path: &str) -> Result<i32, Report> {
    let data = common::read_str_file(path)?;
    let mut depth = 0;
    let mut horiz = 0;
    //info!("{:?}", data);
    for d in data {
        let info: Vec<&str> = d.split_whitespace().collect();
        // info!("{} {}", data[i], data[i + 1]);
        let n: i32 = info[1].parse()?;
        match info[0] {
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
    let mut depth = 0;
    let mut horiz = 0;
    let mut aim = 0;
    for d in data {
        // info!("{} {}", data[i], data[i + 1]);
        let info: Vec<&str> = d.split_whitespace().collect();
        let n: i32 = info[1].parse()?;
        match info[0] {
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
