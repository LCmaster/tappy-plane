pub enum Origin {
    UpperLeft,
    Middle
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Dimension {
    pub width: i32,
    pub height: i32,
}

pub struct Rectangle {
    pub origin: Origin,
    pub size: Dimension,
    pub position: Position,
}

pub struct Circle {
    pub radius: u16,
    pub origin: Origin,
    pub position: Position,
}
