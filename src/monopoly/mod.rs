pub mod board_space;
pub mod game_state;
pub mod utils;

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
const CHANCE: bool = false;
const CHEST: bool = true;
