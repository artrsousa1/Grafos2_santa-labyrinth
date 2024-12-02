mod game;
use game::GameSchema;
use solver::Coordinate;

mod solver;
use solver::is_solved;

use actix_web::{
    get,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};

use std::env;

#[get("/solver")]
async fn solver_endpoint(data: web::Json<GameSchema>) -> impl Responder {
    return match solver::solver(data.0) {
        Some(ret) => HttpResponse::Ok().json(ret),
        _ => HttpResponse::BadRequest().into(),
    };
}

#[get("/is_solved")]
async fn is_solvable_endpoint(game: web::Json<GameSchema>) -> impl Responder {
    solver::print_grid(&game.0.grid);
    return HttpResponse::Ok().json(is_solved(
        &game.0.grid,
        Coordinate {
            x: game.initial_x,
            y: game.initial_y,
        },
        Coordinate {
            x: game.0.target_x,
            y: game.0.target_y,
        },
    ));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let backend_api_port: u16 = match env::var("BACKEND_API_PORT") {
        Ok(val) => val.parse::<u16>().unwrap(),
        Err(_e) => 8086,
    };

    HttpServer::new(|| {
        App::new()
            .service(solver_endpoint)
            .service(is_solvable_endpoint)
    })
    .bind(("0.0.0.0", backend_api_port))?
    .run()
    .await
}
