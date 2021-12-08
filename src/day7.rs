use color_eyre::Report;
use crate::common;
use tracing::info;

pub fn p1(path: &str) -> Result<i32, Report> {
    let file = common::read_file(path)?;
    let data: Vec<i32> = file.split(',').map(|x| x.parse().unwrap()).collect();
    let (&min, &max) = (data.iter().min().unwrap(), data.iter().max().unwrap());
    let mut smallest: i32 = data.iter().sum();
    for n in min..=max {
        let mut total = 0;
        for d in &data {
            total += (d - n).abs();
        }
        if total < smallest {
            smallest = total;
        }
    }
    Ok(smallest)
}
pub fn p2(path: &str) -> Result<i32, Report> {
    let file = common::read_file(path)?;
    let data: Vec<i32> = file.split(',').map(|x| x.parse().unwrap()).collect();
    let (&min, &max) = (data.iter().min().unwrap(), data.iter().max().unwrap());
    let mut smallest: i32 = i32::MAX;
    for n in min..=max {
        let mut total = 0;
        for &d in &data {
            let a = (d - n).abs();
            //info!("{}", a * (a + 1) / 2);
            total += a * (a + 1) / 2;
        }
        //info!("{}", total);
        if total < smallest {
            info!("{}", total);
            smallest = total;
        }
    }
    Ok(smallest)
}