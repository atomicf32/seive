use bv::BitVec;

pub fn calculate(limit: u64) -> SieveResult {
	let mut sieve: BitVec<u8> = BitVec::new_fill(true, limit + 1);

	sieve.set(0, false);
	sieve.set(1, false);

	for i in 2.. {
		if i as f64 > (limit as f64).sqrt() {
			break;
		}
		
		if sieve[i] {
			for j in ((i * i)..=limit).step_by(i as usize) {
				sieve.set(j, false);
			}
		}
	}

	SieveResult(sieve)
}

pub struct SieveResult(BitVec<u8>);

impl SieveResult {
	pub fn count_primes(&self) -> u64 {
		let mut count = 0;

		for i in 0..self.0.len() {
			if self.0[i] {
				count += 1;
			}
		}

		count
	}

	pub fn iter(&self) -> Iter {
		Iter::new(&self.0)
	}
}

impl IntoIterator for SieveResult {
    type Item = u64;

    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.0)
    }
}

pub struct Iter<'a> {
	bits: &'a BitVec<u8>,
	count: u64,
}

impl<'a> Iter<'a> {
	pub fn new(bits: &'a BitVec<u8>) -> Self {
		Self { bits, count: 0 }
	}
}

impl Iterator for Iter<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.bits.len() {
			return None;
		}

		while self.bits[self.count] == false {
			self.count += 1;
		}

		self.count += 1;

		Some(self.count - 1)
    }
}

pub struct IntoIter {
	bits: BitVec<u8>,
	count: u64,
}

impl IntoIter {
	pub fn new(bits: BitVec<u8>) -> Self {
		Self { bits, count: 0 }
	}
}

impl Iterator for IntoIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.bits.len() {
			return None;
		}

		while self.bits[self.count] == false {
			self.count += 1;
		}

		self.count += 1;

		Some(self.count - 1)
    }
}
