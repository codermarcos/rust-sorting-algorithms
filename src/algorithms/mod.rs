pub mod bubble;
pub mod insertion;
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
