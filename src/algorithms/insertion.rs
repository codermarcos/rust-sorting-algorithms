use algorithms::{Algorithm, SortResult};
use generate::time;

pub fn sort(vector: Vec<u8>) -> SortResult {
	let start = time();

	let result = algorithm(vector);

	let duration = time() - start;
	SortResult {
		algorithm: Algorithm::Insertion,
		duration: duration.as_nanos(),
		vector: result,
	}
}

fn algorithm(vector: Vec<u8>) -> Vec<u8> {
	let mut result: Vec<u8> = vector;

	for i in 0..result.len() {
		let mut x = i + 1;
		while x > 0 && x < result.len() && result[x - 1] > result[x] {
			let prev = result[x - 1];
			result[x - 1] = result[x];
			result[x] = prev;
			x -= 1;
		}
	}

	result
}


#[cfg(test)]
mod tests {
	use algorithms::{Algorithm, insertion};

	#[test]
	fn insertion_algorithm() {
		let array = vec![10, 5, 1, 3, 6];
		let result = insertion::algorithm(array.clone());
		assert_eq!(result, vec![1, 3, 5, 6, 10]);
	}

	#[test]
	fn insertion_result() {
		let array = vec![10, 5, 1, 3, 6];
		let result = insertion::sort(array.clone());
		assert_eq!(result.vector, vec![1, 3, 5, 6, 10]);
		assert_eq!(result.algorithm.as_str(), Algorithm::Insertion.as_str());
	}
}