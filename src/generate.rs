use rand::prelude::random;

pub fn vec(size: usize) -> Vec<u8> {
	let mut array: Vec<u8> = Vec::with_capacity(size);

	for _ in 0..array.capacity() {
		array.push(random::<u8>());
	}

	array
}
