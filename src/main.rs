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

mod monopoly;

fn main() {
    monopoly::game_logic::GameState::new().play();
}
