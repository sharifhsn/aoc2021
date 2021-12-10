use color_eyre::Report;
use tracing::info;
use crate::common;

pub fn p1(path: &str) -> Result<u32, Report> {
    let file = common::read_file(path)?;
    let mut data: Vec<Vec<u32>> = Vec::new();
    let mut total = 0;
    for d in file.split('\n') {
        data.push(d.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            let c = data[i][j];
            //info!("considering c: {}", c);
            if let Some(x) = data.get(i - 1) {
                if let Some(y) = x.get(j) {
                    //info!("comparing y: {} and c: {}", y, c);
                    if y <= &c {
                        continue;
                    }
                }
            }
            if let Some(x) = data.get(i + 1) {
                if let Some(y) = x.get(j) {
                    //Info!("comparing y: {} and c: {}", y, c);
                    if y <= &c {
                        continue;
                    }
                }
            }
            if let Some(x) = data.get(i) {
                if let Some(y) = x.get(j - 1) {
                    //info!("comparing y: {} and c: {}", y, c);
                    if y <= &c {
                        continue;
                    }
                }
            }
            if let Some(x) = data.get(i) {
                if let Some(y) = x.get(j + 1) {
                    //info!("comparing y: {} and c: {}", y, c);
                    if y <= &c {
                        continue;
                    }
                }
            }
            //info!("c is lowest point!");
            total += c + 1;
        }
    }
    //info!("{:?}", data);
    Ok(total)
}

// iterate through every element
// recursively add together every element in the basin
// by checking if it's a nine or not
// when done, mark the initial element in a vector
// if you encounter that element when calculating a basin,
// that means you've already found it and you can throw it away
pub fn p2(path: &str) -> Result<u128, Report> {
    let file = common::read_file(path)?;
    let mut data: Vec<Vec<u128>> = Vec::new();
    for d in file.split('\n') {
        data.push(d.chars().map(|x| u128::from(x.to_digit(10).unwrap())).collect());
    }
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut basins: Vec<u128> = Vec::new();
    //let n = get_basin(&data, &mut visited, (0, 0));
    for i in 0..100 {
        for j in 0..100 {
            if data[i][j] != 9 && !visited.contains(&(i, j)) {
                basins.push(get_basin(&data, &mut visited, (i, j)));
            }
        }
    }
    //let mut basins = vec![35, 102, 2, 878, 7, 234, 100, 473];
    basins.sort_unstable();
    basins.reverse();
    //info!("{:?}", basins.len());
    Ok(basins[0] * basins[1] * basins[2])
    // Ok(n)
}
// too high 1392160

fn get_basin(data: &[Vec<u128>], visited: &mut Vec<(usize, usize)>, pos: (usize, usize)) -> u128 {
    //info!("now searching {:?}", pos);
    let (i, j) = pos;
    let mut count = 1;
    visited.push(pos);
    //info!("visited is {:?}", visited);
    if let Some(x) = data.get(i - 1) {
        if let Some(y) = x.get(j) {
            //info!("comparing y: {} and c: {}", y, c);
            if y != &9 && !visited.contains(&(i - 1, j)) {
                count += get_basin(data, visited, (i - 1, j));
            }
        }
    }
    if let Some(x) = data.get(i + 1) {
        if let Some(y) = x.get(j) {
            //info!("comparing y: {} and c: {}", y, c);
            if y != &9 && !visited.contains(&(i + 1, j)) {
                count += get_basin(data, visited, (i + 1, j));
            }
        }
    }
    if let Some(x) = data.get(i) {
        if let Some(y) = x.get(j - 1) {
            //info!("comparing y: {} and c: {}", y, c);
            if y != &9 && !visited.contains(&(i, j - 1)) {
                count += get_basin(data, visited, (i, j - 1));
            }
        }
    }
    if let Some(x) = data.get(i) {
        if let Some(y) = x.get(j + 1) {
            //info!("comparing y: {} and c: {}", y, c);
            if y != &9 && !visited.contains(&(i, j + 1)) {
                count += get_basin(data, visited, (i, j + 1));
            }
        }
    }
    count
}