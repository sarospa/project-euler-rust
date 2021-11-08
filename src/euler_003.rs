pub fn exec() -> i64{
	let mut value: i64 = 600851475143;
	let mut n: i64 = 2;
	while n < value {
		if value % n == 0 {
			value /= n;
		}
		else {
			n += 1;
		}
	}
	value
}