use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;


use std::env;
use std::str;


fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("Arguments are required. Pick 1 to read from commandline, 2 to read from file");
        },
        // one arg
        2 => {
            println!("One argument detected, defaulting to command line reading");
            open_base64(&args[1]);
        },
        // two args
        3 => {
            let option = &args[1];
            let input = &args[2];

            let option = match option.parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Error first argument not an integer");
                    help();
                    return;
                }
            };

            match option {
                1 => open_base64(input),
                2 => read_file(input),
                _ => panic!("Unknown option")
            };
        }
        _ => println!("Too many arguments"),
    };
}

fn read_file (input: &str) {
    // unused error
    let _ = read_file_actual(input);
}

fn read_file_actual (input: &str) -> std::io::Result<()> {
    // reading from a file
    let buffered_reader = BufReader::new(File::open(input)?);

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
    let input = convert_to_utf8(input);
    // print out for debug
    println!("{:?}", input);
    // system open call unused result
    let _ = open::that(input);
}

fn convert_to_utf8(input: &str) -> String {
    // decode
    let temp = base64::decode(input).unwrap();

    // reencode to utf8
    str::from_utf8(&temp).unwrap().to_string()
}

fn help () {
    println!("This is base 64 decoder and opener");
    println!("It opens any base 64 encoded https link");
    println!("There are two ways to use this program: 
    1) Reading from commandline 2) Reading from File");
    println!("Possible options: ");
    println!("Directly pass in a base 64 string to commandline to just open it in browser");
    println!("Pass the arguments 2 and the file path to open the file and every base 64 encoded URL in it");
    println!("====================================");
    println!("Argument Examples: ");
    println!("1 <base64string>");
    println!("2 <filepath>");
    println!("<base64string>");
}