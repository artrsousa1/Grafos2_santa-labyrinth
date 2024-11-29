use actix_web::{
    get,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};

use core::time;
use serde::{Deserialize, Serialize};
use std::{env, thread};

#[derive(Serialize, Deserialize, Debug)]
struct CellPiece {
    u: bool,
    l: bool,
    d: bool,
    r: bool,
    can_rotate: bool,
}

type CellGrid = Vec<Vec<CellPiece>>;

#[derive(Serialize, Deserialize, Debug)]
struct GameSchema {
    pub number_of_rows: i32,
    pub number_of_columns: i32,

    pub initial_x: i32,
    pub initial_y: i32,

    pub target_x: i32,
    pub target_y: i32,

    pub grid: CellGrid,
}

#[get("/solver")]
async fn solver_endpoint(data: web::Json<GameSchema>) -> impl Responder {
    let sec_to_sleep = rand::random::<u64>() % 10;
    thread::sleep(time::Duration::from_secs(sec_to_sleep));

    let solved = rand::random::<u64>() % 7;
    if solved == 0 {
        return HttpResponse::Ok().json(data);
    } else {
        return HttpResponse::BadRequest().into();
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let backend_api_port: u16 = match env::var("BACKEND_API_PORT") {
        Ok(val) => val.parse::<u16>().unwrap(),
        Err(_e) => 8086,
    };

    HttpServer::new(|| App::new().service(solver_endpoint))
        .bind(("0.0.0.0", backend_api_port))?
        .run()
        .await
}
