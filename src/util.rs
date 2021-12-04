use std::fs::File;
use std::{env, io};
use std::io::BufRead;

pub fn read_input_file() -> impl Iterator<Item=String> {
    let filename = env::args().nth(1).expect("no input file given");
    let file = File::open(&filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|line| line.unwrap())
}

pub fn to_string_iter(strs: Vec<&'static str>) -> impl Iterator<Item=String> {
    strs.into_iter().map(|str| String::from(str)).into_iter()
}
