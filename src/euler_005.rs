pub fn exec() -> i32 {
	const TARGET: usize = 20;
	let mut values: [i32; TARGET] = [0; TARGET];
	for i in 0..values.len() { values[i] = i as i32 + 1; }
	let mut result: i32 = 1;
	let mut prime: i32 = 2;
	while prime <= TARGET as i32 {
		let mut divides = false;
		for i in 0..values.len() {
			if values[i] % prime == 0 {
				values[i] /= prime;
				divides = true;
			}
		}
		if divides {
			result *= prime;
		}
		else {
			prime += 1;
		}
	}
	result
}