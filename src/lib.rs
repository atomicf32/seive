use std::collections::{ LinkedList, HashSet };

pub fn calculate(limit: u128) -> Primes {
	let mut primes = Primes::new();
	let mut invalids = HashSet::new();

	for i in 2..=limit {
		if !invalids.contains(&i) {
			primes.add(i);

			for j in 1..=limit {
				if i * j > limit {
					break;
				}

				invalids.insert(i * j);
			}
		}
	}

	primes
}

pub struct Primes {
	array: LinkedList<u128>
}

impl Primes {
	fn new() -> Self {
		Self { array: LinkedList::new() }
	}

	fn add(&mut self, value: u128) {
		self.array.push_back(value);
	}
}

impl IntoIterator for Primes {
	type Item = u128;
	type IntoIter = <LinkedList<u128> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.array.into_iter()
	}
}