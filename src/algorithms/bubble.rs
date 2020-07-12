#[cfg(test)]
mod tests {

	#[test]
	fn bubble_algorithm() {
		let array = vec![10, 5, 1, 3, 6];
		let result = bubble::algorithm(array.clone());
		assert_eq!(result, vec![1, 3, 5, 6, 10]);
	}

}