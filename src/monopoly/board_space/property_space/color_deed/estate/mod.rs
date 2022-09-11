pub struct Estate {
	pub num_houses: usize,
	pub rents: Vec<usize>,
	pub sibling_count: usize,
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
