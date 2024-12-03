mod game;
use game::GameSchema;

mod solver;
use solver::{is_solved, solve};

use actix_cors::Cors;

use actix_web::{
    post,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};

use std::env;
use std::time::Duration;

use tokio::time::timeout;

mod cell_piece;

mod coordinate;

#[post("/solver")]
async fn solver_endpoint(data: web::Json<GameSchema>) -> impl Responder {
    // TODO: MAKE SURE IT'S A RECTANGLE MAP
    let result = timeout(
        Duration::from_secs(30),
        solve(&data.0.source, &data.0.goal, &data.0.grid),
    )
    .await;
    return match result {
        Ok(ret) => match ret {
            Some(ret) => HttpResponse::Ok().json(ret),
            _ => HttpResponse::BadRequest().into(),
        },
        Err(_) => HttpResponse::BadRequest().into(),
    };
}

#[post("/is_solved")]
async fn is_solved_endpoint(game: web::Json<GameSchema>) -> impl Responder {
    return HttpResponse::Ok().json(is_solved(&game.0.source, &game.0.goal, &game.0.grid));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Defines which port to run the API based in ENVIRONMENT variable
    // BACKEND_API_PORT, if no such variable is fond 8086 is used
    let backend_api_port: u16 = match env::var("BACKEND_API_PORT") {
        Ok(val) => val.parse::<u16>().unwrap(),
        Err(_e) => 8086,
    };

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(solver_endpoint)
            .service(is_solved_endpoint)
    })
    .bind(("0.0.0.0", backend_api_port))?
    .run()
    .await
}
