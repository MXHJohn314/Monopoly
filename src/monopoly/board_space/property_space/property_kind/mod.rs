use crate::monopoly::board_space::property_space::{title::PropertyTitle, railroad::RailRoad, utility::Utility, color_deed::color::Color};

#[derive(Debug, Clone)]
pub enum PropertyKind {
	UtilProp(PropertyTitle, Utility),
	RailProp(PropertyTitle, RailRoad),
	ColorProp(PropertyTitle, Color),
}
