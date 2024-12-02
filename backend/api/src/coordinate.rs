use serde::{Deserialize, Serialize};

// 5 directions, center, up, right, down, left (clockwise)
pub const DELTA_X: [i32; 5] = [0, -1, 0, 1, 0];
pub const DELTA_Y: [i32; 5] = [0, 0, 1, 0, -1];

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Copy, Ord, PartialOrd)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

pub fn calc_manhattan_negative_distance(u: &Coordinate, v: &Coordinate) -> i32 {
    -((u.x - v.x).abs() + (u.y - v.y).abs())
}
