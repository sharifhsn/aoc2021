use color_eyre::Report;
//use tracing::info;
use crate::common;

pub fn p1(path: &str) -> Result<i32, Report> {
    let data = common::read_str_file(path)?;
    let mut start: Vec<i32> = data[0].split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    for _ in 0..170 {
        let mut n = 0;
        for s in &mut start {
            if *s > 0 {
                *s -= 1;
            } else {
                *s = 6;
                n += 1;
            }
        }
        start.resize(start.len() + n, 8);
        //info!("{:?}", start);
    }
    Ok(i32::try_from(start.len()).unwrap())
}

pub fn p2(path: &str) -> Result<usize, Report> {
    //let mut data: Vec<u8> = vec![1];
    let data = common::read_str_file(path)?;
    let start: Vec<usize> = data[0].split(',').map(|s| s.parse::<usize>().unwrap()).collect();
    let magic: Vec<usize> = vec![6703087164, 6206821033, 5617089148, 5217223242, 4726100874, 4368232009, 3989468462, 3649885552, 3369186778, 3053201612];
    let mut total = 0;
    for s in start {
        total += magic[s];
    }
    // for i in 0..x {
    //     let mut n = 0;
    //     for d in &mut data {
    //         if *d > 0 {
    //             *d -= 1;
    //         } else {
    //             *d = 6;
    //             n += 1;
    //         }
    //     }
    //     for _ in 0..n {
    //         data.push(8);
    //     }
    //     info!("{}: {}", i, data.len());
    // }

    Ok(total)
}