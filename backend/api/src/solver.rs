use crate::game::GameSchema;
use core::time;
use std::thread;

pub fn solver(game: GameSchema) -> Option<GameSchema> {
    // pretend it's processing something
    let sec_to_sleep = rand::random::<u64>() % 10;
    thread::sleep(time::Duration::from_secs(sec_to_sleep));

    let solved = rand::random::<u64>() % 7;
    if solved != 0 {
        return Some(game);
    } else {
        return None;
    }
}
