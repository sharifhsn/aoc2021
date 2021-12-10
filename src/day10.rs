use color_eyre::Report;
use tracing::info;
use crate::common;

pub fn p1(path: &str) -> Result<i32, Report> {
    let file = common::read_file(path)?;
    let data: Vec<&str> = file.split('\n').collect();
    let mut total = 0;
    //info!("cordata len: {}", cordata.len());
    //let mut data: Vec<&str> = Vec::new();
    'outer: for d in data {
        let mut stk: Vec<char> = Vec::new();
        for &c in d.as_bytes() {
            match char::from(c) {
                '(' | '[' | '{' | '<' => stk.push(char::from(c)),
                ')' => {
                    match stk.pop() {
                        Some('(') => {},
                        _ => {
                            total += 3;
                            continue 'outer;
                        }
                    }
                }
                ']' => {
                    match stk.pop() {
                        Some('[') => {},
                        _ => {
                            total += 57;
                            continue 'outer;
                        }
                    }
                }
                '}' => {
                    match stk.pop() {
                        Some('{') => {},
                        _ => {
                            total += 1197;
                            continue 'outer;
                        }
                    }
                }
                '>' => {
                    match stk.pop() {
                        Some('<') => {},
                        _ => {
                            total += 25137;
                            continue 'outer;
                        }
                    }
                }
                _ => {}
            }
        }
        //data.push(d);
    }
    //info!("fixed {:?}", data.len());
    Ok(total)
}

pub fn p2(path: &str) -> Result<u128, Report> {
    let file = common::read_file(path)?;
    let cordata: Vec<&str> = file.split('\n').collect();
    //info!("cordata len: {}", cordata.len());
    let mut data: Vec<Vec<char>> = Vec::new();
    'outer: for d in cordata {
        let mut stk: Vec<char> = Vec::new();
        for &c in d.as_bytes() {
            match char::from(c) {
                '(' | '[' | '{' | '<' => stk.push(char::from(c)),
                ')' => {
                    match stk.pop() {
                        Some('(') => {},
                        _ => {
                            continue 'outer;
                        }
                    }
                }
                ']' => {
                    match stk.pop() {
                        Some('[') => {},
                        _ => {
                            continue 'outer;
                        }
                    }
                }
                '}' => {
                    match stk.pop() {
                        Some('{') => {},
                        _ => {
                            continue 'outer;
                        }
                    }
                }
                '>' => {
                    match stk.pop() {
                        Some('<') => {},
                        _ => {
                            continue 'outer;
                        }
                    }
                }
                _ => {}
            }
        }
        data.push(stk);
    }
    let mut scores: Vec<u128> = Vec::new();
    for mut stk in data {
        let mut score = 0;
        while !stk.is_empty() {
            score *= 5;
            match stk.pop() {
                Some('(') => score += 1,
                Some('[') => score += 2,
                Some('{') => score += 3,
                Some('<') => score += 4,
                _ => {}
            }
        }
        scores.push(score);
    }
    scores.sort_unstable();
    info!("scores are {:?}", scores);
    Ok(scores[scores.len() / 2])
}