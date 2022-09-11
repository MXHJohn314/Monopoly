#![allow(dead_code)]


use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::io::{stdin, BufReader, BufRead};
use std::rc::Rc;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::Write;
use std::slice::SliceIndex;
use crate::BlueKind::{BoardWalk, ParkPlace};
use crate::BoardSpace::{Blue, Chance, Chest, FreeParking, Go, GoToJail, Green, IncomeTax, Jail, LightBlue, LuxuryTax, Orange, Pink, Purple, RailRoad, Red, Utility, Yellow};
use crate::monopoly::board_space;

mod monopoly;
pub struct Account {
	utilities: Vec<UtilityKind>,
	railroads: Vec<RailRoadKind>,
	purple: Vec<PurpleKind>,
	light_blue: Vec<LightBlueKind>,
	pink: Vec<PinkKind>,
	orange: Vec<OrangeKind>,
	red: Vec<RedKind>,
	yellow: Vec<YellowKind>,
	green: Vec<GreenKind>,
	blue: Vec<BlueKind>,
}



fn main() {
	let mut chest_cards: Vec<ChestKind> =  ChestKind::get_deck();
	let mut chance_cards: Vec<ChanceKind> = ChanceKind::get_deck();
	let mut board: Vec<BoardSpace> = vec![
		Go,
		Chest(chest_cards),
		Jail,
		FreeParking,
		GoToJail,
		Purple(Mediterranean),
		Purple(BalticAvenue),
		RailRoad(Reading),
		RailRoad(Pennsylvania),
		RailRoad(BAndO),
		RailRoad(ShortLine),
		LightBlue(Oriental),
		Chance(chance_cards),
		LightBlue(Vermont),
		LightBlue(Connecticut),
		Utility(ElectricCompany),
		Utility(WaterWorks),
		Pink(SaintCharles),
		Pink(StatesAvenue),
		Pink(VirginiaAvenue),
		Orange(SaintJamesPlace),
		Orange(TennesseeAvenue),
		Orange(NewYorkAvenue),
		Red(KentuckyAvenue),
		Red(IllinoisAvenue),
		Red(IndianaAvenue),
		Yellow(AtlanticAvenue),
		Yellow(VentnorAvenue),
		Yellow(MarvinGardens),
		Green(PacificAvenue),
		Green(NorthCarolinaAvenue),
		Green(PennsylvaniaAvenue),
		Blue(ParkPlace),
		Blue(BoardWalk),
		IncomeTax,
		LuxuryTax,
	];
	for i in board	{println!("{:?}", i)};
	
	
}

/// pub(crate) mod action {
// use std::fmt;
// use crate::BoardSpace;
use crate::ChanceKind::{AdvanceBoardwalk, AdvanceGo, AdvanceIllinois, AdvanceNearestUtil, AdvanceReadingRailroad, AdvanceStCharles, BankDividendCollect, ChanceGoToJail, CrosswordWin, GeneralRepairs, GeneralRepairsPayEach, JailFree, PoorTaxPay, RailroadSpecial};
use crate::ChestKind::{AdvanceGoCollect, BeautyContestCollect, BirthdayCollect, ConsultancyCollect, DividendCollect, DoctorsFeePay, HolidayFundCollect, HospitalFeePay, IncomeTaxRefund, InheritanceCollect, LifeInsuranceCollect, OperaCollectEach, SchoolFeesPay, StreetRepairsPayEach};
use crate::GreenKind::{NorthCarolinaAvenue, PacificAvenue, PennsylvaniaAvenue};
use crate::LightBlueKind::{Connecticut, Oriental, Vermont};
use crate::OrangeKind::{NewYorkAvenue, SaintJamesPlace, TennesseeAvenue};
use crate::PinkKind::{SaintCharles, StatesAvenue, VirginiaAvenue};
use crate::PurpleKind::{BalticAvenue, Mediterranean};
use crate::RailRoadKind::{BAndO, Pennsylvania, Reading, ShortLine};
use crate::RedKind::{IllinoisAvenue, IndianaAvenue, KentuckyAvenue};
use crate::UtilityKind::{ElectricCompany, WaterWorks};
use crate::YellowKind::{AtlanticAvenue, MarvinGardens, VentnorAvenue};

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub struct Player {
	pub(crate) name: String,
	pub(crate) location: isize,
	pub(crate) money: isize,
	pub(crate) deeds: Vec<BoardSpace>,
	pub(crate) in_jail: isize,
	pub(crate) index: usize,
}

