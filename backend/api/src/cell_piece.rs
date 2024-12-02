use crate::coordinate::Coordinate;
use serde::{Deserialize, Serialize};
use std::mem::swap;

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
        /*
              U        L       L      L
             L R  ->  U R - > D R -> D U
              D        D       U      R
        */
        swap(&mut self.u, &mut self.l);
        swap(&mut self.l, &mut self.d);
        swap(&mut self.r, &mut self.d);
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
