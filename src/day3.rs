use crate::common;
use color_eyre::Report;

pub fn p1(path: &str) -> Result<i32, Report> {
    let data = common::read_str_file(path)?;
    let mut zeros = [0; 12];
    let mut ones = [0; 12];
    for d in data {
        for (i, c) in d.chars().enumerate() {
            match c {
                '0' => zeros[i] += 1,
                '1' => ones[i] += 1,
                _ => {}
            }
        }
    }
    let mut gamma = String::new();
    for (z, o) in zeros.zip(ones) {
        if z > o {
            gamma.push('0');
        } else {
            gamma.push('1');
        }
    }
    let mut epsilon = String::new();
    for (z, o) in zeros.zip(ones) {
        if z < o {
            epsilon.push('0');
        } else {
            epsilon.push('1');
        }
    }
    Ok(i32::from_str_radix(gamma.as_str(), 2)? * i32::from_str_radix(epsilon.as_str(), 2)?)
}

pub fn p2(path: &str) -> Result<i32, Report> {
    let data = common::read_str_file(path)?;
    let mut zeros = [0; 12];
    let mut ones = [0; 12];
    for d in &data {
        for (i, c) in d.chars().enumerate() {
            match c {
                '0' => zeros[i] += 1,
                '1' => ones[i] += 1,
                _ => {}
            }
        }
    }
    let mut gamma = String::new();
    //info!("{:?}", zeros.zip(ones));
    for (z, o) in zeros.zip(ones) {
        if z > o {
            gamma.push('0');
        } else {
            gamma.push('1');
        }
    }
    let mut epsilon = String::new();
    for (z, o) in zeros.zip(ones) {
        if z <= o {
            epsilon.push('0');
        } else {
            epsilon.push('1');
        }
    }
    // let mut zeroes: Vec<i32> = Vec::new();
    // let mut ones: Vec<i32> = Vec::new();
    // let mut count = String::new();
    let mut oxy = data;
    for i in 0..12 {
        let mut zeros = [0; 12];
        let mut ones = [0; 12];
        for d in &oxy {
            for (i, c) in d.chars().enumerate() {
                match c {
                    '0' => zeros[i] += 1,
                    '1' => ones[i] += 1,
                    _ => {}
                }
            }
        }
        let mut gamma = String::new();
        //info!("{:?}", zeros.zip(ones));
        for (z, o) in zeros.zip(ones) {
            if z <= o {
                gamma.push('0');
            } else {
                gamma.push('1');
            }
        }
        let mut j = 0;
        while j < oxy.len() {
            if oxy[j].as_bytes()[i] != gamma.as_bytes()[i] {
                oxy.remove(j);
            } else {
                j += 1;
            }
        }
        //info!("matches {}: {:?}", &gamma[..i], oxy);
        if oxy.len() == 1 {
            break;
        }
    } // 011001010000
      // 101110111101

    //info!("gamma {:?}", gamma);

    Ok(0)
}
