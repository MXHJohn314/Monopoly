use crate::monopoly::board_space::BoardSpace;

pub enum Action {
	Message(String),
	AdvanceTo(BoardSpace),
	MoveDist(usize),
	Pay(usize), 
	PayEach(usize), 
	Collect(usize), 
	CollectEach(usize), 
	Maintenance((usize, usize)),
}

impl Action {
	
}
