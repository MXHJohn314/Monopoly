use std::{fs::File, io::{BufRead, BufReader}, borrow::BorrowMut, collections::HashMap};
use std::fmt::format;
use rand::{Rng, thread_rng};
use crate::monopoly::board_space::property_space::color_deed::place::Place::{MediterraneanAve, RailRoad};
use crate::monopoly::board_space::property_space::property_kind::PropertyKind::ColorProp;
use crate::monopoly::board_space::property_space::utility::Utility;
use super::{
	BOARD_PATH, 
	TOO_MANY_DOUBLES,
	board_space::{BoardSpace, BoardSpace::{
		CornerSpace, PropertySpace,
	},
	card_space::{deck::Deck, kind::{CardKind, CardKind::{Chance, Chest}}},
	corner_space::{CornerKind, CornerKind::{Go, GoToJail, FreeParking, Jail}},
	property_space::{
		utility::Utility::{WaterWorks, ElectricCompany},
		railroad,
		title::PropertyTitle,
		color_deed::{color::Color, estate::Estate, place::Place},
		property_kind::PropertyKind},
		tax_space::TaxKind::{*}},
	utils::helpers::{my_parsing::{*}, my_macros::{*}, my_io::{*}},
};

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
	
	pub fn take_turn(mut self) -> bool {
		for i in 0..TOO_MANY_DOUBLES {
			let player_name = self.player_map.get(&self.current_player).unwrap();
			println!("It's {}'s turn", player_name);
			let roll = self.roll();
			let name = self.player_map.get(&self.current_player).unwrap();
			let old_player_location = self.location_map.get(&self.current_player).unwrap().clone();
			let current_player_location = (old_player_location + roll[0] + roll[1]) % self.board.len();
			self.location_map.insert(self.current_player.clone(), current_player_location.clone());
			let mut space_name = self.board[current_player_location].name.clone();
			echo!(name.clone()," rolled ", roll[1]," + ", roll[0], " and is now at ", space_name,"\n");
			let mut current_player_balance = *self.accounts.get(&self
			 .current_player).unwrap().borrow_mut();
			match self.board[current_player_location] {
				CardKind(card_kind) => {}
				ColorSpace(mut color_deedc) => {
					let current_player_name = self.player_map[&self.current_player].clone();
					match &mut color_deed.deed.owner {
						Some(owner) => {
							let rent = 5; //self.calculate_rent(self.board[current_player_location]);
							if current_player_balance - rent < 0 {
								printc!(current_player_name, " can't pay up,\
                                 and has been kicked out of the game!");
								self.player_map.remove(&self.current_player);
								return true;
							}
							self.accounts.insert(self.current_player, current_player_balance - rent);
							self.accounts.insert(owner.clone(), self.accounts.get(owner).unwrap() + rent);
						}
						None => {
							let mut deed = color_deed.deed;
							let purchase_price = &deed.purchase_price;
							if purchase_price <= current_player_balance
							 && bool_in(
								join!("Does ",current_player_name,
                                        " want to buy ", space_name, 
                                        "?\nPrice: ", purchase_price,
                                        "\nbalance: ", current_player_balance)) {
								deed.owner = Option::from(self.current_player);
								self.accounts.insert(
									self.current_player,
									current_player_balance - purchase_price);
								printc!(current_player_name, " has purchased ", space_name,
                                "\nRemaining balance: $", self.accounts.get(&self.current_player).unwrap());
							} else {
								self.auction(purchase_price, player_name);
								// (&mut self, bidders: &mut Vec<(&str, isize)>, deed: &Deed)
							}
						}
					}
				}
				CornerSpace(corner) => {}
				TaxSpace(tax) => {}
				UtilSpace(utility) => {}
				RailSpace(railroad) => {}
				_ => {}
			}
			if roll[0] == roll[1] {
				println!("You rolled doubles, roll again!");
				continue;
			} else {
				break;
			}
		}
		return false;
	}
		/*match &space.subtype {
		
	}
		_kind::
		ColorSpace(color_deed { space: _, deed, color: _color, houses: _, }) => {
			match &deed.owner {
				Some(_non_banker) => {
					printc!("you owe ", _non_banker, " money");
				}
				None => {
					let bidders = game_board.get_bidders();
					match auction(bidders, deed) {
						Some((mut winner, amount)) => {
							ownership_map.insert(deed.name.clone(), winner.clone());
							player_map.get(deed.name.as_str()).unwrap().setMoney(-1 * amount);
						}
						_ => println!("Nobody bought {}", deed),
					};
				}
			};
		}
		_ => {} //other kinds of spaces
	}
	   fn auction<'a>(deed: Deed ) -> Option<(String, isize)> {
			// &mut self, bidders: &mut Vec<(&str, isize)>
			let mut player = bidders[0].0;
			let mut player_index = bidders[0].1;
			let mut i = 0;
			let mut highest_bid = deed.purchase_price;
			let mut num_players = bidders.len();
			while bidders.len() > 1 {
				i %= bidders.len();
				player = bidders[0].0;
				let mut quit = false;
				if player.money >= highest_bid
					&&
					bool_in(concat!(
					"Does ", self.player_map.get(&i), " want to continue to bid?",
					"\nhighest bid: ", highest_bid,
					"\nBalance: ", player.money).as_ref()) {
					let mut bid;
					loop {
						println!("How much do you want to pay?");
						bid = int_in();
						if bid > highest_bid {
							println!("{} is higher than {}, all good", highest_bid, bid);
							highest_bid = bid;
							break;
						} else if bool_in(concat!(
						"$", bid, " is not higher than $", highest_bid, ".",
						" Quit bidding?").as_ref()) {
							break;
						}
					}
					if quit {
						bidders.remove(i);
						i -= 1;
					}
				} else {
					bidders.remove(i);
					i -= 1;
				}
			}
			if i < num_players {
				match bool_in(concat!("All other bidders have been eliminated. ", player, ",\n\
			do you want to buy ", deed, " for $", highest_bid, "?").as_ref()) {
					true => Some((player.name.clone(), highest_bid)),
					false => None,
				}
			} else {
				printc!("All other bidders have been eliminated.\n", player.name,
			 ", you have purchased ", deed, " for $", highest_bid,"!");
				Some((player.name.clone(), highest_bid))
			}
		}*/
}
