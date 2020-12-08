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

extern crate csv;


macro_rules! printc {
	($($a_string:expr), *) => {{
		let mut message = String::new();
		$( message = format!("{}{}", message, $a_string); )*
		match message.chars().last().unwrap() {
            '\n' => print!("{}",message),
            _ => println!("{}",message),
        };
	}};
}
macro_rules! join {
    ($($a_string:expr), *) => {{
        let mut message = String::new();
        $( message = format!("{}{}", message, $a_string); )*
        message.to_owned().as_str()
    }};
}

#[derive(Debug, Clone)]
enum Card {
    Pay { text: String, amount: isize },
    // pay/collect $x
    PayEach { text: String, amount: isize },
    // pay /collect from each player $x
    MoveX { text: String, spaces: isize },
    // move back 3 spaces
    Advance { text: String, to: isize, collect: Option<isize> },
    // location
    NextRailRoad { text: String, multiplier: isize },
    NextUtility { text: String, multiplier: isize },
    Repairs { text: String, house: isize, hotel: isize },
    // amount per house/hotel
    JailFree { text: String, kind: CardKind },
    // get out of jail free card, which deck it came from.
    Jail { text: String, to: isize }, // location of jail
}

#[derive(Debug, Copy, Clone)]
enum CardKind {
    Chance,
    Chest,
}

#[derive(Debug)]
enum CardSpace {
    Chance { text: String },
    Chest { deck: Deck },
}

#[derive(Debug, Clone)]
enum Color {
    Purple,
    LightBlue,
    Pink,
    Orange,
    Red,
    Yellow,
    Green,
    Blue,
}

#[derive(Debug, Clone)]
struct ColorDeed {
    space: isize,
    deed: Deed,
    color: Color,
    houses: isize,
}

#[derive(Debug)]
enum Corner {
    Neutral { location: isize },
    // free parking/just visiting
    Special { money: isize },
    //  200 for passing go
    Teleport { to: isize }, // go to jail
}

#[derive(Debug, Clone)]
struct Player {
    name: String,
    location: isize,
    money: isize,
    deeds: Vec<ColorDeed>,
    in_jail: isize,
    index: usize,
}

impl Deck {
    pub fn new() -> Self {
        Self { used: vec![], unused: vec![] }
    }
    fn load_cards(&mut self, file_name: &str, is_chance: bool) {
        use CardKind::*;
        use Card::*;
        let mut pile = vec![];
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);
        let kind: CardKind = match is_chance {
            true => Chance,
            false => Chest,
        };
        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            let mut params: Vec<&str> = line.split(",").collect();
            let mut text = params[0].to_string();
            let mut card: Card = match params[1] {
                "10" => {
                    let mut collect: Option<isize>;
                    if params.len() > 3 {
                        collect = Option::from(parse_i(params[3]));
                    } else { collect = None }
                    Advance { text, to: parse_i(params[2]), collect }
                }
                "1" => JailFree { text, kind },
                "8" => Pay { text, amount: parse_i(params[2]) },
                "14" => MoveX { text, spaces: parse_i(params[2]) },
                "3" => Jail { text, to: parse_i(params[2]) },
                "12" => NextUtility { text, multiplier: parse_i(params[2]) },
                "7" => PayEach { text, amount: parse_i(params[2]) },
                "13" => NextRailRoad { text, multiplier: parse_i(params[2]) },
                /*"5"*/ _ => Repairs {
                    text,
                    house: parse_i(params[2]),
                    hotel: parse_i(params[2]),
                },
            };
            pile.push(card);
        }
        self.unused = pile;
    }
}

#[derive(Debug, Clone)]
struct Deed {
    is_mortgaged: bool,
    mortgage_price: isize,
    purchase_price: isize,
    owner: Option<usize>,
    rents: Vec<isize>,
    sibling_count: isize,
    name: String,
    num_houses: usize,
}

impl Deed {
    fn get_stats(&self) -> (bool, isize, isize, Option<usize>, usize) {
        (
            self.is_mortgaged,
            self.mortgage_price,
            self.purchase_price,
            self.owner,
            self.num_houses,
        )
    }
    pub fn new(
        mortgage_price: isize,
        purchase_price: isize,
        rents: Vec<isize>,
        sibling_count: isize,
        name: String,
    ) -> Self {
        Self {
            is_mortgaged: false,
            mortgage_price,
            purchase_price,
            owner: None,
            rents,
            sibling_count,
            name,
            num_houses: 0,
        }
    }
    pub fn get_payout(&self) -> isize {
        *self.rents.get(0).unwrap()
    }
}

