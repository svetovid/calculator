use axum::{
    Json, 
    routing::{get, post}, 
    Router
};
use calculator;
use serde::Deserialize;

#[derive(Deserialize)]
struct CalculatorPayload {
    formula:  String,
    x: f64
}

#[derive(Deserialize)]
struct CalculatorRangePayload {
    formula:  String,
    start: f64,
    end: f64,
    step: f64
}

async fn calculate_once(Json(payload): Json<CalculatorPayload>) {
    let result = calculator::calculate_once(payload.x, payload.formula.as_str());
    println!("{:?}", result);
}

async fn calculate_range(Json(payload): Json<CalculatorRangePayload>) {
    let range = calculator::float_range(payload.start, payload.end, payload.step).collect();
    let result = calculator::calculate(range, payload.formula.as_str());
    println!("{:?}", result);
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async {"Hello world!"}))
        .route("/calculate", post(calculate_once))
        .route("/calculate/range", post(calculate_range));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
