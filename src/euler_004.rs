pub fn exec() -> i32 {
	let mut best = 0;
	for x in 100..1000 { 
		for y in 100..1000 {
			let product = x * y;
			let digits = product.to_string();
			let mut result = true;
			for i in 0..(digits.len() / 2) {
				if digits.chars().nth(i) != digits.chars().nth(digits.len() - (i + 1)) { result = false; }
			}
			if result && product > best {
				best = product;
			}
		}
	}
	best
}