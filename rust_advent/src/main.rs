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

mod daythree;
pub use daythree::*;

const INF: u32 = u32::MAX;

fn main() {	
    let file_content = arg_parse();
    let mut inputs = file_content.split("\n").collect::<VecDeque<&str>>();
    inputs.pop_back();
    let mut obj = daythree::BinaryDiagnostic{ gamma_rate   : 0,
                                              epsilon_rate : 0,
                                              common_bits  : std::iter::repeat((0,0)).take(inputs[0].len()).collect::<Vec<_>>()
                                            };
    obj.power_consumption(inputs);

}

fn arg_parse() -> String {
    let args: Vec<String> = env::args().collect(); 
    let file              = File::open(args[1].clone()).unwrap();
    let mut buffer        = BufReader::new(file);
    let mut contents      = String::new();
    buffer.read_to_string(&mut contents).unwrap();
    contents
}
