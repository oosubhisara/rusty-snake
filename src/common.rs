
//=============================================================================
//    Point
//=============================================================================
#[derive(Eq, PartialEq)]
pub struct Point {
    pub x: i32, 
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_u32(x: u32, y: u32) -> Self {
        Self { x: x as i32, y: y as i32 }
    }
}

//=============================================================================
//    Line
//=============================================================================
pub struct Line {
    pub x1: f32, 
    pub y1: f32,
    pub x2: f32, 
    pub y2: f32,
    pub alpha: f32,
}

impl Line {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32, alpha: f32) -> Line {
        Line  { x1, y1, x2, y2, alpha }
    }
}

//=============================================================================
//    Direction
//=============================================================================
pub struct Direction;

impl Direction {
    pub fn up() -> Point {
        Point { x: 0, y: -1 } 
    }

    pub fn down() -> Point { 
        Point { x: 0, y: 1 }
    }

    pub fn left() -> Point { 
        Point { x: -1, y: 0 } 
    }

    pub fn right() -> Point { 
        Point { x: 1, y: 0 } 
    }

    pub fn opposite(dir: &Point) -> Point {
        match dir {
            Point { x: 0, y: -1 } => Direction::down(),
            Point { x: 0, y: 1 } => Direction::up(), 
            Point { x: -1, y: 0 } => Direction::right(), 
            Point { x: 1, y: 0 } => Direction::left(), 
            _ => Point { x: 0, y: 0 },
        }
    }
} 
