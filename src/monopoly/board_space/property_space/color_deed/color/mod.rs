use super::{place::Place, estate::Estate};

#[derive(Debug, Clone)]
pub enum Color {
	Purple(Place, Estate),
	LightBlue(Place, Estate),
	Pink(Place, Estate),
	Orange(Place, Estate),
	Red(Place, Estate),
	Yellow(Place, Estate),
	Green(Place, Estate),
	Blue(Place, Estate),
}
