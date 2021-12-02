use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::str::FromStr;
use std::collections::VecDeque;

mod dayone;
pub use dayone::*;

mod daytwo;
pub use daytwo::*;

const INF: u32 = u32::MAX;

fn main() {	
    day_two();
}

fn arg_parse() -> String {
    let args: Vec<String> = env::args().collect(); 
    let file              = File::open(args[1].clone()).unwrap();
    let mut buffer        = BufReader::new(file);
    let mut contents      = String::new();
    buffer.read_to_string(&mut contents).unwrap();
    contents
}

fn day_two() {
    let file_content = arg_parse();
    let mut inputs = file_content.split("\n").collect::<VecDeque<&str>>();
    inputs.pop_back();
    let mut obj = daytwo::DayTwo{ horizontal_position: 0,
                                       depth: 0,
                                       aim: 0          };
    obj.pilot_part_two(inputs);
    obj.display();
}


