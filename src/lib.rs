pub fn calculate(limit: usize) -> SeiveResult {
	let mut result = vec![true; limit + 1];
	result[0] = false;
	result[1] = false;

	for i in 2.. {
		if i as f64 > (limit as f64).sqrt() {
			break;
		}
		
		if result[i] {
			let mut j = i.pow(2);
			while j <= limit {
				result[j] = false;
				j += i;
			}
		}
	}

	SeiveResult { result }
}

pub fn get_primes(result: &Vec<bool>) -> Vec<usize> {
	let mut primes = Vec::with_capacity(result.len());

	for (n, i) in result.iter().enumerate() {
		if *i {
			primes.push(n);
		}
	}

	primes
}

pub struct SeiveResult{
	result: Vec<bool>
}

impl SeiveResult {
	pub fn to_vec(&self) -> Vec<usize> {
		let mut primes = Vec::with_capacity(self.result.iter().filter(|x| **x).count());

		for (n, i) in self.result.iter().enumerate() {
			if *i {
				primes.push(n);
			}
		}

		primes
	}
}
