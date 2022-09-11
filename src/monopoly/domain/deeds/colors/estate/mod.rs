/// pub(crate) mod estate
#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub struct Estate {
	pub(crate)  num_houses: usize,
	pub(crate)  rents: Vec<usize>,
	pub(crate)  sibling_count: usize,
}

impl Estate {
	pub fn new(rents: Vec<usize>, count: usize) -> Self {
		Self {
			rents: rents,
			num_houses: 0,
			sibling_count: count
		}
	}
}
