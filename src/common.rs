use color_eyre::Report;
//use tracing::info;
use tracing_subscriber::EnvFilter;

use std::fs::File;
use std::io::Read;

pub fn log_setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

pub fn read_num_file(path: &str) -> Result<Vec<i32>, Report> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect())
}

pub fn read_str_file(path: &str) -> Result<Vec<String>, Report> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents.split('\n').map(String::from).collect())
}
