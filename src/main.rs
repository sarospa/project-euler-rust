use std::env;

mod utils;
mod euler_001;

fn main() {
    let args: Vec<String> = env::args().collect();
	let id: i32;
	if args.len() == 1 {
		id = args[0].parse::<i32>().unwrap();
	}
	else { id = 0; }
	
	
	if id == 0 || id == 1 {
		println!("{}", euler_001::exec());
	}
}
