use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize)]
pub struct Dimension {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Deserialize)]
pub struct Rectangle {
    pub size: Dimension,
    pub position: Position,
}

#[derive(Debug, Deserialize)]
pub struct Circle {
    pub radius: u16,
    pub position: Position,
}
