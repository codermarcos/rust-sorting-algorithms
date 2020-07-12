use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use rand::prelude::random;

pub fn time() -> std::time::Duration {
	SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("Tempo esta fora do range")
}

pub fn vec(size: usize) -> Vec<u8> {
	let mut array: Vec<u8> = Vec::with_capacity(size);

	for _ in 0..array.capacity() {
		array.push(random::<u8>());
	}

	array
}

#[cfg(test)]
mod tests {
	use generate;
	
	#[test]
	fn generate_vec_test() {
		let array = generate::vec(10);
		assert_eq!(array.len(), 10);
	}
}