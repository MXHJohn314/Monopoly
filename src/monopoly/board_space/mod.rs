pub mod card_space;
pub mod corner_space;
pub mod property_space;
pub mod tax_space;

use std::fmt;
use crate::monopoly::board_space::{
	card_space::kind::CardKind, 
	corner_space::CornerKind,
	property_space::property_kind::PropertyKind,
};

pub enum BoardSpace {
	PropertySpace(PropertyKind),
	CornerSpace(CornerKind),
	SpaceCard(CardKind),
}

impl BoardSpace {
	pub(crate) fn get_kind(&mut self) -> &mut BoardSpace {
		self.subtype.borrow_mut()
	}
	fn to_string(&self) -> String {
		format!("{}", self.name)
	}
}
impl fmt::Display for BoardSpace {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			write!(f, "{}", self.name)
		}	
}

