use super::super::BoardSpace;
use super::{kind::CardKind, deck::Deck};

#[derive(Debug, Clone)]
pub struct Card {
	pub(crate) message: String,
	pub(crate) advance_to: Option<BoardSpace>,
	pub(crate) move_dist: Option<isize>,
	pub(crate) pay: Option<usize>,
	pub(crate) pay_each: Option<usize>,
	pub(crate) collect: Option<usize>,
	pub(crate) collectEach: Option<usize>,
	pub(crate) maintenance: Option<(usize, usize)>,	
}
