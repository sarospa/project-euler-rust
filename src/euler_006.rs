pub fn exec() -> i32 {
	const TARGET: i32 = 100;
	let mut sum_of_squares = 0;
	let mut square_of_sums = 0;
	for n in 1..TARGET+1 {
		sum_of_squares += n * n;
		square_of_sums += n;
	}
	square_of_sums *= square_of_sums;
	square_of_sums - sum_of_squares
}