use crate::game::can_connect;
use crate::game::CellGrid;
use crate::game::GameSchema;
use std::collections::BinaryHeap;
use std::collections::HashSet;

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
        //manhattan_negative_distance: -((game.initial_x - game.target_x).abs()
        //    + (game.initial_y - game.target_y).abs()),
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

pub fn solver(mut game: GameSchema) -> Option<GameSchema> {
    game.initial_y -= 1;
    game.initial_x -= 1;
    game.target_y -= 1;
    game.target_x -= 1;

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
            return Some(GameSchema {
                initial_x: game.initial_x + 1,
                initial_y: game.initial_y + 1,
                target_x: game.target_x + 1,
                target_y: game.target_y + 1,
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
                    if can_connect(
                        u_x,
                        u_y,
                        &u_grid[u_x as usize][u_y as usize],
                        v_x,
                        v_y,
                        &u_grid[v_x as usize][v_y as usize],
                    ) {
                        let v = State {
                            manhattan_negative_distance: calc_manhattan_negative_distance(
                                v_x, v_y, target_x, target_y,
                            ),
                            current_x: v_x,
                            current_y: v_y,
                            grid: u_grid.clone(),
                        };
                        if !vis.contains(&v) {
                            vis.insert(v.clone());
                            heap.push(v);
                        }
                    }
                    u_grid[v_x as usize][v_y as usize].rotate()
                }
            }
        }
    }

    None
}
