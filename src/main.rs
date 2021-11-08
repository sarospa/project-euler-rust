#![allow(non_snake_case)]

use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use colored::*;

fn print_answer(answer: &str, id: usize, keys: &Vec<String>) {
	if keys.len() >= id {
		if answer == &keys[id - 1][..] {
			println!("{problem}: {result}", problem = format!("{:0>3}", id.to_string()), result = "[PASS]".bright_green());
		}
		else {
			println!("{problem}: {result} (expected {right}, got {wrong})", problem = format!("{:0>3}", id.to_string()), result = "[FAIL]".bright_red(), right = keys[id - 1], wrong = answer);
		}
	}
	else {
		println!("{problem}: {answer}", problem = format!("{:0>3}", id.to_string()), answer = answer);
	}
}

fn main() {
    let args: Vec<String> = env::args().collect();
	let input_path = Path::new(&env!("OUT_DIR")).join("key.txt");
	let file = File::open(input_path).expect("no such file");
	let buf = BufReader::new(file);
	let lines: Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
	
	let id: usize;
	if args.len() == 2 {
		id = args[1].parse::<usize>().unwrap();
	}
	else { id = 0; }
	
	if id == 0 || id == 1 {
		print_answer(&euler_001::exec().to_string(), 1, &lines);
	}
	if id == 0 || id == 2 {
		print_answer(&euler_002::exec().to_string(), 2, &lines);
	}
	if id == 0 || id == 3 {
		print_answer(&euler_003::exec().to_string(), 3, &lines);
	}
	if id == 0 || id == 4 {
		print_answer(&euler_004::exec().to_string(), 4, &lines);
	}
	if id == 0 || id == 5 {
		print_answer(&euler_005::exec().to_string(), 5, &lines);
	}
	if id == 0 || id == 6 {
		print_answer(&euler_006::exec().to_string(), 6, &lines);
	}
	if id == 0 || id == 7 {
		print_answer(&euler_007::exec().to_string(), 7, &lines);
	}
	if id == 0 || id == 8 {
		print_answer(&euler_008::exec().to_string(), 8, &lines);
	}
	if id == 0 || id == 9 {
		print_answer(&euler_009::exec().to_string(), 9, &lines);
	}
	if id == 0 || id == 10 {
		print_answer(&euler_010::exec().to_string(), 10, &lines);
	}
}

mod utils;
mod euler_001;
mod euler_002;
mod euler_003;
mod euler_004;
mod euler_005;
mod euler_006;
mod euler_007;
mod euler_008;
mod euler_009;
mod euler_010;