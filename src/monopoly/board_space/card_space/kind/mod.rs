use super::{card::Card, deck::Deck};
use crate::monopoly::utils;

#[derive(Debug, Clone)]
pub enum CardKind {
	Chance,
	Chest,
}

impl CardKind {
	pub fn make_cards(kind: CardKind) -> Vec<Card> {
		match kind {
			CardKind::Chance => {vec![
				Card {
					message: format!("Make general repairs on all your property–For each house pay $25–For each hotel $100"),
					advance_to: Option::None,
					move_dist: Option::None,
					pay: Option<usize>, 
					pay_each: Option<usize>, 
					collect: Option<usize>, 
					collectEach:Option::None, 
					maintenance: Option<(usize, usize)>
				}
/*
,5,25 100,
Bank pays you dividend of $50,8,50
Pay poor tax of $15,8,15
Your building and loan matures—Collect $150,8,150
You have won a crossword competition—Collect $100,8,100
Advance to Illinois Ave—If you pass Go collect $200,10,24
Advance to St. Charles Place – If you pass Go collect $200,10,11
Take a trip to Reading Railroad–If you pass Go collect $200,10,5
Take a walk on the Boardwalk–Advance token to Boardwalk,10,39
Advance to Go (Collect $200),10,0 200
Get Out of Jail Free,1,
Go to Jail–Go directly to Jail–Do not pass Go. do not collect $200,3,10,
Advance token to nearest Utility. If unowned you may buy it from the Bank. If owned throw dice and pay owner a total ten times the amount thrown.,12,10,
You have been elected Chairman of the Board–Pay each player $50,7,50,
Go Back 3 Spaces,14,3,
Advance token to the nearest Railroad and pay owner twice the rental to which he/she {he} is otherwise entitled. If Railroad is unowned you may buy it from the Bank.,13,2,
*/				
			]}
			CardKind::Chest => {vec![
/*
Bank pays you dividend of $50,8,50
Doctor's fee—Pay $50,8,-50
From sale of stock you get $50,8,50
Pay hospital fees of $100,8,-100
Holiday Fund matures—Receive $100,8,100
It is your birthday—Collect $10,8,10
Life insurance matures–Collect $100,8,100
Pay school fees of $150,8,-150
Receive $25 consultancy fee,8,25
Income tax refund–Collect $20,8,20
You have won second prize in a beauty contest–Collect $10,8,10
You inherit $100,8,100
Advance to Go (Collect $200),10,0,200,
You are assessed for street repairs–$40 per house–$115 per hotel,5,40,115
Get Out of Jail Free,1,
Grand Opera Night—Collect $50 from every player for opening night seats,3,50
Go to Jail–Go directly to Jail–Do not pass Go. do not collect $200,3,10,
*/
			]}
		}
	}
}
