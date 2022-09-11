mod action;

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    location: isize,
    money: isize,
    deeds: Vec<ColorDeed>,
    in_jail: isize,
    index: usize,
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
