pub fn exec() -> i32 {
	for a in 1..500 {
		for b in a..((1000 - a) / 2) {
			let c = 1000 - (a + b);
			if a * a + b * b == c * c {
				return a * b * c
			}
		}
	}
	0
}