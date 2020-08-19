use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;


use std::env;
use std::str;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // reading from a file
    let buffered_reader = BufReader::new(File::open(&args[1])?);

    // finds the base64 equivalent on https://
    let base64_url_regex : Regex = Regex::new(r"aHR0cHM6Ly9").unwrap();

    buffered_reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| base64_url_regex.is_match(line.as_str()))
        .for_each(|x| open_base64(&x));

    Ok(())
}

fn open_base64 (input : &str) {
    // converst input to ascii
    let input = convert_to_ascii(input);
    // print out for debug
    println!("{:?}", input);
    // system open call unused result
    let _ = open::that(input);
}

fn convert_to_ascii(input: &str) -> String {
    // decode
    let temp = base64::decode(input).unwrap();

    // reencode to utf8
    str::from_utf8(&temp).unwrap().to_string()
}