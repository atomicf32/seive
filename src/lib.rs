pub fn calculate(limit: usize) -> Sieve {
	let mut sieve = vec![true; cache_size::l1_cache_size().unwrap()];



	Sieve(sieve)
}

pub struct Sieve(Vec<bool>);

impl Sieve {
	pub fn count_primes(&self) -> usize {
		self.0.iter().filter(|i| **i).count()
	}

	pub fn iter<'a>(&'a self) -> Box<impl Iterator<Item = usize> + 'a> {
		Box::new(self.0.iter().enumerate().filter_map(|(n, i)| if *i { Some (n) } else { None }))
	}

	pub fn inner(&mut self) -> &mut Vec<bool> {
		&mut self.0
	}
}
