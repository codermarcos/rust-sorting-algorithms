use algorithms::{Algorithm, SortResult};
use generate::time;

pub fn sort(vector: Vec<u8>) -> SortResult {
	let start = time();

	let result = algorithm(vector);

	let duration = time() - start;
	SortResult {
		algorithm: Algorithm::Selection,
		duration: duration.as_nanos(),
		vector: result,
	}
}

fn algorithm(vector: Vec<u8>) -> Vec<u8> {
	let mut result = vector;

	for i in 0..result.len() {
		let mut n: usize = i;

		for x in i..result.len() {
			if result[x] < result[n] {
				n = x;
			}
		}
		if i != n {
			let min = result[n];
			result[n] = result[i];
			result[i] = min;
		}
	}

	result
}


#[cfg(test)]
mod tests {
	use algorithms::{Algorithm, selection};

	#[test]
	fn selection_algorithm() {
		let array = vec![10, 5, 1, 3, 6];
		let result = selection::algorithm(array.clone());
		assert_eq!(result, vec![1, 3, 5, 6, 10]);
	}

	#[test]
	fn selection_result() {
		let array = vec![10, 5, 1, 3, 6];
		let result = selection::sort(array.clone());
		assert_eq!(result.vector, vec![1, 3, 5, 6, 10]);
		assert_eq!(result.algorithm.as_str(), Algorithm::Selection.as_str());
	}
}