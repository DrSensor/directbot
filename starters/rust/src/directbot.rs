use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Response {
	directions: Vec<Direction>
}

#[derive(Debug, Deserialize)]
pub struct Direction {
	coordinates: HashMap<String, Coordinate>,
	direction: String,
	distance: usize,
	speed: usize,
}

#[derive(Debug, Deserialize)]
pub struct Coordinate {
	x: isize,
	y: isize,
}