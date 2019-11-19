extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    // println!("With text:/n{}", contents);

    let mut contents_list: Vec<_> = contents.lines().filter(|line| !line.is_empty()).collect();

    // println!("Lines: {:?}", contents_list);

    contents_list.shuffle(&mut thread_rng());

    // println!("Lines after shuffle: {:?}", contents_list);

    for line in contents_list {
        println!("{}", line);
    }

    Ok(())
}

// pub shuffle()
