use color_eyre::Report;
use tracing::info;
use crate::common;
use std::collections::HashMap;

pub fn p1(path: &str) -> Result<i32, Report> {
    let s = common::read_file(path)?;
    let lines: Vec<&str> = s.split('\n').map(|l| l.split(" | ").collect::<Vec<&str>>()[1]).collect();
    let mut n = 0;
    for l in lines {
        let pats = l.split(' ');
        for p in pats {
            match p.len() {
                2 | 3 | 4 | 7 => n += 1,
                _ => {}
            }
        }
    }
    Ok(n)
}

// 2: 1
// 3: 7
// 4: 4
// 5: 2, 3, 5
// 6: 0, 6, 9
// 7: 8

//  0000
// 1    2
// 1    2
// 1    2
//  3333
// 4    5
// 4    5
// 4    5
//  6666

// fittings:
// everything fits in 8
// 0 6 9 fit in nothing (obviously)
// 1 fits in 0 3 4   7 9
// 7 fits in 0 3       9
// 4 fits in           9
// 2 fits in nothing
// 3 fits in           9
// 5 fits in       6   9
// nothing fits in 2 and 5

// cfgeda afcbg fbcdaeg gbdfa fdgcba cbdf becga cf gcf gbdefa | bfcd acgdfe cfabg dcbf
// cbdf is 4, so size 6 with it is 9 [c, b, d, f] : [1, 2, 3, 5]
// cf is 1, so size 5 with it is 3 [c, f] : [2, 5] -> [b, d] : [1, 3]
// gcf is 7, so size 5 with it is 3 | g : 0
// cfabg is size 5 with gcf, so it is 3 | [a, b] : [3, 6] -> a : 6, b : 3, d : 1
// 


pub fn p2(path: &str) -> Result<i32, Report> {
    let s = common::read_file(path)?;
    let lines: Vec<&str> = s.split('\n').collect();
    for &l in &lines {
        let mut spl = l.split(" | ");
        let sigs = spl.next().unwrap().split(' ').collect::<Vec<&str>>();
        let outp = spl.next().unwrap().split(' ').collect::<Vec<&str>>();
        //info!("{:?}: {:?}", sigs, outp);
        let mut sigmap = sig_init();
        for sig in sigs {
            match sig.len() {
                2 => {
                    for c in sig.chars() {
                        sigmap.entry(c).or_insert(vec![2, 5]).retain(|&x| x == 2 || x == 5);
                    }
                },
                3 => {
                    for c in sig.chars() {
                        sigmap.entry(c).or_insert(vec![0, 2, 5]).retain(|&x| x == 0 || x == 2 || x == 5);
                    }
                },
                4 => {
                    for c in sig.chars() {
                        sigmap.entry(c).or_insert(vec![1, 2, 3, 5]).retain(|&x| x == 1 || x == 2 || x == 3 || x == 5);
                    }
                },
                _ => {}
            }
        }
        info!("{:?}", sigmap);
        //info!("{:?}", ans);
    }
    //info!("{:?}", lines);
    // let mut n = 0;
    // for l in lines {
    //     let pats = l.split(' ');
    //     for p in pats {
    //         match p.len() {
    //             2 | 3 | 4 | 7 => n += 1,
    //             _ => {}
    //         }
    //     }
    // }
    Ok(0)
}

fn sig_init() -> HashMap<char, Vec<u8>> {
    let mut sigmap: HashMap<char, Vec<u8>> = HashMap::new();
    sigmap.insert('a', (0..8).collect());
    sigmap.insert('b', (0..8).collect());
    sigmap.insert('c', (0..8).collect());
    sigmap.insert('d', (0..8).collect());
    sigmap.insert('e', (0..8).collect());
    sigmap.insert('f', (0..8).collect());
    sigmap
}

fn sigs_found(sigmap: &HashMap<char, Vec<u8>>) -> bool {
    for (_, v) in sigmap {
        if v.len() > 1 {
            return false;
        }
    }
    true
}

//fn print_sigs(sigmap: &HashMap<char,)