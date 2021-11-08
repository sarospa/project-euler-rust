pub fn exec() -> usize {
	const TARGET: usize = 2000000;
	let primes = sieve_primes(TARGET);
	primes.iter().sum()
}

pub fn sieve_primes(range: usize) -> Vec<usize> {
	let mut sieve: Vec<bool> = vec![true; range];
	let mut primes: Vec<usize> = vec![];
	for n in 2..sieve.len() {
		if sieve[n] {
			primes.push(n);
		}
		let mut index = n * 2;
		while index < sieve.len() {
			sieve[index] = false;
			index += n;
		}
	}
	primes
}