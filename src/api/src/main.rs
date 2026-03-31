use actix_web::{get, post, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Deserialize, Clone)]
struct ScenarioIn {
    name: String,
    traffic_delta_pct: i32,
}

#[derive(Serialize, Clone, Debug)]
struct ScenarioOut {
    id: usize,
    name: String,
    traffic_delta_pct: i32,
}

struct AppState {
    scenarios: Mutex<Vec<ScenarioOut>>,
    next_id: Mutex<usize>,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

#[get("/scenarios")]
async fn list_scenarios(data: web::Data<AppState>) -> impl Responder {
    let scenarios = data.scenarios.lock().unwrap();
    HttpResponse::Ok().json(scenarios.clone())
}

#[post("/scenarios")]
async fn create_scenario(
    payload: web::Json<ScenarioIn>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut next_id = data.next_id.lock().unwrap();
    let id = *next_id;
    *next_id += 1;

    let scenario = ScenarioOut {
        id,
        name: payload.name.clone(),
        traffic_delta_pct: payload.traffic_delta_pct,
    };

    let mut scenarios = data.scenarios.lock().unwrap();
    scenarios.push(scenario.clone());

    HttpResponse::Ok().json(scenario)
}

#[get("/scenarios/{id}")]
async fn get_scenario(
    path: web::Path<usize>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let scenarios = data.scenarios.lock().unwrap();

    match scenarios.iter().find(|s| s.id == id) {
        Some(scenario) => HttpResponse::Ok().json(scenario.clone()),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "Scenario not found"})),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let app_state = web::Data::new(AppState {
        scenarios: Mutex::new(vec![]),
        next_id: Mutex::new(1),
    });

    println!("Starting Urban Twin API on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .service(health)
            .service(list_scenarios)
            .service(create_scenario)
            .service(get_scenario)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
