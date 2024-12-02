use crate::solver::Coordinate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, Copy, Clone, PartialEq, PartialOrd, Ord, Hash)]
pub struct CellPiece {
    pub u: bool,
    pub l: bool,
    pub d: bool,
    pub r: bool,
    pub can_rotate: bool,
}

impl CellPiece {
    pub fn rotate(&mut self) {
        let rotated_piece: CellPiece = CellPiece {
            u: self.l,
            r: self.u,
            d: self.r,
            l: self.d,
            can_rotate: self.can_rotate,
        };
        // TODO: Clone this properly !
        self.u = rotated_piece.u;
        self.r = rotated_piece.r;
        self.d = rotated_piece.d;
        self.l = rotated_piece.l;
    }
}

pub fn can_connect(u_pos: &Coordinate, u: &CellPiece, v_pos: &Coordinate, v: &CellPiece) -> bool {
    if u_pos == v_pos {
        return true;
    }

    let d_x: i32 = u_pos.x - v_pos.x;
    let d_y: i32 = u_pos.y - v_pos.y;

    if d_x != 0 && d_y != 0 {
        // diagonal
        return false;
    }

    if d_x != 0 {
        if d_x > 0 {
            // v above u
            return u.u && v.d;
        } else {
            // v below u
            return u.d && v.u;
        }
    } else {
        if d_y > 0 {
            // v at left of u
            return u.l && v.r;
        } else {
            return u.r && v.l;
        }
    }
}

pub type CellGrid = Vec<Vec<CellPiece>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSchema {
    pub source: Coordinate,

    pub goal: Coordinate,

    pub grid: CellGrid,
}
