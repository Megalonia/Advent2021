use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::str::FromStr;
use std::collections::VecDeque;

mod dayone;
pub use dayone::*;

const INF: u32 = u32::MAX;

fn main() {	
    day_one(2);
}

fn arg_parse() -> String {
    let args: Vec<String> = env::args().collect(); 
    let file              = File::open(args[1].clone()).unwrap();
    let mut buffer        = BufReader::new(file);
    let mut contents      = String::new();
    buffer.read_to_string(&mut contents).unwrap();
    contents
}

fn day_one(key: u32) {
    let file_content = arg_parse();
    let mut inputs = file_content.split("\n").collect::<VecDeque<&str>>();
    inputs.pop_back();

    match key {
        1 => {
                 let mut part_one = dayone::PartOne { increases: 0,
					          curr: FromStr::from_str(inputs[0]).unwrap()};
		 part_one.solve(inputs);
             }, 
        2 => {
                 let mut part_two = dayone::PartTwo {  increases: 0,
                                                   current_group: VecDeque::new(),
                                                   prev: INF };
                 part_two.solve(inputs);
             },
	_ => println!("INVALID INPUT")
    }
}


