/// pub(crate) mod color {
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq)]
pub struct PropertyTitle {
	pub(crate) owner: Option<usize>,
	pub(crate) purchase_price: usize,
	pub(crate) mortgage_price: usize,
	pub(crate) is_mortgaged: bool,
}

impl PropertyTitle {
	fn get_stats(&self) -> (Option<usize>, usize, usize, bool) {
		(
			self.owner,
			self.purchase_price,
			self.mortgage_price,
			self.is_mortgaged,
		)
	}
	
	pub fn new(mortgage_price: usize, purchase_price: usize) -> Self {
		Self {
			is_mortgaged: false,
			mortgage_price,
			owner: None,
			purchase_price,
		}
	}
	pub fn get_payout(&self) -> isize {
		*self.rents.get(0).unwrap()
	}
}

impl fmt::Display for PropertyTitle {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}
