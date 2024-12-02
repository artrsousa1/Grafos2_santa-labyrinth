use serde::{Deserialize, Serialize};

use crate::game::can_connect;
use crate::game::CellGrid;
use crate::game::GameSchema;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

// State to be added at the A* min_heap
// TODO: Don't use the the negative of distance and actually implement a Ord properly
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
struct State {
    manhattan_negative_distance: i32,
    current_x: i32,
    current_y: i32,
    grid: CellGrid,
}

fn calc_manhattan_negative_distance(u_x: i32, u_y: i32, v_x: i32, v_y: i32) -> i32 {
    -((u_x - v_x).abs() + (u_y - v_y).abs())
}

// TODO: this can be done with traits right ?
fn get_state(game: &GameSchema) -> State {
    State {
        manhattan_negative_distance: calc_manhattan_negative_distance(
            game.initial_x,
            game.initial_y,
            game.target_x,
            game.target_y,
        ),
        current_x: game.initial_x,
        current_y: game.initial_y,
        grid: game.grid.clone(),
    }
}

// 5 directions, center, up, right, down, left (clockwise)
const DELTA_X: [i32; 5] = [0, -1, 0, 1, 0];
const DELTA_Y: [i32; 5] = [0, 0, 1, 0, -1];

pub fn print_grid(grid: &CellGrid) {
    println!();
    for i in 0..grid.len() as usize {
        for j in 0..grid[i].len() as usize {
            if grid[i][j].l && grid[i][j].r {
                print!("═");
            } else if grid[i][j].u && grid[i][j].d {
                print!("║");
            } else if grid[i][j].l && grid[i][j].d {
                print!("╗");
            } else if grid[i][j].l && grid[i][j].u {
                print!("╝");
            } else if grid[i][j].u && grid[i][j].r {
                print!("╚");
            } else if grid[i][j].r && grid[i][j].d {
                print!("╔");
            }
        }
        println!();
        println!();
    }
}

pub async fn solver(game: &GameSchema) -> Option<GameSchema> {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut vis: HashSet<State> = HashSet::new();

    let initial_state = get_state(&game);

    heap.push(initial_state.clone());
    vis.insert(initial_state.clone());

    let target_x = game.target_x;
    let target_y = game.target_y;
    while let Some(State {
        manhattan_negative_distance: u_dist,
        current_x: u_x,
        current_y: u_y,
        grid: mut u_grid,
    }) = heap.pop()
    {
        // Found solution.
        if u_dist == 0 {
            print_grid(&u_grid);
            return Some(GameSchema {
                initial_x: game.initial_x,
                initial_y: game.initial_y,
                target_x: u_x,
                target_y: u_y,
                number_of_rows: game.number_of_rows,
                number_of_columns: game.number_of_columns,
                grid: u_grid,
            });
        }

        for d in 0..5 {
            let v_x = u_x + DELTA_X[d];
            let v_y = u_y + DELTA_Y[d];
            if v_x >= 0 && v_x < game.number_of_rows && v_y >= 0 && v_y < game.number_of_columns {
                for _r in 0..4 {
                    let connectable = can_connect(
                        u_x,
                        u_y,
                        &u_grid[u_x as usize][u_y as usize],
                        v_x,
                        v_y,
                        &u_grid[v_x as usize][v_y as usize],
                    );
                    if connectable {
                        let v = State {
                            manhattan_negative_distance: calc_manhattan_negative_distance(
                                v_x, v_y, target_x, target_y,
                            ),
                            current_x: v_x,
                            current_y: v_y,
                            grid: u_grid.clone(),
                        };

                        if is_solved(
                            &v.grid,
                            Coordinate {
                                x: game.initial_x,
                                y: game.initial_y,
                            },
                            Coordinate { x: v_x, y: v_y },
                        )
                        .solvable
                            && !vis.contains(&v)
                        {
                            vis.insert(v.clone());
                            heap.push(v.clone());
                        }
                    }

                    u_grid[v_x as usize][v_y as usize].rotate();
                }
            }
        }
    }

    None
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Solution {
    pub solvable: bool,
    pub coordinates: Vec<Coordinate>, // TODO: probably better as a result
}

// EEEEEEEEEEEEEEEEEEE trem feio da porra KKKKKKKKKKKKKKKKKKKKKKKK
pub fn is_solved(grid: &CellGrid, source: Coordinate, target: Coordinate) -> Solution {
    let n = grid.len();
    let m = grid[0].len();
    let mut vis = vec![vec![false; m]; n];
    let mut parent_coordinate: HashMap<Coordinate, Coordinate> = HashMap::new();

    let mut queue = VecDeque::from([Coordinate {
        x: source.x,
        y: source.y,
    }]);

    vis[source.x as usize][source.y as usize] = true;

    while let Some(u) = queue.pop_front() {
        if u.x == target.x && u.y == target.y {
            let mut coordinates: Vec<Coordinate> = vec![];
            let mut current = u;
            loop {
                coordinates.push(current.clone());
                if !parent_coordinate.contains_key(&current) {
                    break;
                }
                current = parent_coordinate[&current];
            }
            return Solution {
                solvable: true,
                coordinates: coordinates,
            };
        }

        for d in 1..5 {
            let v = Coordinate {
                x: u.x + DELTA_X[d],
                y: u.y + DELTA_Y[d],
            };

            if v.x >= 0 && v.y >= 0 && v.x < n as i32 && v.y < m as i32 {
                if vis[v.x as usize][v.y as usize] {
                    continue;
                }

                if !can_connect(
                    u.x,
                    u.y,
                    &grid[u.x as usize][u.y as usize],
                    v.x,
                    v.y,
                    &grid[v.x as usize][v.y as usize],
                ) {
                    continue;
                }

                parent_coordinate.insert(v, u);
                vis[v.x as usize][v.y as usize] = true;
                queue.push_back(v);
            }
        }
    }

    Solution {
        solvable: false,
        coordinates: vec![],
    }
}
