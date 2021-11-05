use crate::utils;

pub fn exec() -> i32 {
	let n = 999;
	utils::triangle(n / 3) * 3 + utils::triangle(n / 5) * 5 - utils::triangle(n / 15) * 15
}