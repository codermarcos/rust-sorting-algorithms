use algorithms::{Algorithm, SortResult};
use generate::time;

pub fn sort(vector: Vec<u8>) -> SortResult {
	let start = time();

	let result = algorithm(vector);

	let duration = time() - start;

	SortResult {
		duration: duration.as_nanos(),
		algorithm: Algorithm::Quick,
		vector: result,
	}
}

fn algorithm(vector: Vec<u8>) -> Vec<u8> {
	let mut result = vec![];

	let p = vector[0];
	let mut left = vec![];
	let mut right = vec![];

	for i in 1..vector.len() {
		if vector[i] < p {
			left.push(vector[i]);
		} else {
			right.push(vector[i]);
		}
	}

	if left.len() != 0 {
		left = algorithm(left);
		result.extend(&left);
	}

	result.push(p);

	if right.len() != 0 {
		right = algorithm(right);
		result.extend(&right);
	}
	
	result
}


#[cfg(test)]
mod tests {
	use algorithms::{Algorithm, quick};

	#[test]
	fn quick_algorithm() {
		let array = vec![10, 5, 1, 3, 6];
		let result = quick::algorithm(array.clone());
		assert_eq!(result, vec![1, 3, 5, 6, 10]);
	}

	#[test]
	fn quick_result() {
		let array = vec![10, 5, 1, 3, 6];
		let result = quick::sort(array.clone());
		assert_eq!(result.vector, vec![1, 3, 5, 6, 10]);
		assert_eq!(result.algorithm.as_str(), Algorithm::Quick.as_str());
	}
}