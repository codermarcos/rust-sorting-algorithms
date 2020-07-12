
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