pub mod bubble;
pub mod insertion;
pub mod quick;
pub mod selection;

pub enum Algorithm {
	Quick,
	Bubble,
	Insertion,
	Selection,
}

impl Algorithm {
	pub fn as_str(&self) -> &str {
		match self {
			&Algorithm::Quick => "Quick",
			&Algorithm::Bubble => "Bubble",
			&Algorithm::Insertion => "Insertion",
			&Algorithm::Selection => "Selection",
		}
	}
}

pub struct SortResult {
	pub vector: std::vec::Vec<u8>,
	pub algorithm: Algorithm,
	pub duration: u128,
}

pub fn handle_algorithm(input_algorithm: &str, array: Vec<u8>) -> Vec<SortResult> {
	match input_algorithm {
		"1" => vec![quick::sort(array)],
		"2" => vec![bubble::sort(array)],
		"3" => vec![insertion::sort(array)],
		"4" => vec![selection::sort(array)],
		_ => vec![
			quick::sort(array.clone()),
			bubble::sort(array.clone()),
			insertion::sort(array.clone()),
			selection::sort(array.clone()),
		],
	}
}
