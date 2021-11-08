pub fn exec() -> i32 {
	const TARGET: i32 = 10001;
	let mut n = 1;
	let mut count = 0;
	while count < TARGET {
		n += 1;
		let mut val = 2;
		let mut prime = true;
		while val * val <= n {
			if n % val == 0 {
				prime = false;
				break;
			}
			val += 1;
		}
		if prime {
			count += 1;
		}
	}
	n
}