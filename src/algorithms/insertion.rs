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