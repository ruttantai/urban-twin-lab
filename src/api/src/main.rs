use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ScenarioIn {
    name: String,
    traffic_delta_pct: i32,
}

#[derive(Serialize)]
struct ScenarioOut {
    id: usize,
    name: String,
    traffic_delta_pct: i32,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[post("/scenarios")]
async fn create_scenario(payload: web::Json<ScenarioIn>) -> impl Responder {
    let out = ScenarioOut {
        id: 1,
        name: payload.name.clone(),
        traffic_delta_pct: payload.traffic_delta_pct,
    };
    HttpResponse::Ok().json(out)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health).service(create_scenario))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
