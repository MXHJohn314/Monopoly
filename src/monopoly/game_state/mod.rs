use std::{fs::File, io::{BufRead, BufReader}, borrow::BorrowMut, collections::HashMap};
use std::async_iter::from_iter;
use std::fmt::format;
use rand::{Rng, thread_rng};
use crate::monopoly::board_space::card_space::deck::Deck;
use crate::monopoly::board_space::corner_space::CornerKind;
use crate::monopoly::board_space::property_space::color_deed::place::Place::{*};
use crate::monopoly::board_space::tax_space::TaxSpace;
use super::{BOARD_PATH, TOO_MANY_DOUBLES};
use super::board_space::BoardSpace;
use super::board_space::BoardSpace::{CornerSpace, CardSpace, PropertySpace};
use super::board_space::card_space::kind::CardKind;
use super::board_space::card_space::kind::CardKind::{Chance, Chest};
use super::board_space::corner_space::CornerKind::{Go, GoToJail, FreeParking, Jail};
use super::board_space::property_space::utility::Utility;
use super::board_space::property_space::utility::Utility::{WaterWorks, ElectricCompany};
use super::board_space::property_space::title::PropertyTitle;
use super::board_space::property_space::color_deed::color;
use super::board_space::property_space::color_deed::color::Color; 
use super::board_space::property_space::color_deed::estate::Estate;
use super::board_space::property_space::color_deed::place::Place;
use super::board_space::property_space::property_kind::PropertyKind; 
use super::board_space::property_space::property_kind::PropertyKind::{*};
use super::board_space::tax_space::TaxSpace::{*};
use super::utils::helpers::{my_parsing::{*}, my_macros::{*}, my_io::{*}};


pub struct GameState {
	board: Vec<BoardSpace>,
	deed_map: HashMap<usize, Option<usize>>,
	accounts: HashMap<usize, isize>,
	player_map: HashMap<usize, String>,
	current_player: usize,
	location_map: HashMap<usize, usize>,
}


impl GameState {
	pub fn new() -> Self {
		println!("how many players?");
		let num_players = int_in() as usize;
		let mut location_map: HashMap<usize, usize> = HashMap::with_capacity(num_players);
		let mut player_map: HashMap<usize, String> = HashMap::with_capacity(num_players);
		let mut accounts: HashMap<usize, isize> = HashMap::with_capacity(num_players as usize);
		for i in 0..num_players as usize {
			accounts.insert(i, 1500);
			player_map.insert(i, String::from(
				str_in(join!("What's player ", i + 1, "'s name?"))));
			location_map.insert(i, 0);
		}
		let mut deed_map: HashMap<usize, Option<usize>> = HashMap::new();
		let mut chance = Deck::new(Chance);
		let mut chest = Deck::new(Chest);
		let mut board: Vec<BoardSpace> = vec![
			BoardSpace::new(CornerSpace(CornerKind::Go)),
			BoardSpace::new(
				PropertySpace(ColorProp(PropertyTitle {
					owner: None,
					purchase_price: 60,
					mortgage_price: 30,
					is_mortgaged: false,
				}, Color::Purple(MediterraneanAve, Estate::new(
					vec![10, 30, 90, 160, 250], 0,					
				))))),
			BoardSpace::new(IncomeTax),
			BoardSpace::new(ElectricCompany),
			BoardSpace::new(RailRoad),
			BoardSpace::new(Chance),
			BoardSpace::new(Chest),
			BoardSpace::new(Jail),
			BoardSpace::new(FreeParking),
			
		];
	}
	
	pub fn roll(&self) -> [usize; 2] {
		let mut rng = thread_rng();
		let roll_1 = rng.gen_range(1, 6);
		let roll_2 = rng.gen_range(1, 6);
		[roll_1, roll_2]
	}
	
	pub fn play(&mut self) {
		while self.player_map.len() > 1 {
			if !self.take_turn() {
				self.current_player += 1;
			}
			self.current_player = self.current_player % self.player_map.len();
		}
	}
	
	pub fn auction(&mut self, i1: &isize, x: &String) {}
	
	pub fn movePlayer(mut self) -> ([usize; 2], String, usize, usize) {
		let player_name = self.player_map.get(&self.current_player).unwrap();
		echo!("It's", player_name, "'s turn");
		let (roll, name, old_player_location, current_player_location) = self.movePlayer();
		let old_player_location = self.location_map.get(&self.current_player).unwrap().clone();
		let roll: [usize; 2] = self.roll();
		let name = self.player_map.get(&self.current_player).unwrap();
		let current_player_location = (old_player_location + roll[0] + roll[1]) % self.board.len();
		self.location_map.insert(self.current_player.clone(), current_player_location.clone());
		let mut space_name = self.board[current_player_location].name.clone();
		echo!(name.clone()," rolled ", roll[1]," + ", roll[0], " and is now at ", space_name,"\n");
		(roll, name.to_owned(), old_player_location, current_player_location)
	}
	
	pub fn take_turn(mut self) -> bool {
		for i in 0..TOO_MANY_DOUBLES {
			let (roll, name, old_player_location, current_player_location) = self.movePlayer();
			let mut current_player_balance = *self.accounts.get(&self
			 .current_player).unwrap().borrow_mut();
			match &self.board[current_player_location] {
				CornerSpace(conrner_kind) => self.take_corner_urn(),
				PropertySpace(property_kind) => self.take_property_turn(),
				CardSpace(card_kind) => { match card_kind {
						Chance => self.take_card_turn(),
						Chest => self.take_card_turn(),
				}},
				LuxuryTax => self.take_tax_turn(LuxuryTax), // $75
				IncomeTax => self.take_tax_turn(LuxuryTax), // 15% or $200
			}
			if Self.jail_check(roll){
				break
			}
		}
		false
	}
	
	fn jail_check(roll: [usize; 2]) -> ! {
		if roll[0] == roll[1] {
			println!("You rolled doubles, roll again!");
			true
		} 
		false
	}
	pub fn take_card_turn() -> () {}
	pub fn take_corner_turn() -> () {}
	pub fn take_property_turn() -> () {}
	pub fn take_tax_turn() -> () {}
}
