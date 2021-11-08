pub fn exec() -> i32 {
	let target = 4000000;
	let mut fib1 = 1;
	let mut fib2 = 2;
	let mut sum = 0;
	while fib1 < target {
		if fib1 % 2 == 0 {
			sum += fib1;
		}
		fib2 = fib1 + fib2;
		fib1 = fib2 - fib1;
	}
	sum
}