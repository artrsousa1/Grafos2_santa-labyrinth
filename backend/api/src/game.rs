use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CellPiece {
    pub u: bool,
    pub l: bool,
    pub d: bool,
    pub r: bool,
    pub can_rotate: bool,
}

pub type CellGrid = Vec<Vec<CellPiece>>;
#[derive(Serialize, Deserialize, Debug)]
pub struct GameSchema {
    pub number_of_rows: i32,
    pub number_of_columns: i32,

    pub initial_x: i32,
    pub initial_y: i32,

    pub target_x: i32,
    pub target_y: i32,

    pub grid: CellGrid,
}
