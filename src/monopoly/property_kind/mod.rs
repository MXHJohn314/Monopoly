/// pub mod property_kind
#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq)]
pub enum PropertyKind {
	UtilProp(PropertyTitle, Utility),
	RailProp(PropertyTitle, RailRoad),
	ColorProp(PropertyTitle, Color),
}

impl PropertyKind {
	pub fn get_kind(self) -> PropertyKind {
		if let _ = match self {
			UtilProp(_, _) => UtilProp,
			RailProp(_, _) => RailProp,
			ColorProp(_, _) => ColorProp,
			_ => {},
		}
	}
	
	// pub fn get_owner(self) -> 
}
