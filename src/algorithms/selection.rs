
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