// use std::fs::File;
// use std::io::{stdin, Write};
// use crate::monopoly::TOO_MANY_DOUBLES;
// 
// mod domain;
// mod r#macro;
// 
// pub mod helpers {
// 	pub mod my_macros {
// 		macro_rules! echo {
// 			($($a_string:expr), *) => {{
// 				let mut message = String::new();
// 				$( message = format!("{}{}", message, $a_string); )*
// 				match message.chars().last().unwrap() {
// 					'\n' => print!("{}",message),
// 					_ => println!("{}",message),
// 				};
// 			}};
// 		}
// 		macro_rules! join {
// 			($($a_string:expr), *) => {{
// 				let mut message = String::new();
// 				$( message = format!("{}{}", message, $a_string); )*
// 				message.to_owned().as_str()
// 			}};
// 		}
// 		pub(crate) use {echo, join};
// 	}
// 	
// 	pub mod my_io {
// 		use std::io::{stdin, Write};
// 		use crate::monopoly::utils::helpers::my_macros::*;
// 		
// 		pub fn str_in(message: &str) -> String {
// 			let mut s = String::new();
// 			loop {
// 				println!("{}", message);
// 				stdin().read_line(&mut s);
// 				match s.trim().len() {
// 					0 => {
// 						println!("Empty Strings not allowed. Try again.");
// 						continue;
// 					}
// 					_s => return String::from(s.trim())
// 				};
// 			}
// 		}
// 		
// 		pub fn int_in() -> isize {
// 			loop {
// 				let mut s = String::new();
// 				stdin().read_line(&mut s);
// 				match s.trim().parse::<isize>() {
// 					Ok(num) => { return num; }
// 					Err(e) => {
// 						print!("not a valid input. Try again. {:?}", e);
// 					}
// 				};
// 			}
// 		}
// 		
// 		pub fn bool_in(message: &str) -> bool {
// 			let mut i: isize;
// 			loop {
// 				echo!(message,"\n1 for yes, 0 for no" );
// 				i = int_in();
// 				match i {
// 					0 | 1 => return i == 1,
// 					_ => {
// 						echo!("'", i, "' is not a valid choice.");
// 						// continue;
// 					}
// 				}
// 			}
// 		}
// 		
// 		pub fn pause() {
// 			stdin().read_line(&mut String::new());
// 			std::io::stdout().flush();
// 		}
// 	}
// 	
// 	pub mod my_parsing {
// 		pub fn f_parse(s: &str) -> f32 {
// 			s.split_whitespace()
// 			 .map(|s| s.parse::<f32>())
// 			 .collect::<Result<Vec<_>, _>>().unwrap()[0]
// 		}
// 		
// 		pub fn parse_i(s: &str) -> isize {
// 			parse_i_array(s)[0]
// 		}
// 		
// 		pub fn parse_i_array(s: &str) -> Vec<isize> {
// 			parse_i_array_wrap(s).unwrap()
// 		}
// 		
// 		pub fn parse_i_array_wrap(s: &str) -> Option<Vec<isize>> {
// 			s.split(" ")
// 			 .map(|s| { /*println!("'{}'", s);*/ s.parse::<isize>().ok() })
// 			 .collect()
// 		}
// 	}
// }
// 
// pub mod game_utils {
// 	use std::fs::File;
// 	use crate::monopoly::TOO_MANY_DOUBLES;
// 	
// 	pub fn make_files() {
// 		let all_lines = [
// 			"Corner,GO,-1,-1,-1,-1,Corner,false
// 	Purple,Mediterranean Avenue,60,50,2 10 30 90 160 250,30,Color,true
// 	Chest,Community Chest,-1,-1,-1,-1,Chest,false
// 	Purple,Baltic Avenue Rd,60,50,4 20 60 180 320 450,30,Color,true
// 	IncomeTax,Income Tax,-1,-1,4 10,-1,IncomeTax,false
// 	Railroad,Reading Railroad,200,10,25 50 100 200,100,Railroad,true
// 	LightBlue,Oriental Avenue,100,50,6 30 90 270 400 550,50,Color,true
// 	Chance,Chance,-1,-1,-1,-1,Chance,false
// 	LightBlue,Vermont Avenue,100,50,6 30 90 270 400 550,50,Color,true
// 	LightBlue,Connecticut Avenue,120,50,8 40 100 300 450 600,60,Color,true
// 	Corner,Jail,-1,-1,-1,-1,Corner,false
// 	Pink,St. Charles Place,140,100,10 50 150 450 625 750,70,Color,true
// 	Utility,Electric Company,150,75,4 10,-1,Utility,true
// 	Pink,States Avenue,140,100,10 50 150 450 625 750,70,Color,true
// 	Pink,Virginia Avenue,160,100,12 60 180 500 700 90,80,Color,true
// 	Railroad,Pennsylvania Railroad,10,200,25 50 100 200,100,Railroad,true
// 	Orange,St. James Place,180,100,14 70 200 550 750 950,90,Color,true
// 	Chest,Community Chest,-1,-1,-1,-1,Chest,false
// 	Orange,Tennessee Avenue,180,100,14 70 200 550 750 950,90,Color,true
// 	Orange,New York Avenue,200,100,16 80 220 600 800 1000,100,Color,true
// 	Corner,Free Parking,-1,-1,-1,-1,Corner,false
// 	Red,Kentucky Avenue,220,150,18 90 250 700 875 1050,110,Color,true
// 	Chance,Chance,-1,-1,-1,-1,Chance,true
// 	Red,Indiana Avenue,220,150,18 90 250 700 875 1050,110,Color,true
// 	Railroad,B. & O. Railroad,10,200,25 50 100 200,100,RailRoad,true
// 	Red,Illinois Avenue,240,150,20 100 300 750 925 1100,120,Color,true
// 	Yellow,Atlantic Avenue,260,150,22 110 330 800 975 1150,130,Color,true
// 	Yellow,Ventnor Avenue,260,150,22 110 330 800 975 1150,130,Color,true
// 	Utility,Water Works,150,75,4 10,-1,Utility,true
// 	Yellow,Marvin Gardens,280,150,24 120 360 850 1025 1200,140,Color,true
// 	Corner,Go To Jail,-1,-1,-1,-1,Corner,false
// 	Green,Pacific Avenue,300,200,26 130 390 900 1100 1275,150,Color,true
// 	Green,North Carolina Avenue,300,200,26 130 390 900 1100 1275,150,Color,true
// 	Chest,Community Chest,-1,-1,-1,-1,Chest,false
// 	Green,Pennsylvania Avenue,320,200,28 150 450 1000 1200 1400,160,Color,true
// 	Railroad,Short Line,10,200,25 50 100 200,100,RailRoad,true
// 	Chance,Chance,-1,-1,-1,-1,Chance,false
// 	Blue,Park Place,350,200,35 175 500 1100 1300 1500,175,Color,true
// 	LuxuryTax,Luxury Tax,75,-1,-1,-1,LuxuryTax,false
// 	Blue,Boardwalk,400,200,50 200 600 1400 1700 2000,200,Color,true",
// 			"Bank pays you dividend of $50,8,50
// 	Doctor's fee—Pay $50,8,-50
// 	From sale of stock you get $50,8,50
// 	Pay hospital fees of $100,8,-100
// 	Holiday Fund matures—Receive $100,8,100
// 	It is your birthday—Collect $10,8,10
// 	Life insurance matures–Collect $100,8,100
// 	Pay school fees of $150,8,-150
// 	Receive $25 consultancy fee,8,25
// 	Income tax refund–Collect $20,8,20
// 	You have won second prize in a beauty contest–Collect $10,8,10
// 	You inherit $100,8,100
// 	Advance to Go (Collect $200),10,0,200,
// 	You are assessed for street repairs–$40 per house–$115 per hotel,5,40,115
// 	Get Out of Jail Free,1,
// 	Grand Opera Night—Collect $50 from every player for opening night seats,3,50
// 	Go to Jail–Go directly to Jail–Do not pass Go. do not collect $200,3,10,",
// 			"Make general repairs on all your property–For each house pay $25–For each hotel $100,5,25 100,
// 	Bank pays you dividend of $50,8,50
// 	Pay poor tax of $15,8,15
// 	Your building and loan matures—Collect $150,8,150
// 	You have won a crossword competition—Collect $100,8,100
// 	Advance to Illinois Ave—If you pass Go collect $200,10,24
// 	Advance to St. Charles Place – If you pass Go collect $200,10,11
// 	Take a trip to Reading Railroad–If you pass Go collect $200,10,5
// 	Take a walk on the Boardwalk–Advance token to Boardwalk,10,39
// 	Advance to Go (Collect $200),10,0 200
// 	Get Out of Jail Free,1,
// 	Go to Jail–Go directly to Jail–Do not pass Go. do not collect $200,3,10,
// 	Advance token to nearest Utility. If unowned you may buy it from the Bank. If owned throw dice and pay owner a total ten times the amount thrown.,12,10,
// 	You have been elected Chairman of the Board–Pay each player $50,7,50,
// 	Go Back 3 Spaces,14,3,
// 	Advance token to the nearest Railroad and pay owner twice the rental to which he/she {he} is otherwise entitled. If Railroad is unowned you may buy it from the Bank.,13,2,"];
// 		let paths = ["board_spaces.csv", "chest_cards.csv", "chance_cards.csv"];
// 		for i in 0..paths.len()
// 		{
// 			let mut f = File::create(paths[i]).unwrap();
// 			write!(f, "{}", all_lines[i]).unwrap();
// 		}
// 	}
// 	
// 	pub fn tripleDoublesCheck(i: usize) -> bool {
// 		if i == TOO_MANY_DOUBLES {
// 			println!("3 doubles in a row, GO TO JAIL!");
// 		}
// 		i == TOO_MANY_DOUBLES
// 	}
// }