#[derive(Debug)]
struct BoardSpace {
    name: String,
    subtype: SpaceKind,
    siblings: Vec<usize>,
}

impl BoardSpace {
    pub fn new(name: String, subtype: SpaceKind) -> Self {
        Self { name, subtype, siblings: vec![] }
    }
    fn get_kind(&mut self) -> &mut SpaceKind {
        self.subtype.borrow_mut()
    }
    fn to_string(&self) -> String {
        format!("{}", self.name)
    }
}

#[derive(Debug)]
struct Deck {
    used: Vec<Card>,
    unused: Vec<Card>,
}

struct GameState {
    board: Vec<BoardSpace>,
    deed_map: HashMap<usize, Option<usize>>,
    accounts: HashMap<usize, isize>,
    player_map: HashMap<usize, String>,
    current_player: usize,
    location_map: HashMap<usize, usize>,
}

fn pause()  {
    stdin().read_line(&mut String::new());
    std::io::stdout().flush();
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
        let mut chance = Deck::new();
        let mut chest = Deck::new();
        let mut board: Vec<BoardSpace> = vec![];
        use SpaceKind::*;
        let filename = BOARD_PATH;
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            let mut params: Vec<&str> = line.split(",").collect();
            let mut space_name = params[1].to_string();
            let mut location = i as isize;
            let mut board_space = match params[0] {
                "Corner" => {
                    let mut corner = match params[1] {
                        "Free Parking" | "Jail" =>
                            Corner::Neutral { location },
                        "Go To Jail" =>
                            Corner::Teleport { to: parse_i(params[2]) as isize },
                        _ =>
                            Corner::Special { money: 200 as isize },
                    };
                    BoardSpace::new(space_name, SpaceKind::CornerSpace(corner))
                }
                "LuxuryTax" => {
                    let mut lux = TaxSpace(Tax::Luxury { flat_rate: parse_i(params[2]) });
                    BoardSpace::new(space_name, lux)
                }
                "IncomeTax" => {
                    let mut tax = TaxSpace(Tax::Income { percent: f_parse(params[2]), flat_rate: parse_i(params[3]) });
                    BoardSpace::new(space_name, tax)
                }
                "Chance" => BoardSpace::new(space_name, CardSpace(CardKind::Chance)),
                "Chest" => BoardSpace::new(space_name, CardSpace(CardKind::Chest)),
                _ => {
                    deed_map.insert(i, None);
                    let mortgage_price = parse_i(params[5]);
                    let purchase_price = parse_i(params[2]);
                    let rents = parse_i_array(params[4]);
                    let mut deed = Deed::new(mortgage_price, purchase_price, rents, 0, space_name.clone());
                    let mut space_kind = match params[0] {
                        "Utility" => UtilSpace(Utility { deed }),
                        "RailRoad" => RailSpace(Railroad { deed }),
                        _ => {
                            let k = SpaceKind::to_kind(params[0]);
                            ColorSpace(ColorDeed { space: i as isize, deed, color: k, houses: 0 })
                        }
                    };
                    BoardSpace::new(space_name, space_kind)
                }
            };
            board.push(board_space);
        }
        chance.load_cards(CHANCE_STR, true);
        chest.load_cards(CHEST_PATH, false);
        let mut current_player = 0;
        Self {
            board,
            deed_map,
            accounts,
            player_map,
            current_player,
            location_map,
        }
    }
    fn roll(&self) -> [usize; 2] {
        let mut rng = thread_rng();
        let roll_1 = rng.gen_range(1, 6);
        let roll_2 = rng.gen_range(1, 6);
        [roll_1, roll_2]
    }
    fn play(&mut self) {
        while self.player_map.len() > 1 {
            let mut kicked_out = self.take_turn();
            if !kicked_out {
                self.current_player += 1;
            } 
            self.current_player = self.current_player % self.player_map.len();
        }
    }
    fn take_turn(&mut self) -> bool {
        use SpaceKind::*;
        for i in 0..TOO_MANY_DOUBLES + 1 {
            if i == TOO_MANY_DOUBLES {
                println!("3 doubles in a row, GO TO JAIL!");
                break;
            }
            let player_name = self.player_map.get(&self.current_player).unwrap();
            println!("It's {}'s turn", player_name);
            let roll = self.roll();
            let name = self.player_map.get(&self.current_player).unwrap();
            let old_player_location = self.location_map.get(&self.current_player).unwrap().clone();
            let current_player_location = (old_player_location + roll[0] + roll[1]) % self.board.len();
            self.location_map.insert(self.current_player.clone(), current_player_location.clone());
            let mut space_name = self.board[current_player_location].name.clone();
            printc!(name.clone()," rolled ", roll[1]," + ", roll[0], " and is now at ", space_name,"\n");
                            let mut current_player_balance = *self.accounts.get(&self
                                .current_player).unwrap().borrow_mut();
            match self.board[current_player_location].get_kind() {
                CardSpace(card_kind) => {}
                ColorSpace(color_deed) => {
                    let current_player_name = self.player_map[&self.current_player].clone();
                    match &mut color_deed.deed.owner {
                        Some(owner) => {
                            let rent = self.calculate_rent(board[current_player_location]);
                            if current_player_balance - rent < 0 {
                                printc!(current_player_name, " can't pay up,\
                                 and has been kicked out of the game!");
                                self.player_map.remove(&self.current_player);
                                return true;
                            }
                            self.accounts.insert(self.current_player,current_player_balance - rent);
                            self.accounts.insert(owner.clone(),self.accounts.get(owner).unwrap() + rent);
                        },
                        None => {
                            let purchase_price = color_deed.deed.purchase_price;
                            
                            if purchase_price <= *current_player_balance
                            && bool_in(
                                join!("Does ",current_player_name,
                                        " want to buy ", space_name, 
                                        "?\nPrice: ", purchase_price,
                                        "\nbalance: ", current_player_balance)) {
                                color_deed.deed.owner = Option::from(self.current_player);
                                self.accounts.insert(
                                    self.current_player,
                                    current_player_balance - purchase_price);
                                printc!(current_player_name, " has purchased ", space_name,
                                "\nRemaining balance: $", self.accounts.get(&self.current_player).unwrap());
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
        /*match &space.subtype {
        
    }
        _kind::
        ColorSpace(ColorDeed { space: _, deed, color: _color, houses: _, }) => {
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
    }*/
    }
    /*fn auction<'a>(&mut self, bidders: &mut Vec<(&str, isize)>, deed: &Deed) -> Option<(String,
                                                                                        isize)> {
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

impl SpaceKind {
    pub fn to_kind(s: &str) -> Color {
        use Color::*;
        match s {
            "Purple" => Purple,
            "LightBlue" => LightBlue,
            "Pink" => Pink,
            "Orange" => Orange,
            "Red" => Red,
            "Yellow" => Yellow,
            "Green" => Green,
            _ => Blue,
        }
    }
}

#[derive(Debug, Clone)]
struct Railroad {
    deed: Deed,
}

#[derive(Debug)]
enum SpaceKind {
    CardSpace(CardKind),
    ColorSpace(ColorDeed),
    CornerSpace(Corner),
    TaxSpace(Tax),
    UtilSpace(Utility),
    RailSpace(Railroad),
}

#[derive(Debug, Clone)]
struct Utility {
    deed: Deed,
}

#[derive(Debug)]
enum Tax {
    Income { percent: f32, flat_rate: isize },
    Luxury { flat_rate: isize },
}

impl Player {
    pub fn setMoney(&mut self, amount: isize) {
        self.money += amount;
    }
}

impl fmt::Display for BoardSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Deed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

const BOARD_SIZE: i32 = 40;
const BOARD_PATH: &'static str = "board_spaces.csv";
const CHANCE_PATH: &'static str = "chance_cards.csv";
const CHEST_PATH: &'static str = "chest_cards.csv";
const MAX_PLAYERS: usize = 3;
const TOO_MANY_DOUBLES: usize = 2;
const JAIL: isize = 10;
const SPACES_STR: &str = "board_spaces.csv";
const CHEST_STR: &str = "chest_cards.csv";
const CHANCE_STR: &str = "chance_cards.csv";

fn main() {
    make_files();
    let mut state = GameState::new();
    state.play();
}

fn str_in(message: &str) -> String {
    let mut s = String::new();
    loop {
        println!("{}", message);
        stdin().read_line(&mut s);
        match s.trim().len() {
            0 => {
                println!("Empty Strings not allowed. Try again.");
                continue;
            }
            _s => return String::from(s.trim())
        };
    }
}

fn int_in() -> isize {
    loop {
        let mut s = String::new();
        stdin().read_line(&mut s);
        match s.trim().parse::<isize>() {
            Ok(num) => { return num; }
            Err(e) => {
                print!("not a valid input. Try again. {:?}", e);
            }
        };
    }
}

fn bool_in(message: &str) -> bool {
    let mut i: isize;
    loop {
        printc!(message,"\n1 for yes, 0 for no" );
        i = int_in();
        match i {
            0 | 1 => return i == 1,
            _ => {
                printc!("'", i, "' is not a valid choice.");
                continue;
            }
        }
    }
}

fn f_parse(s: &str) -> f32 {
    s.split_whitespace()
        .map(|s| s.parse::<f32>())
        .collect::<Result<Vec<_>, _>>().unwrap()[0]
}

fn parse_i(s: &str) -> isize {
    parse_i_array(s)[0]
}

fn parse_i_array(s: &str) -> Vec<isize> {
    parse_i_array_wrap(s).unwrap()
}

fn parse_i_array_wrap(s: &str) -> Option<Vec<isize>> {
    s.split(" ")
        .map(|s| { /*println!("'{}'", s);*/ s.parse::<isize>().ok() })
        .collect()
}

fn make_files() {
    let all_lines = [
        "Corner,GO,-1,-1,-1,-1,Corner,false
Purple,Mediterranean Avenue,60,50,2 10 30 90 160 250,30,Color,true
Chest,Community Chest,-1,-1,-1,-1,Chest,false
Purple,Baltic Avenue Rd,60,50,4 20 60 180 320 450,30,Color,true
IncomeTax,Income Tax,-1,-1,4 10,-1,IncomeTax,false
Railroad,Reading Railroad,200,10,25 50 100 200,100,Railroad,true
LightBlue,Oriental Avenue,100,50,6 30 90 270 400 550,50,Color,true
Chance,Chance,-1,-1,-1,-1,Chance,false
LightBlue,Vermont Avenue,100,50,6 30 90 270 400 550,50,Color,true
LightBlue,Connecticut Avenue,120,50,8 40 100 300 450 600,60,Color,true
Corner,Jail,-1,-1,-1,-1,Corner,false
Pink,St. Charles Place,140,100,10 50 150 450 625 750,70,Color,true
Utility,Electric Company,150,75,4 10,-1,Utility,true
Pink,States Avenue,140,100,10 50 150 450 625 750,70,Color,true
Pink,Virginia Avenue,160,100,12 60 180 500 700 90,80,Color,true
Railroad,Pennsylvania Railroad,10,200,25 50 100 200,100,Railroad,true
Orange,St. James Place,180,100,14 70 200 550 750 950,90,Color,true
Chest,Community Chest,-1,-1,-1,-1,Chest,false
Orange,Tennessee Avenue,180,100,14 70 200 550 750 950,90,Color,true
Orange,New York Avenue,200,100,16 80 220 600 800 1000,100,Color,true
Corner,Free Parking,-1,-1,-1,-1,Corner,false
Red,Kentucky Avenue,220,150,18 90 250 700 875 1050,110,Color,true
Chance,Chance,-1,-1,-1,-1,Chance,true
Red,Indiana Avenue,220,150,18 90 250 700 875 1050,110,Color,true
Railroad,B. & O. Railroad,10,200,25 50 100 200,100,RailRoad,true
Red,Illinois Avenue,240,150,20 100 300 750 925 1100,120,Color,true
Yellow,Atlantic Avenue,260,150,22 110 330 800 975 1150,130,Color,true
Yellow,Ventnor Avenue,260,150,22 110 330 800 975 1150,130,Color,true
Utility,Water Works,150,75,4 10,-1,Utility,true
Yellow,Marvin Gardens,280,150,24 120 360 850 1025 1200,140,Color,true
Corner,Go To Jail,-1,-1,-1,-1,Corner,false
Green,Pacific Avenue,300,200,26 130 390 900 1100 1275,150,Color,true
Green,North Carolina Avenue,300,200,26 130 390 900 1100 1275,150,Color,true
Chest,Community Chest,-1,-1,-1,-1,Chest,false
Green,Pennsylvania Avenue,320,200,28 150 450 1000 1200 1400,160,Color,true
Railroad,Short Line,10,200,25 50 100 200,100,RailRoad,true
Chance,Chance,-1,-1,-1,-1,Chance,false
Blue,Park Place,350,200,35 175 500 1100 1300 1500,175,Color,true
LuxuryTax,Luxury Tax,75,-1,-1,-1,LuxuryTax,false
Blue,Boardwalk,400,200,50 200 600 1400 1700 2000,200,Color,true",
        "Bank pays you dividend of $50,8,50
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
Go to Jail–Go directly to Jail–Do not pass Go. do not collect $200,3,10,",
        "Make general repairs on all your property–For each house pay $25–For each hotel $100,5,25 100,
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
Advance token to the nearest Railroad and pay owner twice the rental to which he/she {he} is otherwise entitled. If Railroad is unowned you may buy it from the Bank.,13,2,"];
    let paths = ["board_spaces.csv", "chest_cards.csv", "chance_cards.csv"];
    for i in 0..paths.len()
    {
        let mut f = File::create(paths[i]).unwrap();
        write!(f, "{}", all_lines[i]).unwrap();
    }
}
