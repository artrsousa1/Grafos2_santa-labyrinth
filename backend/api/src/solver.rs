use serde::{Deserialize, Serialize};

use crate::cell_piece::can_connect;

use crate::game::{CellGrid, GameSchema};

use crate::coordinate::{calc_manhattan_negative_distance, Coordinate, DELTA_X, DELTA_Y};

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

// State to be added at the A* min_heap
// TODO: Don't use the the negative of distance and actually implement a Ord properly
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
struct State {
    manhattan_negative_distance: i32,
    pos: Coordinate,
    grid: CellGrid,
}

// A* solution
pub async fn solve(
    source: &Coordinate,
    goal: &Coordinate,
    initial_grid: &CellGrid,
) -> Option<GameSchema> {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut vis: HashSet<State> = HashSet::new();

    let initial_state = State {
        manhattan_negative_distance: calc_manhattan_negative_distance(&source, &goal),
        pos: source.clone(),
        grid: initial_grid.clone(),
    };

    heap.push(initial_state.clone());
    vis.insert(initial_state.clone());

    while let Some(State {
        manhattan_negative_distance: u_dist,
        pos: u_pos,
        grid: mut u_grid,
    }) = heap.pop()
    {
        print_grid(&u_grid);
        // Found solution.
        if u_dist == 0 {
            return Some(GameSchema {
                // TODO: shorthand?
                source: source.clone(),
                goal: goal.clone(),
                grid: u_grid,
            });
        }

        for d in 0..5 {
            let mut v_pos = u_pos.clone();
            v_pos.x += DELTA_X[d];
            v_pos.y += DELTA_Y[d];
            if v_pos.x >= 0
                && v_pos.x < initial_grid.len() as i32
                && v_pos.y >= 0
                && v_pos.y < initial_grid[v_pos.x as usize].len() as i32
            {
                for _r in 0..4 {
                    let connectable = can_connect(
                        &u_pos,
                        &u_grid[u_pos.x as usize][u_pos.y as usize],
                        &v_pos,
                        &u_grid[v_pos.x as usize][v_pos.y as usize],
                    );
                    if connectable {
                        let v = State {
                            manhattan_negative_distance: calc_manhattan_negative_distance(
                                &v_pos, &goal,
                            ),
                            pos: v_pos,
                            grid: u_grid.clone(),
                        };

                        // TODO: don't need to check the whole map, just if
                        // is still connect to it's parent...
                        if is_solved(&source, &v_pos, &v.grid).solvable && !vis.contains(&v) {
                            vis.insert(v.clone());
                            heap.push(v.clone());
                        }
                    }

                    u_grid[v_pos.x as usize][v_pos.y as usize].rotate();
                }
            }
        }
    }

    None
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Solution {
    pub solvable: bool,
    pub coordinates: Vec<Coordinate>, // TODO: probably better as a result
}

pub fn is_solved(source: &Coordinate, goal: &Coordinate, grid: &CellGrid) -> Solution {
    let n = grid.len();
    let m = grid[0].len();
    let mut vis = vec![vec![false; m]; n];
    let mut parent_coordinate: HashMap<Coordinate, Coordinate> = HashMap::new();

    let mut queue = VecDeque::from([source.clone()]);

    vis[source.x as usize][source.y as usize] = true;

    while let Some(u) = queue.pop_front() {
        if u == *goal {
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
                coordinates: coordinates, // TODO: shorthand ?
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
                    &u,
                    &grid[u.x as usize][u.y as usize],
                    &v,
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

// atestado de incompetência:
pub fn print_grid(grid: &CellGrid) {
    println!();
    for i in 0..grid.len() as usize {
        for j in 0..grid[i].len() as usize {
            if (i == 0 && j == 0) || (i == grid.len() - 1 && j == grid[0].len() - 1) {
                print!("*");
            } else if grid[i][j].l && grid[i][j].r {
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
        //println!();
    }
}