impl Player {
	pub fn setMoney(&mut self, amount: isize) {
		self.money += amount;
	}
}

impl fmt::Display for Player {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameState {
	board: Vec<BoardSpace>,
	deed_map: HashMap<usize, Option<usize>>,
	accounts: HashMap<usize, isize>,
	player_map: HashMap<usize, String>,
	current_player: usize,
	location_map: HashMap<usize, usize>,
	chance_cards: Vec<ChanceKind>,
	chest_cards: Vec<ChestKind>,
}


/*impl GameState {
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
		let mut chance_cards = Deck::new(Chance);
		let mut chest_cards = Deck::new(Chest);
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
		Self { board, deed_map, accounts, player_map, current_player: 0, location_map, chance_cards ,chest_cards}
	}
	
	fn roll(&self) -> [usize; 2] {
		let mut rng = thread_rng();
		let roll_1 = rng.gen_range(1, 6);
		let roll_2 = rng.gen_range(1, 6);
		[roll_1, roll_2]
	}
	
	fn play(&mut self) {
		while self.player_map.len() > 1 {
			if !self.take_turn() {
				self.current_player += 1;
			}
			self.current_player = self.current_player % self.player_map.len();
		}
	}
	
	fn auction(&mut self, i1: &isize, x: &String) {}
	
	fn movePlayer(mut self) -> ([usize; 2], String, usize, usize) {
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
	
	fn take_turn(mut self) -> bool {
		for i in 0..TOO_MANY_DOUBLES {
			let (roll, name, old_player_location, current_player_location) = self.movePlayer();
			let mut current_player_balance = *self.accounts.get(&self
			 .current_player).unwrap().borrow_mut();
			match &self.board[current_player_location] {
				CornerSpace(conrner_kind) => self.take_corner_turn(CornerSpace),
				PropertySpace(property_kind) => self.take_property_turn(property_kind.borrow(), self.current_player),
				CardSpace(card_kind) => { match card_kind {
						Chance => self.take_card_turn(),
						Chest => self.take_card_turn(),
				}},
				LuxuryTax => self.take_tax_turn(LuxuryTax, current_player_balance as usize), // $75
				IncomeTax => self.take_tax_turn(IncomeTax, current_player_balance as usize), // 15% or $200
			}
			if Self.jail_check(roll){
				break
			}
		}
		false
	}
		pub fn take_player_action(&self, player: usize, mut wallet: usize, players: Vec<usize>){
		let func = match self {
			Pay(amount) => { wallet -= amount },
			Message(string) => || echo!(string),
			PayEach(amount) => {
				for other in players.iter().map(|x| x).filter(|p: usize|p != player) {
					self.pay(player, other, amount);
				}
			},
			AdvanceTo(board_space) => self.advance_player_to(board_space),
			MoveDist(distance) => self.move_player_n_times(),
			Collect(amount) => { wallet += amount },
			CollectEach(amount) =>  {
				for other in players.iter().map(|x| x).filter(|p: usize|p != player) {
					self.pay(other, player, amount);
				}
			}
			Maintenance(Some((house_cost, hotel_cost))) => {},
			_ => {}
		};
	}

	fn jail_check(roll: [usize; 2]) -> ! {
		if roll[0] == roll[1] {
			println!("You rolled doubles, roll again!");
			true
		} 
		false
	}
	fn take_card_turn(self, card_kind: CardKind) -> () {
		let mut deck = match card_kind {
			Chance => self.chance_cards,
			Chest => self.chest_cards,
		};
		deck
	}
	fn take_corner_turn(self, corner_kind: fn(CornerKind) -> BoardSpace) -> () {
		match corner_kind {
			GoToJail => echo!("Go to Jail–Go directly to Jail–Do not pass Go. do not collect $200"),
			Jail => echo!("Jail!\nJust visiting..."),
			FreeParking | Go => {}
		}
	}
	fn take_tax_turn(self, tax_kind: TaxKind, current_player_balance: usize) -> () {
		match space {
			IncomeTax => match bool_in("You landed on Income Tax!\n Do you want to pay 15% or 200?") {
				true => current_player_balance * 0.15,
				_ => {}
			},
			LuxuryTax => {
				echo!("You landed on Luxury Tax! Pay $100.");
				100
			}
		}
	}
	fn take_property_turn(mut self, property_kind: &PropertyKind, player: usize) -> () {
		match property_kind.get_owner() {
			
		}
		match property_kind {
			UtilProp(property_title, utility) => match utility { 
				_ => {
					let roll = self.roll().iter().sum();
					let property_count = self.board.iter().filter(|b| b == utility).count();
				},
			},
			RailProp(title, railroad) => {
				
			}
			_ => {}
		}
	}
}*/

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum BoardSpace {
	Go,
	Jail,
	FreeParking,
	GoToJail,
	Chance(Vec<ChanceKind>),
	Chest(Vec<ChestKind>),
	Purple(PurpleKind),
	RailRoad(RailRoadKind),
	LightBlue(LightBlueKind),
	Utility(UtilityKind),
	Pink(PinkKind),
	Orange(OrangeKind),
	Red(RedKind),
	Yellow(YellowKind),
	Green(GreenKind),
	Blue(BlueKind),
	IncomeTax,
	LuxuryTax,
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq)] 
pub enum ChanceKind {
	BankDividendCollect,
	GeneralRepairs,
	PoorTaxPay,
	CrosswordWin,
	AdvanceIllinois,
	AdvanceStCharles,
	AdvanceReadingRailroad,
	AdvanceBoardwalk,
	AdvanceGo,
	JailFree,
	ChanceGoToJail,
	AdvanceNearestUtil,
	GeneralRepairsPayEach,
	RailroadSpecial,
}
impl ChanceKind {
	pub fn get_deck() -> Vec<ChanceKind> {
		vec![
			BankDividendCollect,
			GeneralRepairs,
			PoorTaxPay,
			CrosswordWin,
			AdvanceIllinois,
			AdvanceStCharles,
			AdvanceReadingRailroad,
			AdvanceBoardwalk,
			AdvanceGo,
			JailFree,
			ChanceGoToJail,
			AdvanceNearestUtil,
			GeneralRepairsPayEach,
			RailroadSpecial,
		]
	}
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq)] 
pub enum ChestKind {
	DividendCollect,
	DoctorsFeePay,
	HospitalFeePay,
	HolidayFundCollect,
	BirthdayCollect,
	LifeInsuranceCollect,
	SchoolFeesPay,
	ConsultancyCollect,
	IncomeTaxRefund,
	BeautyContestCollect,
	InheritanceCollect,
	AdvanceGoCollect,
	StreetRepairsPayEach,
	JailFree,
	ChestGoToJail,
	OperaCollectEach,
}
impl ChestKind {
	pub fn get_deck() -> Vec<ChestKind>
	{   
		vec![
			DividendCollect,
			DoctorsFeePay,
			HospitalFeePay,
			HolidayFundCollect,
			BirthdayCollect,
			LifeInsuranceCollect,
			SchoolFeesPay,
			ConsultancyCollect,
			IncomeTaxRefund,
			BeautyContestCollect,
			InheritanceCollect,
			AdvanceGoCollect,
			StreetRepairsPayEach,
			ChestKind::JailFree,
			OperaCollectEach,
		]
	}
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum PurpleKind {
	Mediterranean,
	BalticAvenue,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum RailRoadKind {
	Reading,
	Pennsylvania,
	BAndO,
	ShortLine,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum LightBlueKind {
	Oriental,
	Vermont,
	Connecticut,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum UtilityKind {
	ElectricCompany,
	WaterWorks,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum PinkKind {
	SaintCharles,
	StatesAvenue,
	VirginiaAvenue,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum OrangeKind {
	SaintJamesPlace,
	TennesseeAvenue,
	NewYorkAvenue,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum RedKind {
	KentuckyAvenue,
	IllinoisAvenue,
	IndianaAvenue
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum YellowKind {
	AtlanticAvenue,
	VentnorAvenue,
	MarvinGardens,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum GreenKind {
	PacificAvenue,
	NorthCarolinaAvenue,
	PennsylvaniaAvenue,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum BlueKind {
	ParkPlace,
	BoardWalk,
}

