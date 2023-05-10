use std::{collections::LinkedList};

const BITMASK: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

pub fn calculate(limit: usize) -> LinkedList<usize> {
	if limit < 2 {
		return LinkedList::new();
	}

	let sqrt = (limit as f64).sqrt() as usize;
	let mut initial = LinkedList::new();

	{
		let mut sieve = Sieve::new(sqrt + 5 / 8);

		for i in 2..=(sqrt as f64).sqrt() as usize {
			if sieve.get(i - 2) {
				let mut cur = i * i;

				while cur <= sqrt {
					sieve.set(cur - 2, false);

					cur += i;
				}
			}
		}
		
		for i in 2..=sqrt {
			if sieve.get(i - 2) {
				initial.push_back(i);
			}
		}
	}

	let segment_size = cache_size::l1_cache_size().unwrap();

	let mut primes = initial.clone();
	let mut sieve = Sieve::new(segment_size);

	let mut low = sqrt + 1;
	let mut high = std::cmp::min(low + segment_size, limit);

	loop {
		for i in initial.iter() {
			let mut cur = 0;

			while cur * i <= high {
				if cur * i < low {
					cur += 1;
					continue;
				}

				sieve.set(cur * i - low, false);

				cur += 1;
			}
		}

		for i in low..=high {
			if sieve.get(i - low) {
				primes.push_back(i);
			}
		}

		if high == limit {
			break;
		}

		low = high + 1;
		high = std::cmp::min(low + segment_size, limit);
		sieve.reuse();
	}

	primes
}

struct Sieve {
	data: Vec<u8>
}

impl Sieve {
	fn new(bytes: usize) -> Self {
		Self { data: vec![0xff; bytes] }
	}

	fn get(&self, index: usize) -> bool {
		self.data[index / 8] & BITMASK[index % 8] != 0
    }

	fn set(&mut self, index: usize, value: bool) {
		let ref mut byte = self.data[index / 8];

		*byte ^= (-(value as i8) as u8 ^ *byte) & (1 << (index % 8))
	}

	fn reuse(&mut self) {
		self.data.fill(0xff);
	}
}

#[cfg(test)]
mod test {
	use crate::*;

	#[test]
	fn test_set() {
		let bytes = 16;

		let mut sieve = Sieve::new(bytes);

		for i in (0..(bytes * 8)).step_by(2) {
			sieve.set(i, false);
		}

		for i in (0..(bytes * 8)).step_by(2) {
			assert!(!sieve.get(i));
		}
	}
}
