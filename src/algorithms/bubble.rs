use algorithms::{Algorithm, SortResult};
use generate::time;

pub fn sort(vector: Vec<u8>) -> SortResult {
	let start = time();

	let result = algorithm(vector);

	let duration = time() - start;
	SortResult {
		duration: duration.as_nanos(),
		algorithm: Algorithm::Bubble,
		vector: result,
	}
}

fn algorithm(vector: Vec<u8>) -> Vec<u8> {
	let mut ordened: bool = false;
	let mut result: Vec<u8> = vector;

	while !ordened {
		ordened = true;

		for n in 0..result.len() {
			if n + 1 < result.len() && result[n] > result[n + 1] {
				let n1: u8 = result[n];
				let n2: u8 = result[n + 1];

				result[n] = n2;
				result[n + 1] = n1;
				ordened = false;
			}
		}
	}

	result
}

#[cfg(test)]
mod tests {

	#[test]
	fn bubble_algorithm() {
		let array = vec![10, 5, 1, 3, 6];
		let result = bubble::algorithm(array.clone());
		assert_eq!(result, vec![1, 3, 5, 6, 10]);
	}

	#[test]
	fn bubble_result() {
		let array = vec![10, 5, 1, 3, 6];
		let result = bubble::sort(array.clone());
		assert_eq!(result.vector, vec![1, 3, 5, 6, 10]);
		assert_eq!(result.algorithm.as_str(), Algorithm::Bubble.as_str());
	}
}