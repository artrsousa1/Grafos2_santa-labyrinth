use crate::cell_piece::CellPiece;
use crate::coordinate::Coordinate;
use serde::{Deserialize, Serialize};

pub type CellGrid = Vec<Vec<CellPiece>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSchema {
    pub source: Coordinate,

    pub goal: Coordinate,

    pub grid: CellGrid,
}
