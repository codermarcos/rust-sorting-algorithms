pub mod bubble;
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
