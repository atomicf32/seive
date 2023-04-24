pub fn calculate(limit: usize) -> Vec<usize> {
	let mut primes = Vec::with_capacity(limit / 2);
	let mut invalids = vec![true; limit + 1];

	for i in 2..=limit {
		if invalids[i] {
			primes.push(i);

			for j in 1..=limit {
				if i * j > limit {
					break;
				}

				invalids[i * j] = false;
			}
		}
	}

	primes
}