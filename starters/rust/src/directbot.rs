use serde::Deserialize;
// use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub directions: Vec<Direction>,
}

#[derive(Debug, Deserialize)]
pub struct Direction {
    pub coordinates: Option<LinePos>,
    pub direction: String,
    pub distance: u8,
    pub speed: u8,
}
#[derive(Debug, Deserialize)]
pub struct LinePos {
    pub a: Pos,
    pub b: Pos,
}

#[derive(Debug, Deserialize)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}
