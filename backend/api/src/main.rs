mod game;
use game::GameSchema;

mod solver;

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
